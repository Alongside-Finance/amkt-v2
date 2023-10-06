pragma solidity =0.8.18;

import {IActiveBounty} from "src/interfaces/IActiveBounty.sol";

contract ActiveBounty is IActiveBounty {
    address public immutable authority;

    bytes32 public activeBounty;

    constructor(address _authority) {
        if (_authority == address(0)) revert ActiveBountyZeroCheck();
        authority = _authority;
    }

    /// @notice Set the active bounty hash
    /// @param bountyHash The hash of the active bounty
    /// @dev Only callable by the authority
    function setHash(bytes32 bountyHash) external {
        if (msg.sender != authority) revert ActiveBountyAuth();
        activeBounty = bountyHash;
        emit ActiveBountyHashSet(bountyHash);
    }
}
