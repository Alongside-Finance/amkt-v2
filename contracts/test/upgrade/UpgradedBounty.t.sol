pragma solidity =0.8.18;

import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {Dealer} from "test/utils/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty, IInvokeableBounty} from "src/interfaces/IInvokeableBounty.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {MULTISIG} from "src/scripts/Config.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";
import {IActiveBounty} from "src/interfaces/IActiveBounty.sol";

contract UpgradedBountyTest is UpgradedTest {
    function _setUpTestBountyInvariantFuzz(
        uint256 numTokensToAdd,
        uint256 rand
    )
        internal
        returns (
            TokenInfo[] memory,
            TokenInfo[] memory,
            uint256[] memory,
            uint256[] memory,
            uint256[] memory,
            uint256[] memory
        )
    {
        numTokensToAdd = bound(numTokensToAdd, 0, 25);
        TokenInfo[] memory oldUnits = vault.virtualUnits();

        // generate a valid bounty
        Bounty memory bounty = generateValidBounty(numTokensToAdd, rand);
        TokenInfo[] memory proposedUnits = bounty.infos;
        bytes32 _hash = timelockInvokeableBounty.hashBounty(bounty);

        // set bounty by anyone except timelock should fail
        vm.startPrank(address(3));
        vm.expectRevert(IActiveBounty.ActiveBountyAuth.selector);
        timelockActiveBounty.setHash(_hash);
        vm.stopPrank();

        // set bounty by timelock should succeed
        vm.prank(address(timelockController));
        timelockActiveBounty.setHash(_hash);
        satisfyFulfillerBalances(address(this), bounty);

        // store balances for old and new tokens for vault and fulfiller (this contract) before fulfilling
        uint256[] memory oldUnitsVaultBalances = new uint256[](oldUnits.length);
        uint256[] memory oldUnitsFulfillerBalances = new uint256[](
            oldUnits.length
        );
        for (uint256 i = 0; i < oldUnits.length; i++) {
            oldUnitsVaultBalances[i] = IERC20(oldUnits[i].token).balanceOf(
                address(vault)
            );
            oldUnitsFulfillerBalances[i] = IERC20(oldUnits[i].token).balanceOf(
                address(this)
            );
        }
        uint256[] memory proposedUnitsVaultBalances = new uint256[](
            proposedUnits.length
        );
        uint256[] memory proposedUnitsFulfillerBalances = new uint256[](
            proposedUnits.length
        );
        for (uint256 i = 0; i < proposedUnits.length; i++) {
            proposedUnitsVaultBalances[i] = IERC20(proposedUnits[i].token)
                .balanceOf(address(vault));
            proposedUnitsFulfillerBalances[i] = IERC20(proposedUnits[i].token)
                .balanceOf(address(this));
        }

        // fulfill bounty by anyone except fulfiller should fail
        vm.startPrank(address(4));
        vm.expectRevert(IInvokeableBounty.BountyInvalidFulfiller.selector);
        timelockInvokeableBounty.fulfillBounty(bounty, false);
        vm.stopPrank();

        // fulfill bounty by fulfiller should succeed
        timelockInvokeableBounty.fulfillBounty(bounty, false);
        return (
            oldUnits,
            proposedUnits,
            oldUnitsVaultBalances,
            oldUnitsFulfillerBalances,
            proposedUnitsVaultBalances,
            proposedUnitsFulfillerBalances
        );
    }

    function testBountyInvariantsFuzz(
        uint256 numTokensToAdd,
        uint256 rand
    ) public {
        (
            TokenInfo[] memory oldUnits,
            TokenInfo[] memory proposedUnits,
            uint256[] memory oldUnitsVaultBalances,
            uint256[] memory oldUnitsFulfillerBalances,
            uint256[] memory proposedUnitsVaultBalances,
            uint256[] memory proposedUnitsFulfillerBalances
        ) = _setUpTestBountyInvariantFuzz(numTokensToAdd, rand);
        // INVARIANT 1: proposed units are the same as fulfilled units, minus tokens to be removed
        TokenInfo[] memory fulfilledUnits = vault.virtualUnits();
        uint256 skipCounter = 0;
        for (uint256 i = 0; i < proposedUnits.length; i++) {
            if (proposedUnits[i].units == 0) {
                skipCounter += 1;
                assertEq(vault.isUnderlying(proposedUnits[i].token), false);
            } else {
                assertEq(
                    fulfilledUnits[i - skipCounter].units,
                    proposedUnits[i].units
                );
            }
        }

        // INVARIANT 2a: vault balance increases by fmul(targetUnits - virtualUnits + 1, totalSupply) + 1, fulfiller balance decreases by the same amount
        // INVARIANT 2b: fulfiller balance increases by fmul(virtualUnits - targetUnits, totalSupply), vault balance decreases by the same amount
        uint256 totalSupply = AMKT.totalSupply();
        // for all old units, check if proposed units was greater than old units. if so, check that vault balance increased by the correct amount
        for (uint256 i = 0; i < oldUnits.length; i++) {
            if (oldUnits[i].units > proposedUnits[i].units) {
                // fulfiller gains, vault loses
                assertEq(
                    IERC20(oldUnits[i].token).balanceOf(address(vault)),
                    oldUnitsVaultBalances[i] -
                        fmul(
                            oldUnits[i].units - proposedUnits[i].units,
                            totalSupply
                        )
                );
                assertEq(
                    IERC20(oldUnits[i].token).balanceOf(address(this)),
                    oldUnitsFulfillerBalances[i] +
                        fmul(
                            oldUnits[i].units - proposedUnits[i].units,
                            totalSupply
                        )
                );
            } else if (oldUnits[i].units < proposedUnits[i].units) {
                // vault gains, fulfiller loses
                assertEq(
                    IERC20(oldUnits[i].token).balanceOf(address(vault)),
                    oldUnitsVaultBalances[i] +
                        (fmul(
                            proposedUnits[i].units - oldUnits[i].units + 1,
                            totalSupply
                        ) + 1)
                );
                assertEq(
                    IERC20(oldUnits[i].token).balanceOf(address(this)),
                    oldUnitsFulfillerBalances[i] -
                        (fmul(
                            proposedUnits[i].units - oldUnits[i].units + 1,
                            totalSupply
                        ) + 1)
                );
            } else {
                assertEq(
                    IERC20(oldUnits[i].token).balanceOf(address(vault)),
                    oldUnitsVaultBalances[i]
                );
                assertEq(
                    IERC20(oldUnits[i].token).balanceOf(address(this)),
                    oldUnitsFulfillerBalances[i]
                );
            }
        }
        // for all new units, check that vault balance increased by the correct amount
        for (uint256 i = oldUnits.length; i < proposedUnits.length; i++) {
            if (proposedUnits[i].units == 0) {
                // if for some reason added proposed units was empty
                // check that vault balance is 0
                assertEq(
                    IERC20(proposedUnits[i].token).balanceOf(address(vault)),
                    0,
                    "9"
                );
            } else {
                assertEq(
                    IERC20(proposedUnits[i].token).balanceOf(address(vault)),
                    proposedUnitsVaultBalances[i] +
                        (fmul(proposedUnits[i].units + 1, totalSupply) + 1)
                );
                assertEq(
                    IERC20(proposedUnits[i].token).balanceOf(address(this)),
                    proposedUnitsFulfillerBalances[i] -
                        (fmul(proposedUnits[i].units + 1, totalSupply) + 1)
                );
            }
        }
    }

    function satisfyFulfillerBalances(
        address fulfiller,
        Bounty memory bounty
    ) internal {
        Dealer dealer = new Dealer();
        (, TokenInfo[] memory ins) = quoter.quoteFulfillBounty(
            bounty,
            AMKT.totalSupply()
        );
        for (uint256 i = 0; i < ins.length; i++) {
            if (ins[i].units == 0) {
                continue;
            }
            IERC20(ins[i].token).approve(
                address(timelockInvokeableBounty),
                type(uint256).max
            );
            dealer.dealToken(ins[i].token, fulfiller, ins[i].units); // only deal exactly the units needed
        }
    }

    function generateValidBounty(
        uint256 numTokensToAdd,
        uint256 rand
    ) internal returns (Bounty memory) {
        TokenInfo[] memory currentUnits = vault.virtualUnits();
        TokenInfo[] memory newUnits = new TokenInfo[](
            currentUnits.length + numTokensToAdd
        );
        // randomize old tokens
        for (uint256 i = 0; i < currentUnits.length; i++) {
            if (rand % 10 == 0) {
                newUnits[i] = TokenInfo({
                    token: currentUnits[i].token,
                    units: 0
                });
            } else if (rand % 2 == 0) {
                newUnits[i] = TokenInfo({
                    token: currentUnits[i].token,
                    units: currentUnits[i].units +
                        (rand % currentUnits[i].units)
                });
            } else {
                // this can also result in 0 units
                newUnits[i] = TokenInfo({
                    token: currentUnits[i].token,
                    units: currentUnits[i].units -
                        (rand % currentUnits[i].units)
                });
            }
        }
        // add new tokens
        for (uint256 i = currentUnits.length; i < newUnits.length; i++) {
            newUnits[i] = TokenInfo({
                token: address(new MockMintableToken("test", "test", 18, 0)),
                units: (rand % 1e18)
            });
            console.log("adding: %s", newUnits[i].token);
        }

        Bounty memory bounty = Bounty({
            infos: newUnits,
            fulfiller: address(this),
            salt: keccak256("test"),
            deadline: block.timestamp + 1000
        });
        return bounty;
    }
}
