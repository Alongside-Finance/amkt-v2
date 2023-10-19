// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {TokenInfo} from "src/Common.sol";

struct Bounty {
    TokenInfo[] infos;
    address fulfiller;
    uint256 deadline;
    bytes32 salt;
}

struct QuoteInput {
    TokenInfo[] targets;
    uint256 supply;
}

interface IInvokeableBounty {
    error BountyInvalidHash();
    error BountyAlreadyCompleted();
    error BountyPastDeadline();
    error BountyAMKTSupplyChange();
    error BountyReentrant();
    error BountyMustIncludeAllUnderlyings();
    error BountyInvalidFulfiller();

    event BountyFulfilled(Bounty bounty, bool callback);

    function fulfillBounty(Bounty memory bounty, bool callback) external;

    function hashBounty(
        Bounty memory bounty
    ) external view returns (bytes32 hash);
}
