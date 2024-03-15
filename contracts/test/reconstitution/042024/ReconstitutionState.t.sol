pragma solidity =0.8.18;
import {ReconstitutionTest} from "test/reconstitution/042024/Reconstitution.t.sol";
import {TokenInfo} from "src/Common.sol";
import {console2} from "forge-std/console2.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {MULTISIG, AMKT_PROXY} from "src/scripts/Config.sol";
import {IVault} from "src/interfaces/IVault.sol";

contract ReconstitutionStateTest_042024 is ReconstitutionTest {
    address[3] tokensAdded = [_21CO_BNB, _21CO_SOL, _21CO_AVAX];
    address[3] tokensRemoved = [WORMHOLE_BNB, WORMHOLE_SOL, WORMHOLE_AVAX];
    address[13] tokensKept = [
        WBTC,
        WSTETH,
        MATIC,
        LINK,
        SHIB,
        UNI,
        ASTETH,
        _21CO_XRP,
        _21CO_ADA,
        _21CO_DOGE,
        _21CO_DOT,
        _21CO_LTC,
        _21CO_BCH
    ];

    function testVirtualUnits() public {
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        assertEq(units.length, 16);
        assertEq(units[0].token, WBTC);
        assertEq(units[0].units, WBTC_UNITS);
        assertEq(units[1].token, WSTETH);
        assertEq(units[1].units, WSTETH_UNITS);
        assertEq(units[2].token, _21CO_BCH);
        assertEq(units[2].units, _21CO_BCH_UNITS);
        assertEq(units[3].token, _21CO_LTC);
        assertEq(units[3].units, _21CO_LTC_UNITS);
        assertEq(units[4].token, MATIC);
        assertEq(units[4].units, MATIC_UNITS);
        assertEq(units[5].token, LINK);
        assertEq(units[5].units, LINK_UNITS);
        assertEq(units[6].token, SHIB);
        assertEq(units[6].units, SHIB_UNITS);
        assertEq(units[7].token, _21CO_DOT);
        assertEq(units[7].units, _21CO_DOT_UNITS);
        assertEq(units[8].token, UNI);
        assertEq(units[8].units, UNI_UNITS);
        assertEq(units[9].token, ASTETH);
        assertEq(units[9].units, ASTETH_UNITS);
        assertEq(units[10].token, _21CO_XRP);
        assertEq(units[10].units, _21CO_XRP_UNITS);
        assertEq(units[11].token, _21CO_ADA);
        assertEq(units[11].units, _21CO_ADA_UNITS);
        assertEq(units[12].token, _21CO_DOGE);
        assertEq(units[12].units, _21CO_DOGE_UNITS);
        assertEq(units[13].token, _21CO_BNB);
        assertEq(units[13].units, _21CO_BNB_UNITS);
        assertEq(units[14].token, _21CO_SOL);
        assertEq(units[14].units, _21CO_SOL_UNITS);
        assertEq(units[15].token, _21CO_AVAX);
        assertEq(units[15].units, _21CO_AVAX_UNITS);
    }

    function testTotalSupplyMatch() public {
        assertEq(salt, keccak256(abi.encode(IERC20(AMKT_PROXY).totalSupply())));
    }
}
