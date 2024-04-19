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
import {MULTISIG, AMKT_PROXY} from "src/scripts/Config.sol";
import {Fulfiller} from "periphery/Fulfiller.sol";
import {Quoter} from "periphery/Quoter.sol";
import {console2} from "forge-std/console2.sol";
import {Constants} from "test/reconstitution/042024-21co/Constants.t.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";

contract FulfillerSafeTest is BaseTest, Constants {
    Quoter quoter;

    constructor() {
        quoter = Quoter(QUOTER);
    }

    function _satisfyFulfillerBalances(
        address fulfiller,
        Bounty memory bounty
    ) internal {
        Dealer dealer = new Dealer();
        (, TokenInfo[] memory ins) = quoter.quoteFulfillBounty(
            bounty,
            IERC20(AMKT_PROXY).totalSupply()
        );
        for (uint256 i = 0; i < ins.length; i++) {
            if (ins[i].units == 0) {
                continue;
            }
            uint256 currentBalance = IERC20(ins[i].token).balanceOf(fulfiller);
            uint256 neededBalance = ins[i].units;
            if (currentBalance < neededBalance) {
                dealer.dealToken(
                    ins[i].token,
                    fulfiller,
                    neededBalance - currentBalance
                );
                console2.log(
                    "missing: ",
                    ins[i].token,
                    neededBalance - currentBalance,
                    IERC20(ins[i].token).decimals()
                );
            }
        }
    }

    function runFulfillmentBatch(
        Bounty memory bounty
    ) public returns (bytes memory) {
        // TODO: Remove once fulfiller has the necessary balances
        _satisfyFulfillerBalances(FULFILLER_SAFE, bounty);

        (TokenInfo[] memory outs, TokenInfo[] memory ins) = quoter
            .quoteFulfillBounty(bounty, IERC20(AMKT_PROXY).totalSupply());
        GnosisTransaction[] memory batch = new GnosisTransaction[](
            ins.length + 1
        );
        vm.startPrank(FULFILLER_SAFE);
        for (uint256 i; i < ins.length; i++) {
            if (ins[i].token == ASTETH) continue;
            IERC20(ins[i].token).approve(INVOKEABLE_BOUNTY, ins[i].units);
        }
        IInvokeableBounty(INVOKEABLE_BOUNTY).fulfillBounty(bounty, false);
        vm.stopPrank();
    }
}

contract ReconstitutionTest is GnosisTest, Constants {
    Fulfiller fulfiller;

    bytes fulfillmentExecutionData;
    bytes32 salt;
    bytes32 bountyHash;
    FulfillerSafeTest fulfillerSafeTest;

    constructor() GnosisTest(MULTISIG) {}

    function setUp() public {
        fork();
        enableSimulation();
        postAndFulfillBounty();
    }

    function testSetUp() public {}

    function fork() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC"), FORK_BLOCK);
    }

    function tokens() internal returns (TokenInfo[] memory) {
        TokenInfo[] memory _tokens = new TokenInfo[](19);

        // PREVIOUS TOKENS
        _tokens[0] = TokenInfo(WBTC, WBTC_UNITS);
        _tokens[1] = TokenInfo(WSTETH, WSTETH_UNITS);
        _tokens[2] = TokenInfo(WORMHOLE_BNB, WORMHOLE_BNB_UNITS);
        _tokens[3] = TokenInfo(WORMHOLE_SOL, WORMHOLE_SOL_UNITS);
        _tokens[4] = TokenInfo(MATIC, MATIC_UNITS);
        _tokens[5] = TokenInfo(LINK, LINK_UNITS);
        _tokens[6] = TokenInfo(SHIB, SHIB_UNITS);
        _tokens[7] = TokenInfo(WORMHOLE_AVAX, WORMHOLE_AVAX_UNITS);
        _tokens[8] = TokenInfo(UNI, UNI_UNITS);
        _tokens[9] = TokenInfo(ASTETH, ASTETH_UNITS);
        _tokens[10] = TokenInfo(_21CO_XRP, _21CO_XRP_UNITS);
        _tokens[11] = TokenInfo(_21CO_ADA, _21CO_ADA_UNITS);
        _tokens[12] = TokenInfo(_21CO_DOGE, _21CO_DOGE_UNITS);
        _tokens[13] = TokenInfo(_21CO_DOT, _21CO_DOT_UNITS);
        _tokens[14] = TokenInfo(_21CO_LTC, _21CO_LTC_UNITS);
        _tokens[15] = TokenInfo(_21CO_BCH, _21CO_BCH_UNITS);
        // ADD
        _tokens[16] = TokenInfo(_21CO_BNB, _21CO_BNB_UNITS);
        _tokens[17] = TokenInfo(_21CO_SOL, _21CO_SOL_UNITS);
        _tokens[18] = TokenInfo(_21CO_AVAX, _21CO_AVAX_UNITS);

        return _tokens;
    }

    function postAndFulfillBounty() internal {
        GnosisTransaction[] memory batch = new GnosisTransaction[](1);
        salt = keccak256(abi.encode(PREVIOUS_TOTAL_SUPPLY)); // total supply as an arbitrary salt, not sensitive.
        Bounty memory _bountyToSet = Bounty({
            infos: tokens(),
            fulfiller: FULFILLER_SAFE,
            salt: salt,
            deadline: BOUNTY_DEADLINE
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

        bytes memory batchExecutionData = getBatchExecutionData(batch);
        executeBatchData(batchExecutionData);

        fulfillerSafeTest = new FulfillerSafeTest();
        fulfillmentExecutionData = fulfillerSafeTest.runFulfillmentBatch(
            _bountyToSet
        );
    }
}
