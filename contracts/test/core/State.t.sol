pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {Vault} from "src/Vault.sol";
import {Issuance} from "src/invoke/Issuance.sol";
import {InvokeableBounty, Bounty, Rebalancer} from "src/invoke/Bounty.sol";
import {ActiveBounty} from "src/invoke/ActiveBounty.sol";
import {MockMintableToken} from "../mocks/MockMintableToken.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {BaseTest} from "../BaseTest.t.sol";
import {IIndexToken} from "src/interfaces/IIndexToken.sol";
import {FEE_SCALED} from "src/scripts/Config.sol";
import {IssuanceQuoter} from "periphery/IssuanceQuoter.sol";

contract StatefulTest is BaseTest, Rebalancer {
    Vault vault;
    InvokeableBounty bounty;
    Issuance issuance;
    ActiveBounty activeBounty;
    IssuanceQuoter issuanceQuoter;

    MockMintableToken indexToken;

    address constant authority = address(bytes20(keccak256("authority")));

    address constant feeReciever = address(bytes20(keccak256("feeReciever")));

    address constant emergencyResponder =
        address(bytes20(keccak256("emergencyResponder")));

    function setUp() public {
        indexToken = new MockMintableToken("Index", "INDEX", 18, 1e18);

        activeBounty = new ActiveBounty(authority);

        vault = new Vault(
            IIndexToken(address(indexToken)),
            address(this),
            feeReciever,
            emergencyResponder,
            FEE_SCALED
        );

        issuance = new Issuance(address(vault));

        issuanceQuoter = new IssuanceQuoter(address(vault));

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
        TokenInfo[] memory tokens = issuanceQuoter.quoteIssue(amount);
        for (uint256 i = 0; i < tokens.length; i++) {
            IERC20(tokens[i].token).approve(address(issuance), tokens[i].units);
        }

        issuance.issue(amount);
    }

    function burn(uint256 amount) internal {
        indexToken.approve(address(issuance), amount);
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

            IERC20(addr).approve(address(approve), amount);

            tokens[i] = TokenInfo({token: addr, units: amount});
        }
    }

    function bountyMock(
        TokenInfo[] memory tokens
    ) internal view returns (Bounty memory) {
        return
            Bounty({
                infos: tokens,
                salt: keccak256(abi.encode(block.timestamp)),
                deadline: block.timestamp + 1000000000
            });
    }
}
