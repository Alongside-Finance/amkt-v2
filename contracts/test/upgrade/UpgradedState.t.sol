pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {InitialBountyHelper, MULTISIG, FEE_RECEIPIENT, INFLATION_RATE, PROXY, PROXY_ADMIN} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {UpgradeTest} from "test/upgrade/helpers/Upgrade.t.sol";
import {ITransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {fmul} from "src/lib/FixedPoint.sol";

contract UpgradedStateTest is UpgradeTest {
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
                address(0)
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

    // function testDeployedContracts() public {
    //     assertEq(
    //         address(vault),
    //         address(0xD62A80368AdF5919f70193D15dCbD5C77EAf55ac)
    //     );
    //     assertEq(
    //         address(issuance),
    //         address(0x58AD9D36AfAc51206672f855Bf7e76037c5F5198)
    //     );
    //     assertEq(
    //         address(invokeableBounty),
    //         address(0x366A647DE921608bee3987025D23f12263da6884)
    //     );
    //     assertEq(
    //         address(activeBounty),
    //         address(0x12bc3CCaA2E213e9D50faB9752A9daFac01b962F)
    //     );
    //     assertEq(
    //         address(governor),
    //         address(payable(0x774045B30e6fC5DfE73bF386E8845CA1472fb45e))
    //     );
    //     assertEq(
    //         address(timelockController),
    //         address(payable(0xB3970Ae79fD2cD8f1060cF6BAeae27b8E2c05437))
    //     );
    //     assertEq(
    //         newTokenImplementation,
    //         address(0x775715D96cD3B3586728B7420A13Ec74f5dc9e8f)
    //     );

    //     assertEq(
    //         address(timelockActiveBounty),
    //         address(0x8D2A6bcB5713d4b57f2FffB119B7B6D0143e25ed)
    //     );
    //     assertEq(
    //         address(timelockInvokeableBounty),
    //         address(0x703814F9172D6E6EF10F89fCAdE3ff480d812a45)
    //     );
    // }

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
        assertEq(AMKT.totalSupply(), 29559270507524640614886);
        assertEq(AMKT.getPastTotalSupply(block.number - 1), AMKT.totalSupply());
        assertEq(AMKT.getPastTotalSupply(block.number - 2), 0);
        assertEq(AMKT.numCheckpoints(largeAmktHolder), 0);
    }

    function testVaultState() public {
        assertEq(vault.underlyingLength(), 15);
        assertEq(vault.issuance(), address(issuance));
        assertEq(vault.rebalancer(), address(timelockInvokeableBounty));
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
                fmul(tokens[i].units, AMKT.totalSupply())
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

    function testInitializeAssembly() public {
        address target = address(AMKT);
        bytes32 totalSupplyCheckpointsHash = keccak256(
            abi.encodePacked(uint256(206))
        );
        // block.number is casted to 32 bits by SafeCastUpgradeable.toUint32
        assertEq(
            vm.load(target, totalSupplyCheckpointsHash),
            (bytes32(block.number - 1)) | (bytes32(AMKT.totalSupply()) << 32)
        );
        // check using encodePacked too
        assertEq(
            vm.load(target, totalSupplyCheckpointsHash),
            bytes32(
                abi.encodePacked(
                    uint224(AMKT.totalSupply()),
                    uint32(block.number - 1)
                )
            )
        );
        // check 5 more indices to make sure they are all zero
        for (uint256 i = 0; i < 5; i++) {
            assertEq(
                vm.load(
                    target,
                    bytes32(uint256(totalSupplyCheckpointsHash) + i + 1)
                ),
                bytes32(uint256(0))
            );
        }
    }

    // based on `forge inspect IndexToken storage`
    // known issues:
    // - name and symbol storage assertion fails
    // - collision in gap
    function testStorageLayout() public {
        address target = address(AMKT);
        // unstructured slot
        assertEq(
            vm.load(target, keccak256("Alongside::Token::MinterSlot")),
            bytes32(uint256(uint160(AMKT.minter())))
        );

        //// SLOT 0
        // uint8 __initialized
        // bool _initializing
        assertEq(vm.load(target, _b(0)), _b(1)); // TODO: understand the offset

        //// SLOT 1-50
        // uint256[50] __gap

        //// SLOT 51
        // mapping(address => uint256) _balances
        assertEq(
            vm.load(
                target,
                _derive(
                    address(0x209ADBAad63c3008B5C2edb941B991Ef9Bb35027),
                    uint256(51)
                )
            ),
            _b(200e18)
        );

        assertEq(
            vm.load(
                target,
                _derive(
                    address(0x5c90090405d0dFfe53F385925E7F0DA064C4CA05),
                    uint256(51)
                )
            ),
            _b(100e18)
        );

        //// SLOT 52
        // mapping(address => mapping(address => uint256)) _allowances
        // TODO

        //// SLOT 53
        // uint256 _totalSupply
        assertEq(vm.load(target, _b(53)), _b(AMKT.totalSupply()));

        //// SLOT 54
        // string _name
        assertEq(vm.load(target, _b(54)), _stringToBytes32(AMKT.name()));

        //// SLOT 55
        // string _symbol
        assertEq(vm.load(target, _b(55)), _stringToBytes32(AMKT.symbol()));

        //// SLOT 56-100
        // uint256[50] __gap

        //// SLOT 101
        // bytes32 _HASHED_NAME
        assertEq(
            vm.load(target, bytes32(uint256(101))),
            keccak256(bytes(AMKT.name()))
        );

        //// SLOT 102
        // bytes32 _HASHED_VERSION
        assertEq(vm.load(target, bytes32(uint256(102))), keccak256(bytes("2")));

        //// SLOT 103-152
        // uint256[50] __gap

        //// SLOT 153
        // mapping(address => struct CountersUpgradeable.Counter) _nonces
        // TODO

        //// SLOT 154
        // bytes32 _PERMIT_TYPEHASH_DEPRECATED_SLOT
        assertEq(vm.load(target, _b(154)), _b(0));

        //// SLOT 155
        // uint256[49] __gap
        for (uint256 i = 155; i < 204; i++) {
            assertEq(vm.load(target, _b(i)), _b(0));
        }

        //// SLOT 204
        // mapping(address => address) _delegates
        // TODO: FUZZ THIS
        assertEq(
            vm.load(
                target,
                _derive(
                    address(0x209ADBAad63c3008B5C2edb941B991Ef9Bb35027),
                    204
                )
            ),
            _b(0)
        );
        assertEq(
            vm.load(
                target,
                _derive(
                    address(0x5c90090405d0dFfe53F385925E7F0DA064C4CA05),
                    204
                )
            ),
            _b(0)
        );

        //// SLOT 205
        // mapping(address => struct ERC20VotesUpgradeable.Checkpoint[]) _checkpoints
        // TODO

        //// SLOT 206
        // struct ERC20VotesUpgradeable.Checkpoint[] _totalSupplyCheckpoints
        bytes32 totalSupplyCheckpointsHash = keccak256(
            abi.encodePacked(uint256(206))
        );
        // block.number is casted to 32 bits by SafeCastUpgradeable.toUint32
        assertEq(
            vm.load(target, totalSupplyCheckpointsHash),
            (bytes32(block.number - 1)) | (bytes32(AMKT.totalSupply()) << 32)
        );
        // check using encodePacked too
        assertEq(
            vm.load(target, totalSupplyCheckpointsHash),
            bytes32(
                abi.encodePacked(
                    uint224(AMKT.totalSupply()),
                    uint32(block.number - 1)
                )
            )
        );
        // check 5 more indices to make sure they are all zero
        for (uint256 i = 0; i < 5; i++) {
            assertEq(
                vm.load(
                    target,
                    bytes32(uint256(totalSupplyCheckpointsHash) + i + 1)
                ),
                bytes32(uint256(0))
            );
        }

        //// SLOT 207 - 253
        // uint256[47] __gap
    }

    function _b(uint256 x) internal pure returns (bytes32) {
        return bytes32(uint256(x));
    }

    function _derive(
        address key,
        uint256 slot
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(key, slot));
    }

    function _stringToBytes32(
        string memory source
    ) public returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        uint256 length = tempEmptyStringTest.length;
        assertLt(length, 32); // this fn only works for strings less than length 32
        if (length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
        // Set the last byte as the length of the string * 2
        result |= bytes32(length * 2);
    }
}
