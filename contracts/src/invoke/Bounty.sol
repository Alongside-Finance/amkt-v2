// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {IIndexToken} from "src/interfaces/IIndexToken.sol";
import {IVault} from "src/interfaces/IVault.sol";
import {IInvokeableBounty, Bounty, QuoteInput} from "src/interfaces/IInvokeableBounty.sol";
import {IActiveBounty} from "src/interfaces/IActiveBounty.sol";
import {IRebalancer} from "src/interfaces/IRebalancer.sol";
import {TokenInfo} from "src/Common.sol";
import {SCALAR, fmul} from "src/lib/FixedPoint.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract InvokeableBounty is IInvokeableBounty {
    using SafeERC20 for IERC20;

    mapping(bytes32 => bool) public completedBounties;

    IIndexToken public immutable indexToken;

    IVault public immutable vault;

    IActiveBounty public immutable activeBounty;

    uint256 public immutable version;
    uint256 public immutable chainId;

    uint256 public reentrancyLock = 1;

    modifier reentrancyGuard() {
        if (reentrancyLock > 1) revert BountyReentrant();
        reentrancyLock = 2;
        _;
        reentrancyLock = 1;
    }

    modifier invariantCheck() {
        _;
        vault.invariantCheck();
    }

    constructor(
        address _vault,
        address _activeBounty,
        uint256 _version,
        uint256 _chainId
    ) {
        vault = IVault(_vault);
        indexToken = IIndexToken(vault.indexToken());
        activeBounty = IActiveBounty(_activeBounty);
        version = _version;
        chainId = _chainId;
    }

    /// @dev we send out the tokens first, so we need to check for weird supply stuff
    /// @dev also we dont follow CEI so we need to check for reentrancy
    /// @dev the units in the bounty are the target units, ie amount of units per 1e18 amkt
    /// @dev check for supply because even though they mint/burn at the smae price becasue nominals havent been changed yet,
    ///      if the  supply changes the "value" of the first leg will be differnt from the "value" of the second leg
    ///
    ///
    /// @param bounty the bounty to fulfill. if fulfiller is 0, anyone can fulfill
    /// @param callback whether or not to call the rebalancer callback
    function fulfillBounty(
        Bounty memory bounty,
        bool callback
    ) external reentrancyGuard invariantCheck {
        bytes32 bountyHash = _validateInput(bounty);

        uint256 startingSupply = indexToken.totalSupply();

        (
            IVault.InvokeERC20Args[] memory outs,
            TokenInfo[] memory ins,
            IVault.SetNominalArgs[] memory nominals
        ) = _quote(QuoteInput(bounty.infos, startingSupply));

        // sends all the tokens to the rebalancer first
        vault.invokeERC20s(outs);

        if (callback) {
            IRebalancer(msg.sender).rebalanceCallback(
                ins,
                _intoTokenInfo(outs)
            );
        }

        // in case rebalancer has a callback
        _checkSupplyChange(startingSupply);

        // take all tokens from msg.sender
        for (uint256 i; i < ins.length; i++) {
            IERC20(ins[i].token).safeTransferFrom(
                msg.sender,
                address(vault),
                ins[i].units
            );
        }

        // in case any tokens have a callback
        _checkSupplyChange(startingSupply);

        vault.invokeSetNominals(nominals);

        completedBounties[bountyHash] = true;
        emit BountyFulfilled(bounty, callback);
    }

    /// @notice hash a bounty
    /// @param bounty, the bounty to hash
    function hashBounty(
        Bounty memory bounty
    ) public view returns (bytes32 hash) {
        return
            keccak256(
                abi.encode(
                    "alongside::invoker::bounty",
                    abi.encode(version),
                    abi.encode(chainId),
                    keccak256(abi.encode(bounty))
                )
            );
    }

    ///////////////////////// INTERNAL /////////////////////////

    function _validateInput(Bounty memory bounty) internal returns (bytes32) {
        bytes32 bountyHash = hashBounty(bounty);

        if (activeBounty.activeBounty() != bountyHash)
            revert BountyInvalidHash();

        if (completedBounties[bountyHash]) revert BountyAlreadyCompleted();

        if (block.timestamp > bounty.deadline) revert BountyPastDeadline();

        if (bounty.fulfiller != address(0) && bounty.fulfiller != msg.sender)
            revert BountyInvalidFulfiller();

        if (bounty.infos.length < vault.underlyingLength())
            revert BountyMustIncludeAllUnderlyings();

        address[] memory prevTokens = vault.underlying();

        for (uint256 i; i < bounty.infos.length; i++) {
            if (i < prevTokens.length && prevTokens[i] != bounty.infos[i].token)
                revert BountyMustIncludeAllUnderlyings();
            if (bounty.infos[i].token == address(0))
                revert BountyInvalidToken();
        }

        return bountyHash;
    }

    function _checkSupplyChange(uint256 startingSupply) internal {
        if (indexToken.totalSupply() != startingSupply) {
            revert BountyAMKTSupplyChange();
        }
    }

    function _quote(
        QuoteInput memory input
    )
        internal
        view
        returns (
            IVault.InvokeERC20Args[] memory outs,
            TokenInfo[] memory ins,
            IVault.SetNominalArgs[] memory nominals
        )
    {
        outs = new IVault.InvokeERC20Args[](input.targets.length);

        ins = new TokenInfo[](input.targets.length);

        nominals = new IVault.SetNominalArgs[](input.targets.length);

        // store the lengths because we don't actually know the size off the bat
        uint256 lenOuts;
        uint256 lenIns;

        for (uint256 i; i < input.targets.length; i++) {
            address token = input.targets[i].token;

            // number of target units per 1e18 amkt
            uint256 targetUnits = input.targets[i].units;

            uint256 virtualUnits = vault.virtualUnits(token);

            if (virtualUnits > targetUnits) {
                outs[lenOuts] = IVault.InvokeERC20Args(
                    token,
                    msg.sender,
                    fmul(virtualUnits - targetUnits, input.supply)
                );

                unchecked {
                    lenOuts++;
                }
            } else if (targetUnits > virtualUnits) {
                ins[lenIns] = TokenInfo(
                    token,
                    fmul(targetUnits - virtualUnits + 1, input.supply) + 1
                );

                unchecked {
                    lenIns++;
                }
            } else {
                // they're equal, so we don't need to do anything
                continue;
            }

            nominals[lenOuts + lenIns - 1] = IVault.SetNominalArgs(
                token,
                targetUnits
            );
        }

        uint256 lenNominals = lenOuts + lenIns;

        // use assembly to set the actual sizes so were not sending over a bunch of empty data
        // safe, since the compiler is planning on allocating outside of this empty zone anyway
        assembly {
            mstore(outs, lenOuts)
            mstore(ins, lenIns)
            mstore(nominals, lenNominals)
        }

        return (outs, ins, nominals);
    }

    function _intoTokenInfo(
        IVault.InvokeERC20Args[] memory args
    ) internal pure returns (TokenInfo[] memory infos) {
        infos = new TokenInfo[](args.length);

        for (uint256 i; i < args.length; i++) {
            infos[i] = TokenInfo(args[i].token, args[i].amount);
        }
    }
}
