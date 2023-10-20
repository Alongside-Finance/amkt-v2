pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {Vault} from "src/Vault.sol";
import {Issuance} from "src/invoke/Issuance.sol";
import {InvokeableBounty} from "src/invoke/Bounty.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {IRebalancer} from "src/interfaces/IRebalancer.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {BaseTest} from "test/utils/BaseTest.t.sol";
import {IIndexToken} from "src/interfaces/IIndexToken.sol";
import {INFLATION_RATE} from "src/scripts/Config.sol";
import {Quoter} from "periphery/Quoter.sol";

contract StatefulTest is BaseTest, IRebalancer {
    Vault vault;
    InvokeableBounty bounty;
    Issuance issuance;
    ActiveBounty activeBounty;
    Quoter quoter;

    MockMintableToken indexToken;

    address constant authority = address(bytes20(keccak256("authority")));

    address constant feeReciever = address(bytes20(keccak256("feeReciever")));

    address constant emergencyResponder =
        address(bytes20(keccak256("emergencyResponder")));

    function setUp() public virtual {
        indexToken = new MockMintableToken("Index", "INDEX", 18, 1e18);

        activeBounty = new ActiveBounty(authority);

        vault = new Vault(
            IIndexToken(address(indexToken)),
            address(this),
            feeReciever,
            emergencyResponder,
            INFLATION_RATE
        );

        issuance = new Issuance(address(vault));

        quoter = new Quoter(address(vault));

        bounty = new InvokeableBounty(
            address(vault),
            address(activeBounty),
            0,
            1
        );

        vault.setIssuance(address(issuance));
        vault.setRebalancer(address(bounty));
    }

    function seedInitial(
        uint256 quantity
    ) internal returns (TokenInfo[] memory tokens) {
        quantity = bound(quantity, 0, 99);
        if (quantity == 0) {
            return tokens;
        }
        vm.assume(quantity < 98 && quantity > 0);
        tokens = Mocks.ascendingTokenNominalsMock(address(bounty), quantity);

        fulfillBounty(Mocks.bountyMock(tokens));
    }

    function fulfillBounty(Bounty memory _bounty) internal {
        validateBounty(_bounty);
        bounty.fulfillBounty(_bounty, true);
    }

    function validateBounty(Bounty memory _bounty) internal {
        bytes32 _hash = bounty.hashBounty(_bounty);
        vm.prank(authority);
        activeBounty.setHash(_hash);
    }

    function mint(uint256 amount) internal {
        TokenInfo[] memory tokens = quoter.quoteIssue(amount);
        for (uint256 i = 0; i < tokens.length; i++) {
            MockMintableToken(tokens[i].token).mint(
                address(this),
                tokens[i].units
            );
            IERC20(tokens[i].token).approve(address(issuance), tokens[i].units);
        }

        issuance.issue(amount);
    }

    function burn(uint256 amount) internal {
        issuance.redeem(amount);
    }

    function rebalanceCallback(
        TokenInfo[] calldata x,
        TokenInfo[] calldata y
    ) external virtual {}
}

library Mocks {
    // deploys quantity of MockMintableTokens and mints 100 tokens to the calling address, approves i * 1e18 of the i-th token to the approve address
    // returns the tokens and their approved amounts
    function ascendingTokenNominalsMock(
        address approve,
        uint256 quantity
    ) internal returns (TokenInfo[] memory tokens) {
        require(quantity > 0 && quantity < 99, "invalid quantity");
        tokens = new TokenInfo[](quantity);
        for (uint256 i = 0; i < quantity; i++) {
            address addr = address(
                new MockMintableToken(
                    string(abi.encodePacked("token", i)),
                    string(abi.encodePacked("tkn", i)),
                    uint8(i) % 18,
                    100e18
                )
            );

            uint256 amount = (1 + i) * 1e18;

            IERC20(addr).approve(address(approve), amount + 2); // add 2 to account for fulfillBounty rounding

            tokens[i] = TokenInfo({token: addr, units: amount});
        }
    }

    function bountyMock(
        TokenInfo[] memory tokens
    ) internal view returns (Bounty memory) {
        return
            Bounty({
                infos: tokens,
                fulfiller: address(0),
                salt: keccak256(abi.encode(block.timestamp)),
                deadline: block.timestamp + 1000000000
            });
    }
}
