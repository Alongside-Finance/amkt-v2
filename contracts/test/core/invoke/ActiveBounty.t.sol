pragma solidity =0.8.18;

import "forge-std/Test.sol";
import "src/invoke/ActiveBounty.sol";

contract ActiveBountyTest is Test {
    ActiveBounty private activeBounty;

    function setUp() public {
        activeBounty = new ActiveBounty(address(this));
    }

    function testInitialAuthority() public {
        assertEq(activeBounty.authority(), address(this));
    }

    function testInitialActiveBounty() public {
        assertEq(activeBounty.activeBounty(), bytes32(0));
    }

    function testSetHash() public {
        bytes32 newBountyHash = keccak256("New Bounty");
        activeBounty.setHash(newBountyHash);
        assertEq(activeBounty.activeBounty(), newBountyHash);
    }

    function testSetHashNotByAuthorityShouldRevert() public {
        ActiveBounty nonAuthorityBounty = new ActiveBounty(address(1));
        bytes32 newBountyHash = keccak256("New Bounty");
        vm.expectRevert();
        nonAuthorityBounty.setHash(newBountyHash);
    }
}
