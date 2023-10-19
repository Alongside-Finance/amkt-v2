pragma solidity =0.8.18;

import {StatefulTest} from "core-test/State.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SCALAR, fmul} from "src/lib/FixedPoint.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {IIssuance} from "src/interfaces/IIssuance.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty, IInvokeableBounty} from "src/interfaces/IInvokeableBounty.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {MULTISIG} from "src/scripts/Config.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";
import {IActiveBounty} from "src/interfaces/IActiveBounty.sol";

contract BountyInvariantTest is StatefulTest {
    InvokeableBounty invokeableBounty;
    address[10] mockTokens;

    function setUp() public override {
        super.setUp();
        invokeableBounty = InvokeableBounty(bounty);
        seedInitial(10);
        excludeContract(address(indexToken));
        // add new tokens
        for (uint256 i = 0; i < 10; i++) {
            MockMintableToken newToken = new MockMintableToken(
                "TEST",
                "TEST",
                18,
                0
            );
            mockTokens[i] = address(newToken);
        }
    }

    function _satisfyFulfillerBalances(
        address fulfiller,
        Bounty memory bounty
    ) internal {
        (, TokenInfo[] memory ins) = quoter.quoteFulfillBounty(
            bounty,
            indexToken.totalSupply()
        );
        for (uint256 i = 0; i < ins.length; i++) {
            if (ins[i].units == 0) {
                continue;
            }
            IERC20(ins[i].token).approve(
                address(invokeableBounty),
                type(uint256).max
            );
            MockMintableToken(ins[i].token).mint(fulfiller, ins[i].units);
        }
    }

    function _generateValidBounty(
        uint256 numTokensToAdd,
        uint256 rand
    ) internal view returns (Bounty memory) {
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
                token: mockTokens[newUnits.length - currentUnits.length - 1],
                units: (rand % 1e18)
            });
        }

        Bounty memory bounty = Bounty({
            infos: newUnits,
            fulfiller: address(this),
            salt: keccak256("test"),
            deadline: block.timestamp + 1000
        });
        return bounty;
    }

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
        Bounty memory bounty = _generateValidBounty(numTokensToAdd, rand);
        TokenInfo[] memory proposedUnits = bounty.infos;
        bytes32 _hash = invokeableBounty.hashBounty(bounty);

        // set bounty by anyone except fulfiller should fail
        vm.startPrank(address(3));
        vm.expectRevert(IActiveBounty.ActiveBountyAuth.selector);
        activeBounty.setHash(_hash);
        vm.stopPrank();

        // set bounty by authority should succeed
        vm.prank(authority);
        activeBounty.setHash(_hash);
        _satisfyFulfillerBalances(address(this), bounty);

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
        invokeableBounty.fulfillBounty(bounty, false);
        vm.stopPrank();

        // fulfill bounty by fulfiller should succeed
        invokeableBounty.fulfillBounty(bounty, false);
        return (
            oldUnits,
            proposedUnits,
            oldUnitsVaultBalances,
            oldUnitsFulfillerBalances,
            proposedUnitsVaultBalances,
            proposedUnitsFulfillerBalances
        );
    }

    function invariant_bounty_formula_reconstitution() public {
        uint256 numTokensToAdd = 10;
        uint256 rand = 1e25;
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
        uint256 totalSupply = indexToken.totalSupply();
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
}
