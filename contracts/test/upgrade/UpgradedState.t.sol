pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {InitialBountyHelper} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {Dealer} from "test/Dealer.t.sol";
import {UpgradeTest} from "./Upgrade.t.sol";

contract UpgradedState is UpgradeTest {
    address largeAmktHolder =
        address(0x804B68f60765F4559b7096B158C912eD33aa0c26);
    address oldMinter = address(0x0D44F856E1a7c70E35c54261c3f07DbFBDCA4857);

    function testState() public {
        assertEq(AMKT.minter(), address(vault));
        assertEq(AMKT.decimals(), 18);
        assertEq(AMKT.symbol(), "AMKT");
        assertEq(AMKT.name(), "Alongside Crypto Market Index");
        assertEq(vault.underlyingLength(), 15); // TODO: Increase to 15
    }

    function testVaultCanMint() public {
        vm.startPrank(address(vault));
        AMKT.mint(address(vault), 1);
        AMKT.burn(address(vault), 1);
    }

    function testUserCanTransfer() public {
        vm.startPrank(largeAmktHolder);
        AMKT.transfer(address(vault), 1);
    }

    function testRevertOldMinterCanMint() public {
        vm.prank(oldMinter);
        vm.expectRevert();
        AMKT.mint(address(vault), 1);
    }

    function testRevertOldMinterCanBurn() public {
        vm.prank(oldMinter);
        vm.expectRevert();
        AMKT.burn(largeAmktHolder, 1);
    }

    function testIssuance() public {
        Dealer dealer = new Dealer();
        TokenInfo[] memory tokens = (new InitialBountyHelper()).tokens();

        uint256 initialDealtAmount = 5;
        uint256 issuedAmount = 5;

        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            uint256 initialBalance = token.balanceOf(address(this));
            dealer.dealToken(
                address(token),
                address(this),
                (initialDealtAmount * tokens[i].units) / 1e18 + 1
            );
            token.approve(address(issuance), token.balanceOf(address(this)));
        }

        issuance.issue(issuedAmount);

        // Check the balances of address(this) after issuance
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20 token = IERC20(tokens[i].token);
            assertEq(
                token.balanceOf(address(this)),
                initialDealtAmount - issuedAmount
            );
            assertEq(token.balanceOf(address(this)), 0);
        }

        // TODO: Check vault banaces
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
