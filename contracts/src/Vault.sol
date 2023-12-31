// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {VerifiableAddressArray} from "src/lib/VArray.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {TokenInfo} from "src/Common.sol";
import {IIndexToken} from "src/interfaces/IIndexToken.sol";
import {SCALAR, fdiv, fmul, finv} from "./lib/FixedPoint.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";

contract Vault is Ownable2Step, IVault {
    using VerifiableAddressArray for VerifiableAddressArray.VerifiableArray;
    using SafeERC20 for IERC20;

    IIndexToken public immutable indexToken;

    bool public emergency;

    address public issuance;
    address public rebalancer;
    address public feeRecipient;
    address public emergencyResponder;

    uint256 public inflationRate;

    uint256 public lastKnownTimestamp;

    VerifiableAddressArray.VerifiableArray internal _underlying;
    mapping(address => uint256) internal _nominals;

    ///////////////////////// MODIFIERS / CONSTRUCTOR /////////////////////////

    modifier onlyInvokers() {
        if (msg.sender != issuance && msg.sender != rebalancer) {
            revert AMKTVaultOnlyInvokers();
        }
        _;
    }

    modifier only(address who) {
        if (msg.sender != who) {
            revert AMKTVaultOnly(who);
        }
        _;
    }

    modifier whenNotEmergency() {
        if (emergency) {
            revert AMKTVaultEmergency();
        }
        _;
    }

    /// @notice Constructor
    /// @param _indexToken The index token address
    /// @param _owner The owner of the vault
    /// @param _feeRecipient The recipient of the fee
    /// @param _emergencyResponder The emergency responder, can set emergency flag
    /// @param _inflationRate The per second inflation rate
    constructor(
        IIndexToken _indexToken,
        address _owner,
        address _feeRecipient,
        address _emergencyResponder,
        uint256 _inflationRate
    ) {
        if (_owner == address(0)) revert VaultZeroCheck();
        if (_feeRecipient == address(0)) revert VaultZeroCheck();
        if (_emergencyResponder == address(0)) revert VaultZeroCheck();
        if (_inflationRate > SCALAR) {
            revert AMKTVaultInflationRateTooLarge();
        }

        indexToken = _indexToken;

        _transferOwnership(_owner);

        emergencyResponder = _emergencyResponder;

        feeRecipient = _feeRecipient;
        inflationRate = _inflationRate;

        lastKnownTimestamp = block.timestamp;
    }

    ///////////////////////// CONFIG /////////////////////////

    /// @notice Set the issuance module for the vault
    /// @param _issuance The issuance module address
    /// @dev only owner
    function setIssuance(address _issuance) external only(owner()) {
        if (_issuance == address(0)) revert VaultZeroCheck();
        issuance = _issuance;
        emit VaultIssuanceSet(_issuance);
    }

    /// @notice Set the rebalancer module for the vault
    /// @param _rebalancer The rebalancer module address
    /// @dev only owner
    function setRebalancer(address _rebalancer) external only(owner()) {
        if (_rebalancer == address(0)) revert VaultZeroCheck();
        rebalancer = _rebalancer;
        emit VaultRebalancerSet(_rebalancer);
    }

    /// @notice Set the fee recipient
    /// @param _feeRecipient The fee recipient address
    /// @dev only owner
    function setFeeRecipient(address _feeRecipient) external only(owner()) {
        if (_feeRecipient == address(0)) revert VaultZeroCheck();
        feeRecipient = _feeRecipient;
        emit VaultFeeRecipientSet(_feeRecipient);
    }

    /// @notice Set the emergency responder
    /// @param _emergencyResponder The emergency responder address
    /// @dev only owner
    function setEmergencyResponder(
        address _emergencyResponder
    ) external only(owner()) {
        if (_emergencyResponder == address(0)) revert VaultZeroCheck();
        emergencyResponder = _emergencyResponder;
        emit VaultEmergencyResponderSet(_emergencyResponder);
    }

    /// @notice Set the inflation rate
    /// @param _inflationRate The per second inflation rate
    /// @dev only owner & accrues inflation
    function setInflationRate(uint256 _inflationRate) external only(owner()) {
        if (_inflationRate > SCALAR) {
            revert AMKTVaultInflationRateTooLarge();
        }
        inflationRate = _inflationRate;
        emit VaultInflationRateSet(_inflationRate);
    }

    /// @notice Set the emergency flag
    /// @param _emergency The emergency flag
    /// @dev only emergency responder
    function setEmergency(bool _emergency) external only(emergencyResponder) {
        emergency = _emergency;
        emit VaultEmergencySet(_emergency);
    }

    ///////////////////////// INFLATION /////////////////////////

    /// @notice Try to accrue inflation
    function tryInflation() external only(feeRecipient) {
        if (block.timestamp < lastKnownTimestamp + 1 days)
            revert AMKTVaultFeeTooEarly();
        uint256 startingSupply = indexToken.totalSupply();
        uint256 timestampDiff = block.timestamp - lastKnownTimestamp;
        uint256 inflation = fmul(startingSupply, timestampDiff * inflationRate);
        if (inflation == 0) revert AMKTVaultFeeTooSmall();

        lastKnownTimestamp = block.timestamp;

        uint256 valueMultiplier = fdiv(
            startingSupply,
            startingSupply + inflation
        );

        TokenInfo[] memory tokens = virtualUnits();
        for (uint256 i = 0; i < tokens.length; i++) {
            _setNominal(
                SetNominalArgs({
                    token: tokens[i].token,
                    virtualUnits: fmul(tokens[i].units, valueMultiplier)
                })
            );
        }

        indexToken.mint(feeRecipient, inflation);
        invariantCheck();
        emit VaultFeeMinted(feeRecipient, inflation);
    }

    ///////////////////////// INVOKE /////////////////////////

    /// @notice Set the nominal units of more than one token
    /// @param args The SetNominalArgs[]
    /// @dev only rebalancer
    function invokeSetNominals(
        SetNominalArgs[] calldata args
    ) external whenNotEmergency only(rebalancer) {
        for (uint256 i; i < args.length; i++) {
            _setNominal(args[i]);
        }
    }

    /// @notice Invoke ERC20 transfers
    /// @param args The InvokeERC20Args[]
    /// @dev only invokers
    function invokeERC20s(
        InvokeERC20Args[] calldata args
    ) external onlyInvokers {
        for (uint256 i; i < args.length; i++) {
            _transferERC20(args[i]);
        }
    }

    /// @notice Mint index tokens
    /// @param to The recipient of the index tokens
    /// @param amount The amount of index tokens to mint
    /// @dev only issuance
    function invokeMint(
        address to,
        uint256 amount
    ) external whenNotEmergency only(issuance) {
        indexToken.mint(to, amount);
    }

    /// @notice Burn index tokens
    /// @param from The owner of the index tokens
    /// @param amount The amount of index tokens to burn
    /// @dev only issuance
    function invokeBurn(address from, uint256 amount) external only(issuance) {
        indexToken.burn(from, amount);
    }

    ///////////////////////// VIEW ////////////////////////

    /// @notice Returns the underlying tokens
    /// @return address[] memory of underlying tokens with nominal units > 0
    function underlying() external view returns (address[] memory) {
        return _underlying.toMemoryArray();
    }

    /// @notice Returns the underlying tokens and their nominal units
    /// @return the number of tokens backing the index
    function underlyingLength() external view returns (uint256) {
        return _underlying.size();
    }

    /// @notice Returns true if the token is an underlying
    /// @param _token address
    /// @return bool true if underlying
    function isUnderlying(address _token) public view returns (bool) {
        return _underlying.includes(_token);
    }

    /// @notice Returns the virtual units of a token
    /// @dev warning! does not revert on non-underlying token
    /// @param token address
    /// @return uint256
    function virtualUnits(address token) public view returns (uint256) {
        return _nominals[token];
    }

    /// @notice Returns the virtual units of all tokens
    /// @return TokenInfo[] memory
    function virtualUnits() public view returns (TokenInfo[] memory) {
        address[] storage stor = _underlying.toStorageArray();
        uint256 len = stor.length;

        TokenInfo[] memory info = new TokenInfo[](len);

        for (uint256 i; i < len; i++) {
            address token = stor[i];
            info[i] = TokenInfo({token: token, units: _nominals[token]});
        }

        return info;
    }

    /// @notice Checks that the vault is in a valid state
    /// @notice i.e. we can wind down to 0 safely
    /// @notice reverts if the check fails
    function invariantCheck() public view {
        TokenInfo[] memory tokens = virtualUnits();

        for (uint256 i; i < tokens.length; ) {
            uint256 expectedAmount = fmul(
                tokens[i].units,
                indexToken.totalSupply()
            );
            if (
                IERC20(tokens[i].token).balanceOf(address(this)) <
                expectedAmount
            ) revert VaultInvariant();

            unchecked {
                ++i;
            }
        }
    }

    ///////////////////////// INTERNAL /////////////////////////

    function _setNominal(SetNominalArgs memory args) internal {
        address token = args.token;
        uint256 _virtualUnits = args.virtualUnits;

        if (_virtualUnits == 0) {
            delete _nominals[token];
            _underlying.remove(token);
            return;
        }

        if (!isUnderlying(token)) {
            _underlying.add(token);
        }

        _nominals[token] = _virtualUnits;
    }

    function _transferERC20(InvokeERC20Args memory args) internal {
        IERC20(args.token).safeTransfer(args.to, args.amount);
    }
}
