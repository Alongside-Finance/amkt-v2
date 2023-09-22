pragma solidity =0.8.15;

import {TokenInfo} from "../Common.sol";
import {IIndexToken} from "./IIndexToken.sol";

interface IVault {
    error AMKTVaultOnlyInvokers();
    error AMKTVaultOnly(address who);
    error AMKTVaultFeeTooLarge();
    error AMKTVaultEmergency();
    error VaultInvariant();

    event VaultIssuanceSet(address issuance);
    event VaultRebalancerSet(address rebalancer);
    event VaultFeeRecipientSet(address feeRecipient);
    event VaultEmergencyResponderSet(address emergencyResponder);
    event VaultFeeScaledSet(uint256 feeScaled);
    event VaultEmergencySet(bool emergency);
    event VaultFeeMinted(address indexed to, uint256 amount);

    struct InvokeERC20Args {
        address token;
        address to;
        uint256 amount;
    }

    struct SetNominalArgs {
        address token;
        uint256 virtualUnits;
    }

    function issuance() external view returns (address);

    function rebalancer() external view returns (address);

    function tryInflation() external returns (uint256);

    function feeScaled() external view returns (uint256);

    function feeRecipient() external view returns (address);

    function invokeERC20s(InvokeERC20Args[] calldata args) external;

    function invokeSetNominals(SetNominalArgs[] calldata args) external;

    function invokeERC20(InvokeERC20Args calldata args) external;

    function invokeSetNominal(SetNominalArgs calldata args) external;

    function invokeSetMultiplier(uint256 multiplier) external;

    function virtualUnits(address token) external view returns (uint256);

    function realUnits(address token) external view returns (uint256);

    function realUnits() external view returns (TokenInfo[] memory);

    function invariantCheck() external view;

    function isUnderlying(address target) external view returns (bool);

    function underlying() external view returns (address[] memory);

    function underlyingLength() external view returns (uint256);

    function invokeMint(address to, uint256 amount) external;

    function invokeBurn(address from, uint256 amount) external;

    function indexToken() external view returns (IIndexToken);

    function multiplier()
        external
        view
        returns (
            uint256 lastTrackedTimestamp,
            uint256 lastTrackedMultiplier,
            uint256 newFeeAccrued,
            uint256 multiplier
        );
}
