pragma solidity =0.8.18;
import {ReconstitutionTest} from "test/reconstitution/012024/Reconstitution.t.sol";
import {TokenInfo} from "src/Common.sol";
import {console2} from "forge-std/console2.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {BTC, ETH as WSTETH, BNB, SOL, LINK, AVAX, MATIC, SHIB, UNI, MKR, LDO, CRO, MNT, OP, QNT, MULTISIG, AMKT_PROXY} from "src/scripts/Config.sol";
import {IVault} from "src/interfaces/IVault.sol";

contract ReconstitutionStateTest is ReconstitutionTest {
    address constant ASTETH =
        address(0x27C2B9fd547EAd2c05C305BeE2399A55811257c2);
    address constant XRP = address(0x0d3bd40758dF4F79aaD316707FcB809CD4815Ffe);
    address constant ADA = address(0x9c05d54645306d4C4EAd6f75846000E1554c0360);
    address constant DOGE = address(0xD2aEE1CE2b4459dE326971DE036E82f1318270AF);
    address constant DOT = address(0xF4ACCD20bFED4dFFe06d4C85A7f9924b1d5dA819);
    address constant LTC = address(0x9F2825333aa7bC2C98c061924871B6C016e385F3);
    address constant BCH = address(0xFf4927e04c6a01868284F5C3fB9cba7F7ca4aeC0);
    address constant VAULT =
        address(0xf3bCeDaB2998933c6AAD1cB31430D8bAb329dD8C);
    address constant STETH =
        address(0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84);
    address constant FULFILLER =
        address(0xF2bD82133cE54BE7D9A66Bf36240C47f6A874F2e);

    address constant INVOKEABLE_BOUNTY =
        address(0xE13Ee59C41c67696754277cDC73710f6D65Ef2Ac);
    address constant ACTIVE_BOUNTY =
        address(0x0DAF7e851f6054085432229150c1706988aBc562);

    address constant FULFILLER_SAFE =
        address(0x5ae65506979C00D70A13E7cE9eBf984d31660e5c);
    address constant QUOTER =
        address(0xE3BE63E1B959c152212ce1dD45D0d2f749eB227c);

    address[7] tokensAdded = [ASTETH, XRP, ADA, DOGE, DOT, LTC, BCH];
    address[6] tokensRemoved = [MKR, LDO, CRO, MNT, OP, QNT];
    address[9] tokensKept = [
        BTC,
        WSTETH,
        BNB,
        SOL,
        MATIC,
        LINK,
        SHIB,
        AVAX,
        UNI
    ];

    function testVirtualUnits() public {
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        assertEq(units.length, 16);
        assertEq(units[0].token, BTC);
        assertEq(
            units[15].token,
            address(0xFf4927e04c6a01868284F5C3fB9cba7F7ca4aeC0)
        );
    }

    function testStETHBalance() public {
        console2.log(IERC20(STETH).balanceOf(FULFILLER_SAFE));
        console2.log(IERC20(ASTETH).balanceOf(FULFILLER_SAFE));
        console2.log(IERC20(WSTETH).balanceOf(FULFILLER_SAFE));
        console2.log(IERC20(STETH).balanceOf(FULFILLER));
        console2.log(IERC20(ASTETH).balanceOf(FULFILLER));
        console2.log(IERC20(WSTETH).balanceOf(FULFILLER));
    }

    function testTokensKept() public {
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        for (uint256 i = 0; i < tokensKept.length; i++) {
            assertEq(units[i].token, tokensKept[i]);
            assertGt(units[i].units, 0);
            assertEq(IERC20(tokensKept[i]).balanceOf(FULFILLER), 0);
            if (tokensKept[i] != WSTETH) {
                assertGt(IERC20(tokensKept[i]).balanceOf(FULFILLER_SAFE), 0);
                console2.log(
                    tokensKept[i],
                    IERC20(tokensKept[i]).balanceOf(FULFILLER_SAFE)
                );
            }
        }
    }

    function testTokensAdded() public {
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        for (uint256 i = 0; i < tokensAdded.length; i++) {
            assertEq(units[i + 9].token, tokensAdded[i]);
            assertGt(units[i + 9].units, 0);
            assertEq(IERC20(tokensAdded[i]).balanceOf(FULFILLER), 0);
            if (tokensAdded[i] != ASTETH) {
                assertGt(IERC20(tokensAdded[i]).balanceOf(FULFILLER_SAFE), 0);
                console2.log(
                    tokensAdded[i],
                    IERC20(tokensAdded[i]).balanceOf(FULFILLER_SAFE)
                );
            }
        }
    }

    function testTokensRemoved() public {
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        for (uint256 i = 0; i < tokensRemoved.length; i++) {
            assertGt(IERC20(tokensRemoved[i]).balanceOf(FULFILLER_SAFE), 0);
            console2.log(
                tokensRemoved[i],
                IERC20(tokensRemoved[i]).balanceOf(FULFILLER_SAFE)
            );
        }
    }

    function testFulfillerSafeBalances() public {
        console2.log(
            STETH,
            IERC20(STETH).balanceOf(FULFILLER_SAFE) /
                (10 ** IERC20(STETH).decimals())
        );
        assertGt(IERC20(BTC).balanceOf(FULFILLER_SAFE), 0);
        assertGt(IERC20(BNB).balanceOf(FULFILLER_SAFE), 0);
        assertGt(IERC20(SOL).balanceOf(FULFILLER_SAFE), 0);
        assertGt(IERC20(STETH).balanceOf(FULFILLER_SAFE), 0);
    }

    function testTotalSupplyMatch() public {
        assertEq(salt, keccak256(abi.encode(IERC20(AMKT_PROXY).totalSupply())));
    }
}
