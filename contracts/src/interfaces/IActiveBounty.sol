// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

interface IActiveBounty {
    error ActiveBountyAuth();
    error ActiveBountyZeroCheck();
    event ActiveBountyHashSet(bytes32 bountyHash);

    function setHash(bytes32 bountyHash) external;

    function authority() external view returns (address);

    function activeBounty() external view returns (bytes32);
}
