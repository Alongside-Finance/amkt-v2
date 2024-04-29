pragma solidity =0.8.18;
import {ReconstitutionTest} from "test/reconstitution/042024-21co/Reconstitution.t.sol";
import {TokenInfo} from "src/Common.sol";
import {console2} from "forge-std/console2.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {MULTISIG, AMKT_PROXY} from "src/scripts/Config.sol";
import {IVault} from "src/interfaces/IVault.sol";

contract ReconstitutionStateTest_042024_21co is ReconstitutionTest {
    function testVirtualUnits() public {
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        assertEq(units.length, 16);
        for (uint256 i = 0; i < units.length; i++) {
            assertFalse(units[i].token == WORMHOLE_BNB);
            assertFalse(units[i].token == WORMHOLE_SOL);
        }
    }

    function testTotalSupplyMatch() public {
        assertEq(salt, keccak256(abi.encode(IERC20(AMKT_PROXY).totalSupply())));
    }

    // TODO: Write specific tests
}
