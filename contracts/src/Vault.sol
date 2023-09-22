pragma solidity =0.8.15;

import {VerifiableAddressArray} from "./lib/VArray.sol";
import {IVault} from "./interfaces/IVault.sol";
import {Multiplier} from "./lib/Multiplier.sol";
import {TokenInfo} from "./Common.sol";
import {IIndexToken} from "./interfaces/IIndexToken.sol";
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

    uint256 public feeScaled;

    // exposed in multiplier()
    uint256 internal lastKnownTimestamp;
    uint256 internal lastKnownMultiplier;

    VerifiableAddressArray.VerifiableArray internal _underlying;
    mapping(address => uint256) internal nominals;

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
    /// @param _feeScaled The *daily* fee scaled by 1e18
    constructor(
        IIndexToken _indexToken,
        address _owner,
        address _feeRecipient,
        address _emergencyResponder,
        uint256 _feeScaled
    ) {
        if (_feeScaled > SCALAR) {
            revert AMKTVaultFeeTooLarge();
        }

        indexToken = _indexToken;

        _transferOwnership(_owner);

        emergencyResponder = _emergencyResponder;

        feeRecipient = _feeRecipient;
        feeScaled = _feeScaled;

        lastKnownMultiplier = SCALAR;
        lastKnownTimestamp = block.timestamp;
    }

    ///////////////////////// OWNER /////////////////////////

    function setIssuance(address _issuance) external only(owner()) {
        issuance = _issuance;
        emit VaultIssuanceSet(_issuance);
    }

    function setRebalancer(address _rebalancer) external only(owner()) {
        rebalancer = _rebalancer;
        emit VaultRebalancerSet(_rebalancer);
    }

    function setFeeRecipient(address _feeRecipient) external only(owner()) {
        feeRecipient = _feeRecipient;
        emit VaultFeeRecipientSet(_feeRecipient);
    }

    function setEmergencyResponder(
        address _emergencyResponder
    ) external only(owner()) {
        emergencyResponder = _emergencyResponder;
        emit VaultEmergencyResponderSet(_emergencyResponder);
    }

    function setFeeScaled(uint256 _feeScaled) external only(owner()) {
        if (_feeScaled > SCALAR) {
            revert AMKTVaultFeeTooLarge();
        }

        tryInflation();
        feeScaled = _feeScaled;
        emit VaultFeeScaledSet(_feeScaled);
    }

    function setEmergency(bool _emergency) external only(emergencyResponder) {
        emergency = _emergency;
        emit VaultEmergencySet(_emergency);
    }

    ///////////////////////// INFLATION /////////////////////////

    /// Can be called by anyone. Only accrues inflation to the feeRecipient
    function tryInflation() public returns (uint256) {
        uint256 startingSupply = indexToken.totalSupply();

        (
            uint256 timestamp,
            uint256 trackedMultiplier,
            uint256 newFeeAccrued,
            uint256 currentMultiplier
        ) = multiplier();

        if (newFeeAccrued < SCALAR) {
            uint256 inflation = fmul(startingSupply, finv(newFeeAccrued)) -
                startingSupply;

            lastKnownMultiplier = trackedMultiplier;
            lastKnownTimestamp = timestamp;

            if (inflation > 0) {
                indexToken.mint(feeRecipient, inflation);
                emit VaultFeeMinted(feeRecipient, inflation);
            }
        }

        return currentMultiplier;
    }

    ///////////////////////// REBALANCER /////////////////////////

    function invokeSetNominals(
        SetNominalArgs[] calldata args
    ) external whenNotEmergency only(rebalancer) {
        for (uint256 i; i < args.length; i++) {
            _setNominal(args[i]);
        }
    }

    function invokeSetNominal(
        SetNominalArgs calldata args
    ) external whenNotEmergency only(rebalancer) {
        _setNominal(args);
    }

    function invokeSetMultiplier(
        uint256 _multiplier
    ) external whenNotEmergency only(rebalancer) {
        lastKnownMultiplier = _multiplier;
    }

    ///////////////////////// ISSUANCE /////////////////////////

    function invokeMint(
        address to,
        uint256 amount
    ) external whenNotEmergency only(issuance) {
        indexToken.mint(to, amount);
    }

    function invokeBurn(address from, uint256 amount) external only(issuance) {
        indexToken.burn(from, amount);
    }

    ///////////////////////// INVOKERS /////////////////////////

    function invokeERC20s(
        InvokeERC20Args[] calldata args
    ) external onlyInvokers {
        uint256 len = args.length;
        for (uint256 i; i < len; i++) {
            InvokeERC20Args calldata arg = args[i];

            _invokeERC20(arg.token, arg.to, arg.amount);
        }
    }

    function invokeERC20(InvokeERC20Args calldata args) external onlyInvokers {
        _invokeERC20(args.token, args.to, args.amount);
    }

    ///////////////////////// VIEW ////////////////////////

    function isUnderlying(address _token) public view returns (bool) {
        return _underlying.includes(_token);
    }

    /// @notice Returns the virtual units of a token
    /// @dev warning! does not revert on non-underlying token
    /// @param token address
    /// @return uint256
    function virtualUnits(address token) public view returns (uint256) {
        return nominals[token];
    }

    /// @notice Returns the virtual units of all tokens
    /// @return TokenInfo[] memory
    function virtualUnits() external view returns (TokenInfo[] memory) {
        address[] storage stor = _underlying.toStorageArray();
        uint256 len = stor.length;

        TokenInfo[] memory info = new TokenInfo[](len);

        for (uint256 i; i < len; i++) {
            address token = stor[i];
            info[i] = TokenInfo({token: token, units: nominals[token]});
        }

        return info;
    }

    /// @notice Returns the real units of a token
    /// @dev warning! does not revert on non-underlying token
    function realUnits(address token) external view returns (uint256) {
        (, , , uint256 currentMultiplier) = multiplier();
        return _computeRealUnits(token, currentMultiplier);
    }

    /// @notice Returns the real units of all tokens
    /// @return TokenInfo[] memory
    function realUnits() public view returns (TokenInfo[] memory) {
        address[] storage stor = _underlying.toStorageArray();
        uint256 len = stor.length;

        TokenInfo[] memory info = new TokenInfo[](len);

        (, , , uint256 currentMultiplier) = multiplier();

        for (uint256 i; i < len; i++) {
            address token = stor[i];
            info[i] = TokenInfo({
                token: token,
                units: _computeRealUnits(token, currentMultiplier)
            });
        }

        return info;
    }

    function underlying() external view returns (address[] memory) {
        return _underlying.toMemoryArray();
    }

    function underlyingLength() external view returns (uint256) {
        return _underlying.size();
    }

    function multiplier()
        public
        view
        returns (
            uint256 trackedTimestamp,
            uint256 trackedMultiplier,
            uint256 newFeeAccrued,
            uint256 currentMultiplier
        )
    {
        (
            trackedTimestamp,
            trackedMultiplier,
            newFeeAccrued,
            currentMultiplier
        ) = Multiplier.computeMultiplier(
            lastKnownTimestamp,
            lastKnownMultiplier,
            feeScaled
        );
    }

    function invariantCheck() public view {
        TokenInfo[] memory tokens = realUnits();

        (, , , uint256 currentMultiplier) = multiplier();

        // adjust total supply by inverse of intraday fee (inflation)
        uint256 totalSupply = fmul(
            fdiv(lastKnownMultiplier, currentMultiplier),
            indexToken.totalSupply()
        );

        for (uint256 i; i < tokens.length; ) {
            uint256 expectedAmount = fmul(tokens[i].units, totalSupply);
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
            delete nominals[token];
            _underlying.remove(token);
            return;
        }

        if (!isUnderlying(token)) {
            _underlying.add(token);
        }

        nominals[token] = _virtualUnits;
    }

    function _invokeERC20(address token, address to, uint256 amount) internal {
        IERC20(token).safeTransfer(to, amount);
    }

    function _computeRealUnits(
        address token,
        uint256 _multiplier
    ) internal view returns (uint256) {
        return fmul(virtualUnits(token), _multiplier);
    }
}
