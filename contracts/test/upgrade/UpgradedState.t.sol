pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {InitialBountyHelper, MULTISIG, FEE_RECEIPIENT, FEE_SCALED, PROXY, PROXY_ADMIN} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {UpgradeTest} from "./Upgrade.t.sol";
import {ITransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

contract UpgradedState is UpgradeTest {
    function testConfig() public {
        assertEq(FEE_RECEIPIENT, 0xC19a5b6E0a923519603985153515222D59cb3F2e);
        assertEq(MULTISIG, 0xAeB9ef94b6542BE7112f3a295646B5AaAa9Fca13);
        assertEq(FEE_SCALED, 26151474053915);
        assertEq(
            address(AMKT),
            address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D)
        );
        assertEq(
            address(PROXY),
            address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D)
        );
        assertEq(
            address(PROXY_ADMIN),
            address(0x998930C351EcB4918A5c5238B62d5277fE45ab9b)
        );
    }

    function testGovernanceConfig() public {
        uint256 AVG_BLOCK_TIME = 12;
        assertEq(governor.votingDelay(), 1 days / AVG_BLOCK_TIME);
        assertEq(governor.votingPeriod(), 4 days / AVG_BLOCK_TIME);
        assertEq(governor.proposalThreshold(), 100e18);
        assertEq(governor.quorumNumerator(), 5);
        assertEq(governor.quorumDenominator(), 100);
        assertEq(governor.timelock(), address(timelockController));
        assertEq(timelockController.getMinDelay(), 4 days);
    }

    function testDeployedContracts() public {
        assertEq(
            address(vault),
            address(0xD62A80368AdF5919f70193D15dCbD5C77EAf55ac)
        );
        assertEq(
            address(issuance),
            address(0x58AD9D36AfAc51206672f855Bf7e76037c5F5198)
        );
        assertEq(
            address(invokeableBounty),
            address(0x366A647DE921608bee3987025D23f12263da6884)
        );
        assertEq(
            address(activeBounty),
            address(0x12bc3CCaA2E213e9D50faB9752A9daFac01b962F)
        );
        assertEq(
            address(governor),
            address(payable(0x774045B30e6fC5DfE73bF386E8845CA1472fb45e))
        );
        assertEq(
            address(timelockController),
            address(payable(0xB3970Ae79fD2cD8f1060cF6BAeae27b8E2c05437))
        );
        assertEq(
            newTokenImplementation,
            address(0x775715D96cD3B3586728B7420A13Ec74f5dc9e8f)
        );

        assertEq(
            address(timelockActiveBounty),
            address(0x8D2A6bcB5713d4b57f2FffB119B7B6D0143e25ed)
        );
        assertEq(
            address(timelockInvokeableBounty),
            address(0x703814F9172D6E6EF10F89fCAdE3ff480d812a45)
        );
    }

    function testProxyState() public {
        ITransparentUpgradeableProxy proxy = ITransparentUpgradeableProxy(
            PROXY
        );
        ProxyAdmin proxyAdmin = ProxyAdmin(PROXY_ADMIN);
        assertEq(
            proxyAdmin.getProxyImplementation(proxy),
            newTokenImplementation
        );
        assertEq(proxyAdmin.getProxyAdmin(proxy), PROXY_ADMIN);
        assertEq(proxyAdmin.owner(), address(timelockController));
    }

    function testTokenState() public {
        assertEq(AMKT.minter(), address(vault));
        assertEq(AMKT.decimals(), 18);
        assertEq(AMKT.symbol(), "AMKT");
        assertEq(AMKT.name(), "Alongside Crypto Market Index");
        assertEq(
            AMKT.balanceOf(address(0x209ADBAad63c3008B5C2edb941B991Ef9Bb35027)),
            200e18
        ); // random user with 200 balance
        assertEq(
            AMKT.balanceOf(address(0x5c90090405d0dFfe53F385925E7F0DA064C4CA05)),
            100e18
        ); // random user with 100 balance
        // TODO: supply check
    }

    function testVaultState() public {
        assertEq(vault.underlyingLength(), 15); // TODO: Increase to 15
        assertEq(vault.issuance(), address(issuance));
        assertEq(vault.rebalancer(), address(timelockInvokeableBounty));
        assertEq(vault.feeRecipient(), FEE_RECEIPIENT);
        assertEq(vault.emergencyResponder(), MULTISIG);
        assertEq(vault.emergency(), false);
        assertEq(address(vault.indexToken()), address(AMKT));
        assertEq(vault.feeScaled(), FEE_SCALED);
        assertEq(vault.owner(), MULTISIG);
        assertEq(vault.pendingOwner(), address(timelockController));
    }

    function testBountyState() public {
        assertEq(address(invokeableBounty.indexToken()), address(AMKT));
        assertEq(address(invokeableBounty.vault()), address(vault));
        assertEq(
            address(invokeableBounty.activeBounty()),
            address(activeBounty)
        );
        assertEq(invokeableBounty.version(), 0);
        assertEq(invokeableBounty.chainId(), 1);
        assertEq(activeBounty.authority(), MULTISIG);

        assertEq(address(timelockInvokeableBounty.indexToken()), address(AMKT));
        assertEq(address(timelockInvokeableBounty.vault()), address(vault));
        assertEq(
            address(timelockInvokeableBounty.activeBounty()),
            address(timelockActiveBounty)
        );
        assertEq(timelockInvokeableBounty.version(), 0);
        assertEq(timelockInvokeableBounty.chainId(), 1);
        assertEq(timelockActiveBounty.authority(), address(timelockController));
    }

    function testTokens() public {
        // for all tokens in vault, it should match tokens in bounty helper
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        address[] memory underlying = vault.underlying();
        for (uint256 i = 0; i < tokens.length; i++) {
            assertEq(underlying[i], tokens[i].token);
        }
    }

    function testVirtualUnitsMatchInitialBounty() public {
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        TokenInfo[] memory virtualUnits = vault.virtualUnits();
        for (uint256 i = 0; i < tokens.length; i++) {
            assertEq(virtualUnits[i].units, tokens[i].units);
            assertEq(virtualUnits[i].token, tokens[i].token);
        }
    }

    function testExpectedSafeBalances() public {
        // for all tokens in vault, safe balance should be zero
        address[] memory underlying = vault.underlying();
        for (uint256 i = 0; i < vault.underlyingLength(); i++) {
            IERC20 token = IERC20(underlying[i]);
            assertEq(token.balanceOf(MULTISIG), 0);
        }
    }

    function testExpectedVaultBalances() public {
        // for all tokens in vault, vault balance should match NAV
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        address[] memory underlying = vault.underlying();

        for (uint256 i = 0; i < vault.underlyingLength(); i++) {
            IERC20 token = IERC20(underlying[i]);
            assertEq(
                token.balanceOf(address(vault)),
                (tokens[i].units * AMKT.totalSupply()) / 1e18
            );
        }
    }

    function testNeqErc20PermitDomainSeparator() public {
        bytes32 typeHash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        bytes32 nameHash = keccak256(bytes(AMKT.name()));
        bytes32 versionHash = keccak256(bytes("1"));
        uint256 chainId = 1;
        address token = address(AMKT);
        bytes32 expectedDomainSeparator = keccak256(
            abi.encode(typeHash, nameHash, versionHash, chainId, token)
        );
        assertNotEq(AMKT.DOMAIN_SEPARATOR(), expectedDomainSeparator);
    }

    function testEqErc20PermitDomainSeparator() public {
        bytes32 typeHash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        bytes32 nameHash = keccak256(bytes(AMKT.name()));
        bytes32 versionHash = keccak256(bytes("2"));
        uint256 chainId = 1;
        address token = address(AMKT);
        bytes32 expectedDomainSeparator = keccak256(
            abi.encode(typeHash, nameHash, versionHash, chainId, token)
        );
        assertEq(AMKT.DOMAIN_SEPARATOR(), expectedDomainSeparator);
    }
}
