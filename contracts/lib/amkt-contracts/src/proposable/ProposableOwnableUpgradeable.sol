// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

/// @title ProposableOwnableUpgradeable
/// @author Alongside Finance
/// @notice OpenZeppelin's OwnableUpgradeable with propose/accept
/// @dev This contract uses an upgradeable pattern
abstract contract ProposableOwnableUpgradeable is OwnableUpgradeable {
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

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}
