pragma solidity =0.8.18;

import {UpgradeTest} from "./helpers/Upgrade.t.sol";
import {Dealer} from "test/Dealer.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {InvokeableBounty, Bounty} from "src/invoke/Bounty.sol";
import {MockMintableToken} from "mocks/MockMintableToken.sol";
import {MULTISIG} from "src/scripts/Config.sol";
import {fmul} from "src/lib/FixedPoint.sol";
import {console} from "forge-std/console.sol";

contract UpgradedBountyTest is UpgradeTest {
    function testBountyInvariant(uint256 numTokensToAdd, uint256 rand) public {
        numTokensToAdd = bound(numTokensToAdd, 0, 25);
        TokenInfo[] memory oldUnits = vault.virtualUnits();
        Bounty memory bounty = generateValidBounty(numTokensToAdd, rand);
        TokenInfo[] memory proposedUnits = bounty.infos;
        bytes32 _hash = timelockInvokeableBounty.hashBounty(bounty);
        vm.prank(address(timelockController));
        timelockActiveBounty.setHash(_hash);
        satisfyFulfillerBalances(address(this), bounty);
        timelockInvokeableBounty.fulfillBounty(bounty, false);
        // INVARIANT 1: proposed units are the same as fulfilled units
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
    }

    function satisfyFulfillerBalances(
        address fulfiller,
        Bounty memory bounty
    ) internal {
        Dealer dealer = new Dealer();
        TokenInfo[] memory units = bounty.infos;
        for (uint256 i = 0; i < units.length; i++) {
            if (units[i].units == 0) {
                continue;
            }
            IERC20(units[i].token).approve(
                address(timelockInvokeableBounty),
                type(uint256).max
            );
            dealer.dealToken(units[i].token, fulfiller, type(uint256).max / 10); // a very large number, but wouldn't overflow with existing supply
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
                units: rand % 1e18
            });
        }

        Bounty memory bounty = Bounty({
            infos: newUnits,
            fulfiller: address(this),
            salt: keccak256("test"),
            deadline: block.timestamp + 1000
        });

        // loop through infos and log them all
        for (uint256 i = 0; i < bounty.infos.length; i++) {
            console.log(
                "token: %s, units: %s",
                bounty.infos[i].token,
                bounty.infos[i].units
            );
        }

        return bounty;
    }
}
