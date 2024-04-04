// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

import {TokenInfo} from "src/Common.sol";

// MAINNET CONFIGS
string constant NAME = "Alongside Crypto Market Index";
string constant SYMBOL = "AMKT";

// timelock is measured in seconds
uint256 constant CANCELLATION_PERIOD = 4 days;

// governor measured in blocks
uint256 constant AVG_BLOCK_TIME = 12; // seconds
uint256 constant VOTE_DELAY = 1 days / AVG_BLOCK_TIME;
uint256 constant VOTE_PERIOD = 4 days / AVG_BLOCK_TIME;
uint256 constant PROPOSAL_THRESHOLD = 100e18; // 100 AMKT. Number of votes required to create a proposal
uint256 constant QUORUM_NUMERATOR = 250; // 2.5%. Denominator is 10000.

address constant MULTISIG = address(0xAeB9ef94b6542BE7112f3a295646B5AaAa9Fca13);

address constant FEE_RECEIPIENT = address(0xC19a5b6E0a923519603985153515222D59cb3F2e);

uint256 constant INFLATION_RATE = 304132280; // per second rate for 95 bps per year

address constant AMKT_PROXY = address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D);

address constant PROXY_ADMIN = address(0x998930C351EcB4918A5c5238B62d5277fE45ab9b);

address constant VAULT = address(0xf3bCeDaB2998933c6AAD1cB31430D8bAb329dD8C);

// Native
address constant BTC = address(0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599);
address constant ETH = address(0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0);
address constant MATIC = address(0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0);
address constant FTM = address(0x4E15361FD6b4BB609Fa63C81A2be19d873717870);
address constant SHIB = address(0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE);
address constant LINK = address(0x514910771AF9Ca656af840dff83E8264EcF986CA);
address constant UNI = address(0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984);
address constant LDO = address(0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32);
address constant MNT = address(0x3c3a81e81dc49A522A592e7622A7E711c06bf354);
address constant CRO = address(0xA0b73E1Ff0B80914AB6fe0444E65848C4C34450b);
address constant QNT = address(0x4a220E6096B25EADb88358cb44068A3248254675);
address constant ARB = address(0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1);
address constant MKR = address(0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2);
address constant AAVE = address(0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9);
address constant GRT = address(0xc944E90C64B2c07662A292be6244BDf05Cda44a7);

// Wormhole Bridge
address constant BNB = address(0x418D75f65a02b3D53B2418FB8E1fe493759c7605);
address constant SOL = address(0xD31a59c85aE9D8edEFeC411D448f90841571b89c);
address constant AVAX = address(0x85f138bfEE4ef8e540890CFb48F620571d67Eda3);
address constant OP = address(0x1df721D242E0783F8fCab4A9FfE4F35bdf329909);

// Rainbow Bridge
address constant NEAR = address(0x85F17Cf997934a597031b2E18a9aB6ebD4B9f6a4);

contract InitialBountyHelper {
    function tokens() public pure returns (TokenInfo[] memory) {
        TokenInfo[] memory _tokens = new TokenInfo[](15);
        _tokens[0] = TokenInfo(BTC, 210808);
        _tokens[1] = TokenInfo(ETH, 11341559218314196);
        _tokens[2] = TokenInfo(BNB, 16376857087864504);
        _tokens[3] = TokenInfo(SOL, 45261289);
        _tokens[4] = TokenInfo(MATIC, 1005002983049670270);
        _tokens[5] = TokenInfo(LINK, 60112353509468574);
        _tokens[6] = TokenInfo(SHIB, 63620700281571205185534);
        _tokens[7] = TokenInfo(AVAX, 38334030865722302);
        _tokens[8] = TokenInfo(UNI, 63495487776147326);
        _tokens[9] = TokenInfo(MKR, 105536410232513);
        _tokens[10] = TokenInfo(LDO, 96100274703325102);
        _tokens[11] = TokenInfo(CRO, 272717390);
        _tokens[12] = TokenInfo(MNT, 335630438363918014);
        _tokens[13] = TokenInfo(OP, 95044850012921166);
        _tokens[14] = TokenInfo(QNT, 1303432234303866);
        return _tokens;
    }

    // WARNING:
    // The amounts will be determined shortly before the bounty is proposed.
    // The goal is to have the bounty be equivalent the net asset value of AMKT at the time of proposal.
    // 15 assets to be included in the index
    // Once tokens and their amounts are finalized, switch this to false
    // Expected date of finalization is October 30, 2023
    bool public constant triggerMigrationWarning_finalTokens = false; // Flip when tokens are finalized
}
