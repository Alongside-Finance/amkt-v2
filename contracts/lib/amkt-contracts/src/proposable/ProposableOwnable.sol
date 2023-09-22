pragma solidity 0.8.7;

import "@openzeppelin/contracts/access/Ownable.sol";

/// @title ProposableOwnable
/// @author Alongside Finance
/// @notice OpenZeppelin's Ownable with propose/accept
abstract contract ProposableOwnable is Ownable {
    address public proposedOwner;

    function transferOwnership(address newOwner) public virtual override {
        require(newOwner != address(0), "ProposableOwnable: new owner is the zero address");
        require(newOwner == proposedOwner, "ProposableOwnable: new owner is not proposed owner");
        require(newOwner == msg.sender, "ProposableOwnable: this call must be made by the new owner");
        _transferOwnership(newOwner);
    }

    function proposeOwner(address _proposedOwner) public virtual onlyOwner {
        require(_proposedOwner != address(0), "ProposableOwnable: new owner is the zero address");
        proposedOwner = _proposedOwner;
    }
}
