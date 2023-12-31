pragma solidity =0.8.18;

import "forge-std/Test.sol";
import "src/IndexToken.sol";

contract IndexTokenTest is Test {
    IndexToken private indexToken;

    function setUp() public {
        indexToken = new IndexToken();
    }

    function testMinterSlot() public {
        assertEq(
            keccak256("Alongside::Token::MinterSlot"),
            0x1af730152eea9813c49583a406e8dd55a4df08cae9e33ae45721374fdde82bae
        );
    }

    function testInitialMinter() public {
        indexToken.initialize(address(this));
        assertEq(indexToken.minter(), address(this));
    }

    function testInitializeAgainRevert() public {
        indexToken.initialize(address(this));
        vm.expectRevert("IndexToken: already initialized");
        indexToken.initialize(address(1));
    }

    function testMint() public {
        indexToken.initialize(address(this));
        uint256 initialSupply = indexToken.totalSupply();
        uint256 mintAmount = 1000 ether;
        indexToken.mint(address(this), mintAmount);
        assertEq(indexToken.totalSupply(), initialSupply + mintAmount);
        assertEq(
            indexToken.balanceOf(address(this)),
            initialSupply + mintAmount
        );
    }

    function testMintNotByMinterRevert(address randomAddress) public {
        indexToken.initialize(address(this));
        vm.assume(
            randomAddress != indexToken.minter() &&
                randomAddress != address(DEFAULT_TEST_CONTRACT)
        );
        vm.expectRevert();
        IndexToken(randomAddress).mint(address(this), 1000 ether);
    }

    function testBurn() public {
        indexToken.initialize(address(this));
        indexToken.mint(address(this), 1000 ether);
        uint256 initialSupply = indexToken.totalSupply();
        uint256 burnAmount = 500 ether;
        indexToken.burn(address(this), burnAmount);
        assertEq(indexToken.totalSupply(), initialSupply - burnAmount);
        assertEq(
            indexToken.balanceOf(address(this)),
            initialSupply - burnAmount
        );
    }

    function testTransfer() public {
        indexToken.initialize(address(this));
        address recipient = address(1);
        uint256 transferAmount = 100 ether;
        indexToken.mint(address(this), transferAmount);
        indexToken.transfer(recipient, transferAmount);
        assertEq(indexToken.balanceOf(recipient), transferAmount);
        assertEq(
            indexToken.balanceOf(address(this)),
            indexToken.totalSupply() - transferAmount
        );
    }

    function testTransferFrom() public {
        indexToken.initialize(address(this));
        address sender = address(this);
        address recipient = address(2);
        uint256 transferAmount = 200 ether;
        uint256 allowanceAmount = 300 ether;
        indexToken.mint(sender, transferAmount);
        vm.prank(sender);
        indexToken.approve(address(this), allowanceAmount);
        indexToken.transferFrom(sender, recipient, transferAmount);
        assertEq(indexToken.balanceOf(recipient), transferAmount);
        assertEq(
            indexToken.balanceOf(sender),
            indexToken.totalSupply() - transferAmount
        );
        assertEq(
            indexToken.allowance(sender, address(this)),
            allowanceAmount - transferAmount
        );
    }
}
