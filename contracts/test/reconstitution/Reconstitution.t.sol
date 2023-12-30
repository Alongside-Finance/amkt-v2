// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {Test} from "forge-std/Test.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {IInvokeableBounty} from "src/interfaces/IInvokeableBounty.sol";
import {IActiveBounty} from "src/interfaces/IActiveBounty.sol";
// import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {GnosisTest, GnosisTransaction} from "test/upgrade/helpers/Gnosis.t.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {TokenInfo} from "src/Common.sol";
import {Dealer} from "test/utils/Dealer.t.sol";
import {BTC, ETH as WSTETH, BNB, SOL, LINK, AVAX, MATIC, SHIB, UNI, MKR, LDO, CRO, MNT, OP, QNT, MULTISIG, AMKT_PROXY} from "src/scripts/Config.sol";
import {Fulfiller} from "periphery/Fulfiller.sol";
import {Quoter} from "periphery/Quoter.sol";
import {console2} from "forge-std/console2.sol";

// THESE NUMBERS WILL BE DETERMINED SHORTLY BEFORE RECONSTITUTION
uint256 constant ASTETH_REMAINDER_AMOUNT = 26000144433693728047; // this will be determined by running the tests after determining units
uint256 constant ADA_UNITS = 3581925;
uint256 constant AVAX_UNITS = 37021598089974576;
uint256 constant BCH_UNITS = 198525;
uint256 constant BNB_UNITS = 15367679603473306;
uint256 constant BTC_UNITS = 198375;
uint256 constant DOGE_UNITS = 1442564286;
uint256 constant DOT_UNITS = 1354579209;
uint256 constant ASTETH_UNITS = 6087764850689860;
uint256 constant WSTETH_UNITS = 5286662159884741;
uint256 constant LINK_UNITS = 57552784774828544;
uint256 constant LTC_UNITS = 749905;
uint256 constant MATIC_UNITS = 968027545433906560;
uint256 constant SHIB_UNITS = 59700570576920143462400;
uint256 constant SOL_UNITS = 43446024;
uint256 constant UNI_UNITS = 60600827954304176;
uint256 constant XRP_UNITS = 5477191;

// STATIC
address constant ASTETH = address(0x27C2B9fd547EAd2c05C305BeE2399A55811257c2);
address constant XRP = address(0x0d3bd40758dF4F79aaD316707FcB809CD4815Ffe);
address constant ADA = address(0x9c05d54645306d4C4EAd6f75846000E1554c0360);
address constant DOGE = address(0xD2aEE1CE2b4459dE326971DE036E82f1318270AF);
address constant DOT = address(0xF4ACCD20bFED4dFFe06d4C85A7f9924b1d5dA819);
address constant LTC = address(0x9F2825333aa7bC2C98c061924871B6C016e385F3);
address constant BCH = address(0xFf4927e04c6a01868284F5C3fB9cba7F7ca4aeC0);

address constant VAULT = address(0xf3bCeDaB2998933c6AAD1cB31430D8bAb329dD8C);
address constant STETH = address(0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84);
address constant FULFILLER = address(
    0xF2bD82133cE54BE7D9A66Bf36240C47f6A874F2e
);

address constant INVOKEABLE_BOUNTY = address(
    0xE13Ee59C41c67696754277cDC73710f6D65Ef2Ac
);
address constant ACTIVE_BOUNTY = address(
    0x0DAF7e851f6054085432229150c1706988aBc562
);

address constant FULFILLER_SAFE = address(
    0x5ae65506979C00D70A13E7cE9eBf984d31660e5c
);
address constant QUOTER = address(0xE3BE63E1B959c152212ce1dD45D0d2f749eB227c);

