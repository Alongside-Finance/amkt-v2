pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {InitialBountyHelper, MULTISIG, FEE_RECEIPIENT, INFLATION_RATE, PROXY_ADMIN, AMKT_PROXY} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {ITransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {fmul} from "src/lib/FixedPoint.sol";

contract UpgradedStateTest is UpgradedTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);

    function testConfig() public {
        assertEq(FEE_RECEIPIENT, 0xC19a5b6E0a923519603985153515222D59cb3F2e);
        assertEq(MULTISIG, 0xAeB9ef94b6542BE7112f3a295646B5AaAa9Fca13);
        assertEq(INFLATION_RATE, 304132280);
        assertEq(
            address(AMKT),
            address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D)
        );
        assertEq(
            address(AMKT_PROXY),
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
        assertEq(governor.quorumNumerator(), 250);
        assertEq(governor.quorumDenominator(), 10000);
        assertEq(governor.timelock(), address(timelockController));
        assertEq(timelockController.getMinDelay(), 4 days);
        assertEq(
            governor.quorum(block.number - 1),
            (AMKT.totalSupply() * 250) / 10000
        );
        assertEq(governor.name(), "Alongside Governor");
        assertEq(governor.version(), "1");
        assertEq(governor.getVotes(largeAmktHolder, block.number - 1), 0);
    }

    function testTimelockConfig() public {
        assertEq(timelockController.getMinDelay(), 4 days);
        assertEq(
            timelockController.hasRole(
                timelockController.EXECUTOR_ROLE(),
                MULTISIG
            ),
            true
        );
        assertEq(
            timelockController.hasRole(
                timelockController.PROPOSER_ROLE(),
                MULTISIG
            ),
            true
        );
        assertEq(
            timelockController.hasRole(
                timelockController.CANCELLER_ROLE(),
                MULTISIG
            ),
            true
        );
        assertEq(
            timelockController.hasRole(
                timelockController.PROPOSER_ROLE(),
                address(governor)
            ),
            true
        );
    }

    function testProxyState() public {
        ITransparentUpgradeableProxy proxy = ITransparentUpgradeableProxy(
            AMKT_PROXY
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
            2275e17
        ); // random user with 200 balance
        assertEq(
            AMKT.balanceOf(address(0x5c90090405d0dFfe53F385925E7F0DA064C4CA05)),
            100e18
        ); // random user with 100 balance
        assertEq(AMKT.totalSupply(), 29261206678972195561284);
        assertEq(AMKT.getPastTotalSupply(block.number - 1), AMKT.totalSupply());
        assertEq(
            AMKT.getPastTotalSupply(batchExecutionBlock - 1),
            AMKT.totalSupply()
        );
        assertEq(AMKT.getPastTotalSupply(batchExecutionBlock - 2), 0);
        assertEq(AMKT.numCheckpoints(largeAmktHolder), 0);
    }

    function testVaultState() public {
        assertEq(vault.underlyingLength(), 15);
        assertEq(vault.issuance(), address(issuance));
        assertEq(vault.rebalancer(), address(invokeableBounty));
        assertEq(vault.feeRecipient(), FEE_RECEIPIENT);
        assertEq(vault.emergencyResponder(), MULTISIG);
        assertEq(vault.emergency(), false);
        assertEq(address(vault.indexToken()), address(AMKT));
        assertEq(vault.inflationRate(), INFLATION_RATE);
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
        assertEq(activeBounty.authority(), MULTISIG);
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

    function testExpectedVaultBalances() public {
        // for all tokens in vault, vault balance should match NAV
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();
        address[] memory underlying = vault.underlying();

        for (uint256 i = 0; i < vault.underlyingLength(); i++) {
            IERC20 token = IERC20(underlying[i]);
            assertEq(
                token.balanceOf(address(vault)),
                fmul(tokens[i].units + 1, AMKT.totalSupply()) + 1
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
