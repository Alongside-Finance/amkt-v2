pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {GnosisTest, GnosisTransaction} from "./Gnosis.t.sol";
import {CoreDeployScript} from "src/scripts/__0__CoreDeploy.s.sol";
import {InitialBountyHelper, AMKT, MULTISIG, PROXY_ADMIN, PROXY, INFLATION_RATE, AMKT as AMKTAddress} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {Bounty, InvokeableBounty} from "src/invoke/Bounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {Dealer} from "test/Dealer.t.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {IndexToken} from "src/IndexToken.sol";
import {Vault} from "src/Vault.sol";
import {AlongsideGovernor} from "src/Governor.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import {Issuance} from "src/invoke/Issuance.sol";
import {fmul} from "src/lib/FixedPoint.sol";

contract UpgradeTest is GnosisTest {
    IndexToken AMKT;
    Vault vault;
    Issuance issuance;
    InvokeableBounty invokeableBounty;
    ActiveBounty activeBounty;
    AlongsideGovernor governor;
    TimelockController timelockController;
    address newTokenImplementation;
    InvokeableBounty timelockInvokeableBounty;
    ActiveBounty timelockActiveBounty;

    function setUp() public {
        vm.createSelectFork(vm.envString("MAINNET_RPC"), 18229914);
        enableSimulation();
        setDeployedContracts();
        mockSafeBalances(); // TODO: Remove when ready. Due before bundle submission.
        checkSafeBalances();
        GnosisTransaction[] memory batch = createUpgradeBatch();
        _warpForward(1 hours); // there will be some time after we craft the batch, and we execute it
        bytes memory dataExecuted = executeBatch(batch);
    }

    function setDeployedContracts() internal {
        AMKT = IndexToken(AMKTAddress);
        // vault = Vault(0xD62A80368AdF5919f70193D15dCbD5C77EAf55ac);
        // issuance = Issuance(0x58AD9D36AfAc51206672f855Bf7e76037c5F5198);
        // invokeableBounty = InvokeableBounty(
        //     0x366A647DE921608bee3987025D23f12263da6884
        // );
        // activeBounty = ActiveBounty(0x12bc3CCaA2E213e9D50faB9752A9daFac01b962F);
        // governor = AlongsideGovernor(
        //     payable(0x774045B30e6fC5DfE73bF386E8845CA1472fb45e)
        // );
        // timelockController = TimelockController(
        //     payable(0xB3970Ae79fD2cD8f1060cF6BAeae27b8E2c05437)
        // );
        // newTokenImplementation = address(
        //     0x775715D96cD3B3586728B7420A13Ec74f5dc9e8f
        // );

        // timelockActiveBounty = ActiveBounty(
        //     0x8D2A6bcB5713d4b57f2FffB119B7B6D0143e25ed
        // );
        // timelockInvokeableBounty = InvokeableBounty(
        //     0x703814F9172D6E6EF10F89fCAdE3ff480d812a45
        // );
        CoreDeployScript script = new CoreDeployScript(); // TODO: Remove when ready. Due before external review.
        CoreDeployScript.DeployedContracts memory deployed = script.run(); // TODO: Remove when ready. Due vefore external review.
        vault = deployed.vault;
        issuance = deployed.issuance;
        invokeableBounty = deployed.invokeableBounty;
        activeBounty = deployed.activeBounty;
        governor = deployed.governor;
        timelockController = deployed.timelockController;
        newTokenImplementation = deployed.newTokenImplementation;
        timelockInvokeableBounty = deployed.timelockInvokeableBounty;
        timelockActiveBounty = deployed.timelockActiveBounty;
        _warpForward(3 days + 2 hours);
    }

    function mockSafeBalances() internal {
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        for (uint256 i = 0; i < tokens.length; i++) {
            dealer.dealToken(
                tokens[i].token,
                MULTISIG,
                fmul(tokens[i].units, AMKT.totalSupply())
            );
        }
    }

    function checkSafeBalances() internal {
        // check that the safe balance matches exactly what the initial bounty helper expects
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(
                token.balanceOf(MULTISIG),
                fmul(tokens[i].units, AMKT.totalSupply())
            );
        }
    }

    function createUpgradeBatch() public returns (GnosisTransaction[] memory) {
        // Initialize batch with known size
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens(); // TODO: Replace token units with real values. Due before submission.
        uint256 batchLength = tokens.length + 8;
        GnosisTransaction[] memory batch = new GnosisTransaction[](batchLength);

        // First 15 transactions are approving each token in the index for the issuance contract
        for (uint256 i = 0; i < 15; i++) {
            batch[i] = GnosisTransaction({
                to: tokens[i].token,
                data: abi.encodeWithSelector(
                    bytes4(keccak256("approve(address,uint256)")),
                    invokeableBounty,
                    2 ** 256 - 1
                )
            });
        }

        // Set hash to ActiveBounty for initial bounty
        Bounty memory _bountyToSet = Bounty({
            infos: tokens,
            fulfiller: MULTISIG,
            salt: keccak256(abi.encode(block.timestamp)),
            deadline: block.timestamp + 1 days
        });

        bytes32 hashToSet = InvokeableBounty(invokeableBounty).hashBounty(
            _bountyToSet
        );

        batch[15] = GnosisTransaction({
            to: address(activeBounty),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setHash(bytes32)")),
                hashToSet
            )
        });

        // Accept ownership of Vault
        batch[16] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(bytes4(keccak256("acceptOwnership()")))
        });

        // Fulfill initial bounty
        batch[17] = GnosisTransaction({
            to: address(invokeableBounty),
            data: abi.encodeWithSelector(
                bytes4(
                    keccak256(
                        "fulfillBounty(((address,uint256)[],address,uint256,bytes32))"
                    )
                ),
                _bountyToSet,
                false
            )
        });

        // Upgrade and initialize
        batch[18] = GnosisTransaction({
            to: PROXY_ADMIN,
            data: abi.encodeWithSelector(
                bytes4(keccak256("upgradeAndCall(address,address,bytes)")),
                PROXY,
                newTokenImplementation,
                abi.encodeWithSignature("initialize(address)", vault)
            )
        });

        // Set fee scaled
        batch[19] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setInflationRate(uint256)")),
                INFLATION_RATE
            )
        });

        // Set rebalancer to timeblock bounty
        batch[20] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("setRebalancer(address)")),
                timelockInvokeableBounty
            )
        });

        // Transfer vault ownership to timelock
        batch[21] = GnosisTransaction({
            to: address(vault),
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                timelockController
            )
        });

        // Transfer proxyAdmin ownership to timelock
        batch[22] = GnosisTransaction({
            to: PROXY_ADMIN,
            data: abi.encodeWithSelector(
                bytes4(keccak256("transferOwnership(address)")),
                timelockController
            )
        });
        return batch;
    }
}