contract FulfillerSafeTest is GnosisTest {
    Quoter quoter;

    constructor() GnosisTest(FULFILLER_SAFE) {
        quoter = Quoter(QUOTER);
    }

    function runFulfillmentBatch(
        Bounty memory bounty
    ) public returns (bytes memory) {
        enableSimulation();

        GnosisTransaction[] memory batch = new GnosisTransaction[](24);

        (TokenInfo[] memory outs, TokenInfo[] memory ins) = quoter
            .quoteFulfillBounty(bounty, IERC20(AMKT_PROXY).totalSupply());

        for (uint256 i; i < ins.length; i++) {
            if (ins[i].token == ASTETH) continue;
            batch[i] = GnosisTransaction({
                to: address(ins[i].token),
                data: abi.encodeWithSelector(
                    bytes4(keccak256("transfer(address,uint256)")),
                    FULFILLER,
                    ins[i].units
                )
            });
        }

        batch[7] = GnosisTransaction({
            to: FULFILLER,
            data: abi.encodeWithSelector(
                bytes4(
                    keccak256(
                        "fulfillBounty(((address,uint256)[],address,uint256,bytes32),bool)"
                    )
                ),
                bounty,
                true
            )
        });

        for (uint256 i; i < outs.length; i++) {
            if (outs[i].token == WSTETH) {
                batch[i + 8] = GnosisTransaction({
                    to: FULFILLER,
                    data: abi.encodeWithSelector(
                        bytes4(keccak256("withdrawERC20(address,uint256)")),
                        ASTETH,
                        ASTETH_REMAINDER_AMOUNT
                    )
                });
            } else {
                batch[i + 8] = GnosisTransaction({
                    to: FULFILLER,
                    data: abi.encodeWithSelector(
                        bytes4(keccak256("withdrawERC20(address,uint256)")),
                        outs[i].token,
                        outs[i].units
                    )
                });
            }
        }

        batch[23] = GnosisTransaction({
            to: ASTETH,
            data: abi.encodeWithSelector(
                bytes4(keccak256("withdraw(uint256)")),
                ASTETH_REMAINDER_AMOUNT
            )
        });

        bytes memory batchExecutionData = getBatchExecutionData(batch);
        executeBatchData(batchExecutionData);
        return batchExecutionData;
    }
}

