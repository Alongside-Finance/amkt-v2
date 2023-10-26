pragma solidity =0.8.18;

import {StatefulTest} from "test/core/State.t.sol";
import {TokenInfo} from "src/Common.sol";
import {MockMintableToken} from "test/utils/MockMintableToken.sol";
import {Bounty} from "src/interfaces/IInvokeableBounty.sol";
import {console} from "forge-std/console.sol";

contract BountyRoundingTest is StatefulTest {
    address fulfiller = address(bytes20(keccak256("fulfiller")));
    address user = address(bytes20(keccak256("user")));
    uint256 userMintAmount = 1e11;

    function testBug() public {
        // setup
        MockMintableToken USDC = new MockMintableToken("USDC", "USDC", 6, 0);
        MockMintableToken WETH = new MockMintableToken("WETH", "WETH", 18, 0);
        USDC.mint(fulfiller, 10000e6);
        WETH.mint(fulfiller, 10000e18);
        vm.startPrank(fulfiller);
        USDC.approve(address(bounty), 10000e6);
        WETH.approve(address(bounty), 10000e18);
        vm.stopPrank();
        USDC.mint(user, userMintAmount);
        // original bounty

        console.log(" ");
        console.log("initial state");
        console.log("Token supply: %s", indexToken.totalSupply());
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );

        TokenInfo[] memory originalTokens = new TokenInfo[](1);
        originalTokens[0] = TokenInfo(address(USDC), 1800e6);
        Bounty memory originalBounty = Bounty({
            infos: originalTokens,
            fulfiller: fulfiller,
            salt: keccak256("test"),
            deadline: block.timestamp + 100
        });
        bytes32 originalBountyHash = bounty.hashBounty(originalBounty);
        vm.prank(authority);
        activeBounty.setHash(originalBountyHash);
        vm.prank(fulfiller);
        bounty.fulfillBounty(originalBounty, false);

        console.log(" ");
        console.log("after original bounty fulfill");
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );

        // mint
        console.log(" ");
        console.log("before user mint");
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );
        vm.startPrank(user);
        USDC.approve(address(issuance), userMintAmount);
        issuance.issue(userMintAmount);
        vm.stopPrank();
        console.log(" ");
        console.log("after user mint");
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );
        // new bounty
        TokenInfo[] memory newTokens = new TokenInfo[](2);
        newTokens[0] = TokenInfo(address(USDC), 0);
        newTokens[1] = TokenInfo(address(WETH), 1e18);
        Bounty memory newBounty = Bounty({
            infos: newTokens,
            fulfiller: fulfiller,
            salt: keccak256("test"),
            deadline: block.timestamp + 100
        });
        bytes32 newBountyHash = bounty.hashBounty(newBounty);
        vm.prank(authority);
        activeBounty.setHash(newBountyHash);
        vm.prank(fulfiller);
        bounty.fulfillBounty(newBounty, false);

        console.log(" ");
        console.log("after new bounty fulfill");
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );

        // redeem
        console.log(" ");
        console.log("before user redeem");
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );
        vm.prank(user);
        issuance.redeem(userMintAmount);
        console.log(" ");
        console.log("after user redeem");
        console.log(
            "User: USDC %s WETH %s",
            USDC.balanceOf(user),
            WETH.balanceOf(user)
        );
        console.log(
            "Fulfiller: USDC %s WETH %s",
            USDC.balanceOf(fulfiller),
            WETH.balanceOf(fulfiller)
        );
        console.log(
            "Vault: USDC %s WETH %s",
            USDC.balanceOf(address(vault)),
            WETH.balanceOf(address(vault))
        );
    }
}
