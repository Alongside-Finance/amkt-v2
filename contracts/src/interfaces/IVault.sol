pragma solidity =0.8.18;

import {TokenInfo} from "src/Common.sol";
import {IIndexToken} from "src/interfaces/IIndexToken.sol";

interface IVault {
    error AMKTVaultOnlyInvokers();
    error AMKTVaultOnly(address who);
    error AMKTVaultInflationRateTooLarge();
    error AMKTVaultFeeTooEarly();
    error AMKTVaultEmergency();
    error VaultInvariant();
    error VaultZeroCheck();

    event VaultIssuanceSet(address issuance);
    event VaultRebalancerSet(address rebalancer);
    event VaultFeeRecipientSet(address feeRecipient);
    event VaultEmergencyResponderSet(address emergencyResponder);
    event VaultInflationRateSet(uint256 inflationRate);
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

    function tryInflation() external;

    function inflationRate() external view returns (uint256);

    function feeRecipient() external view returns (address);

    function lastKnownTimestamp() external view returns (uint256);

    function invokeERC20s(InvokeERC20Args[] calldata args) external;

    function invokeSetNominals(SetNominalArgs[] calldata args) external;

    function invokeERC20(InvokeERC20Args calldata args) external;

    function invokeSetNominal(SetNominalArgs calldata args) external;

    function virtualUnits(address token) external view returns (uint256);

    function virtualUnits() external view returns (TokenInfo[] memory);

    function invariantCheck() external view;

    function isUnderlying(address target) external view returns (bool);

    function underlying() external view returns (address[] memory);

    function underlyingLength() external view returns (uint256);

    function invokeMint(address to, uint256 amount) external;

    function invokeBurn(address from, uint256 amount) external;

    function indexToken() external view returns (IIndexToken);
}
