// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

interface MembersInterface {
    ///=============================================================================================
    /// Events
    ///=============================================================================================

    event IssuerSet(address indexed issuer);
    event MerchantAdd(address indexed merchant);
    event MerchantRemove(address indexed merchant);

    function setIssuer(address _issuer) external returns (bool);

    function addMerchant(address merchant) external returns (bool);

    function removeMerchant(address merchant) external returns (bool);

    function isIssuer(address addr) external view returns (bool);

    function isMerchant(address addr) external view returns (bool);

    function getMerchants() external view returns (address[] memory);

    function merchantsLength() external view returns (uint256);
}