contract ReconstitutionTest is GnosisTest {
    Fulfiller fulfiller;

    bool triggerReconstitutionWarning_determineTokens;
    bool triggerReconstitutionWarning_determineAstETHAmount;
    bool triggerReconstitutionWarning_removeForkBlock;
    bool triggerReconstitutionWarning_removeMockBalances;
    bool triggerReconstitutionWarning_postBounty;
    bool triggerReconstitutionWarning_fulfillBounty;

    bytes fulfillmentExecutionData;
    bytes32 salt;
    bytes32 bountyHash;
    FulfillerSafeTest fulfillerSafeTest;

    constructor() GnosisTest(MULTISIG) {}

    function setUp() public {
        fork();
        enableSimulation();
        _setFulfiller();
        _postAndFulfillBounty();
    }

    function testSetUp() public {}

    function fork() public {
        triggerReconstitutionWarning_removeForkBlock = true;
        vm.createSelectFork(vm.envString("MAINNET_RPC"), 18871224);
    }

    function tokens() internal returns (TokenInfo[] memory) {
        triggerReconstitutionWarning_determineTokens = true;
        TokenInfo[] memory _tokens = new TokenInfo[](15 + 7);

        // KEEP
        _tokens[0] = TokenInfo(BTC, BTC_UNITS);
        _tokens[1] = TokenInfo(WSTETH, WSTETH_UNITS);
        _tokens[2] = TokenInfo(BNB, BNB_UNITS);
        _tokens[3] = TokenInfo(SOL, SOL_UNITS);
        _tokens[4] = TokenInfo(MATIC, MATIC_UNITS);
        _tokens[5] = TokenInfo(LINK, LINK_UNITS);
        _tokens[6] = TokenInfo(SHIB, SHIB_UNITS);
        _tokens[7] = TokenInfo(AVAX, AVAX_UNITS);
        _tokens[8] = TokenInfo(UNI, UNI_UNITS);

        // REMOVE
        _tokens[9] = TokenInfo(MKR, 0);
        _tokens[10] = TokenInfo(LDO, 0);
        _tokens[11] = TokenInfo(CRO, 0);
        _tokens[12] = TokenInfo(MNT, 0);
        _tokens[13] = TokenInfo(OP, 0);
        _tokens[14] = TokenInfo(QNT, 0);

        // ADD
        _tokens[15] = TokenInfo(ASTETH, ASTETH_UNITS);
        _tokens[16] = TokenInfo(XRP, XRP_UNITS);
        _tokens[17] = TokenInfo(ADA, ADA_UNITS);
        _tokens[18] = TokenInfo(DOGE, DOGE_UNITS);
        _tokens[19] = TokenInfo(DOT, DOT_UNITS);
        _tokens[20] = TokenInfo(LTC, LTC_UNITS);
        _tokens[21] = TokenInfo(BCH, BCH_UNITS);

        return _tokens;
    }

    function _setFulfiller() public {
        fulfiller = Fulfiller(FULFILLER);
    }

    function _postAndFulfillBounty() internal {
        GnosisTransaction[] memory batch = new GnosisTransaction[](1);
        salt = keccak256(abi.encode("AMKT 01-01-2024 reconstitution"));
        Bounty memory _bountyToSet = Bounty({
            infos: tokens(),
            fulfiller: address(fulfiller),
            salt: salt,
            deadline: 1704585600 // January 7, 2024 0:0:0 GMT
        });

        bountyHash = IInvokeableBounty(INVOKEABLE_BOUNTY).hashBounty(
            _bountyToSet
        );
        batch[0] = GnosisTransaction({
            to: ACTIVE_BOUNTY,
            data: abi.encodeWithSelector(
                bytes4(keccak256("setHash(bytes32)")),
                bountyHash
            )
        });

        triggerReconstitutionWarning_postBounty = true;
        bytes memory batchExecutionData = getBatchExecutionData(batch);
        executeBatchData(batchExecutionData);

        triggerReconstitutionWarning_determineAstETHAmount = true;
        triggerReconstitutionWarning_removeMockBalances = true;
        triggerReconstitutionWarning_fulfillBounty = true;

        fulfillerSafeTest = new FulfillerSafeTest();
        fulfillmentExecutionData = fulfillerSafeTest.runFulfillmentBatch(
            _bountyToSet
        );
    }

    function test_state() public {
        address[9] memory tokensKept = [
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
        address[7] memory tokensAdded = [ASTETH, XRP, ADA, DOGE, DOT, LTC, BCH];
        address[6] memory tokensRemoved = [MKR, LDO, CRO, MNT, OP, QNT];
        TokenInfo[] memory units = IVault(VAULT).virtualUnits();
        assertEq(units.length, 16);
        assertEq(units[0].token, BTC);
        assertEq(
            units[15].token,
            address(0xFf4927e04c6a01868284F5C3fB9cba7F7ca4aeC0)
        );
        assertEq(units[0].units, BTC_UNITS);
        assertEq(units[15].units, BCH_UNITS);

        for (uint256 i = 0; i < tokensKept.length; i++) {
            assertEq(units[i].token, tokensKept[i]);
            assertGt(units[i].units, 0);
            assertEq(IERC20(tokensKept[i]).balanceOf(FULFILLER), 0);
            if (tokensKept[i] != WSTETH) {
                assertGt(IERC20(tokensKept[i]).balanceOf(FULFILLER_SAFE), 0);
                console2.log(
                    tokensKept[i],
                    IERC20(tokensKept[i]).balanceOf(FULFILLER_SAFE) /
                        (10 ** IERC20(tokensKept[i]).decimals())
                );
            }
        }
        for (uint256 i = 0; i < tokensAdded.length; i++) {
            assertEq(units[i + 9].token, tokensAdded[i]);
            assertGt(units[i + 9].units, 0);
            assertEq(IERC20(tokensAdded[i]).balanceOf(FULFILLER), 0);
            if (tokensAdded[i] != ASTETH) {
                assertGt(IERC20(tokensAdded[i]).balanceOf(FULFILLER_SAFE), 0);
                console2.log(
                    tokensAdded[i],
                    IERC20(tokensAdded[i]).balanceOf(FULFILLER_SAFE) /
                        (10 ** IERC20(tokensAdded[i]).decimals())
                );
            }
        }
        for (uint256 i = 0; i < tokensRemoved.length; i++) {
            assertGt(IERC20(tokensRemoved[i]).balanceOf(FULFILLER_SAFE), 0);
            console2.log(
                tokensRemoved[i],
                IERC20(tokensRemoved[i]).balanceOf(FULFILLER_SAFE) /
                    (10 ** IERC20(tokensRemoved[i]).decimals())
            );
        }
        console2.log(
            STETH,
            IERC20(STETH).balanceOf(FULFILLER_SAFE) /
                (10 ** IERC20(STETH).decimals())
        );

        // check that fulfiller has positive balance of certain tokens
        assertGt(IERC20(BTC).balanceOf(FULFILLER_SAFE), 0);
        assertGt(IERC20(BNB).balanceOf(FULFILLER_SAFE), 0);
        assertGt(IERC20(SOL).balanceOf(FULFILLER_SAFE), 0);
        assertGt(IERC20(STETH).balanceOf(FULFILLER_SAFE), 0);

        // check balance of multisig
        // loop through all known tokens and console log multisig's balance
    }
}
