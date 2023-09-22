// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "../proposable/ProposableOwnable.sol";
import "../utils/IndexedMapping.sol";
import "./MembersInterface.sol";

/// @title AMKT Members
/// @author Alongside Finance
/// @notice Stores addresses for the different ypes of roles
/// @dev Do not depend on ordering of addresses in collection returning functions
contract Members is MembersInterface, ProposableOwnable {
    ///=============================================================================================
    /// State Variables
    ///=============================================================================================

    using IndexedMapping for IndexedMapping.Data;

    address public issuer;

    IndexedMapping.Data internal merchants;

    ///=============================================================================================
    /// Constructor
    ///=============================================================================================

    constructor(address newOwner) {
        require(newOwner != address(0), "invalid newOwner address");
        proposeOwner(newOwner);
        transferOwnership(newOwner);
    }

    ///=============================================================================================
    /// Setters
    ///=============================================================================================

    /// @notice Allows the owner of the contract to set the issuer
    /// @param _issuer address
    /// @return bool
    function setIssuer(address _issuer) external override onlyOwner returns (bool) {
        require(_issuer != address(0), "invalid issuer address");
        issuer = _issuer;

        emit IssuerSet(_issuer);
        return true;
    }

    /// @notice Allows the owner of the contract to add a merchant
    /// @param merchant address
    /// @return bool
    function addMerchant(address merchant) external override onlyOwner returns (bool) {
        require(merchant != address(0), "invalid merchant address");
        require(merchants.add(merchant), "merchant add failed");

        emit MerchantAdd(merchant);
        return true;
    }

    /// @notice Allows the owner of the contract to remove a merchant
    /// @param merchant address
    /// @return bool
    function removeMerchant(address merchant) external override onlyOwner returns (bool) {
        require(merchant != address(0), "invalid merchant address");
        require(merchants.remove(merchant), "merchant remove failed");

        emit MerchantRemove(merchant);
        return true;
    }

    ///=============================================================================================
    /// Non Mutable
    ///=============================================================================================

    function isIssuer(address addr) external view override returns (bool) {
        return (addr == issuer);
    }

    function isMerchant(address addr) external view override returns (bool) {
        return merchants.exists(addr);
    }

    function getMerchant(uint256 index) external view returns (address) {
        return merchants.getValue(index);
    }

    function getMerchants() external view override returns (address[] memory) {
        return merchants.getValueList();
    }

    function merchantsLength() external view override returns (uint256) {
        return merchants.valueList.length;
    }
}
