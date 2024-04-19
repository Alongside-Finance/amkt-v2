pragma solidity =0.8.18;
import {ReconstitutionTest} from "test/reconstitution/042024-21co/Reconstitution.t.sol";
import {TokenInfo} from "src/Common.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {BTC, ETH as WSTETH, BNB, SOL} from "src/scripts/Config.sol";
import {console2} from "forge-std/console2.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// NOTE TO DEVELOPERS:
// This checklist test is used to ensure safety of the reconstitution process.
// If any one of these tests fail, it means that it is not safe to reconstitute.
// These tests should guide the reconstitution process, step by step, to ensure that the reconstitution is safe.
contract ReconstitutionChecklistTest_042024_21co is ReconstitutionTest {
    function test_printBountyHash() public view {
        bool shouldPrintBountyHash = true;
        if (shouldPrintBountyHash) {
            console2.logBytes32(bountyHash);
        }
    }

    function test_printSalt() public view {
        bool shouldPrintSalt = true;
        if (shouldPrintSalt) {
            console2.logBytes32(salt);
        }
    }
}
