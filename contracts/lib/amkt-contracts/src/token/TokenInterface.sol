// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

interface TokenInterface {
    event FeeReceiverSet(address indexed feeReceiver);
    event FeeRateSet(uint256 indexed feeRatePerDayScaled);
    event MethodologistSet(address indexed methodologist);
    event MethodologySet(string methodology);
    event MinterSet(address indexed minter);
    event SupplyCeilingSet(uint256 supplyCeiling);
    event MintFeeToReceiver(address feeReceiver, uint256 timestamp, uint256 totalSupply, uint256 amount);
    event ToggledRestricted(address indexed account, bool isRestricted);

    ///=============================================================================================
    /// Initializer
    ///=============================================================================================

    function initialize(
        string memory tokenName,
        string memory tokenSymbol,
        uint256 _feeRatePerDayScaled,
        address _feeReceiver,
        uint256 _supplyCeiling
    ) external virtual;

    ///=============================================================================================
    /// Mint Logic
    ///=============================================================================================

    /// @notice External mint function
    /// @dev Mint function can only be called externally by the controller
    /// @param to address
    /// @param amount uint256
    function mint(address to, uint256 amount) external virtual;

    /// @notice External burn function
    /// @dev burn function can only be called externally by the controller
    /// @param from address
    /// @param amount uint256
    function burn(address from, uint256 amount) external virtual;

    /// @notice Expands supply and mints fees to fee reciever
    /// @dev Can only be called by the owner externally,
    /// @dev _mintToFeeReciver is the internal function and is called after each supply/rate change
    function mintToFeeReceiver() external virtual;

    ///=============================================================================================
    /// Setters
    ///=============================================================================================

    /// @notice Only owner function for setting the methodologist
    /// @param _methodologist address
    function setMethodologist(address _methodologist) external virtual;

    /// @notice Callable only by the methodoligst to store on chain data about the underlying weight of the token
    /// @param _methodology string
    function setMethodology(string memory _methodology) external virtual;

    /// @notice Ownable function to set the fee rate
    /// @dev Given the annual fee rate this function sets and calculates the rate per second
    /// @param _feeRatePerDayScaled uint256
    function setFeeRate(uint256 _feeRatePerDayScaled) external virtual;

    /// @notice Ownable function to set the receiver
    /// @param _feeReceiver address
    function setFeeReceiver(address _feeReceiver) external virtual;

    /// @notice Ownable function to set the contract that controls minting
    /// @param _minter address
    function setMinter(address _minter) external virtual;

    /// @notice Ownable function to set the limit at which the total supply cannot exceed
    /// @param _supplyCeiling uint256
    function setSupplyCeiling(uint256 _supplyCeiling) external virtual;

    ///=============================================================================================
    /// Pausable Logic
    ///=============================================================================================

    function pause() external virtual;

    function unpause() external virtual;

    ///=============================================================================================
    /// Restrict
    ///=============================================================================================

    /// @notice Compliance feature to blacklist bad actors
    /// @dev Negates current restriction state
    /// @param who address
    function toggleRestriction(address who) external virtual;
}
