pragma solidity =0.8.18;

import {TokenInfo} from "src/Common.sol";

// MAINNET CONFIGS
string constant NAME = "Alongside Crypto Market Index";
string constant SYMBOL = "AMKT";
uint256 constant VERSION = 0;

// timelock is measured in seconds
uint256 constant CANCELLATION_PERIOD = 4 days;

// governor measured in blocks
uint256 constant AVG_BLOCK_TIME = 12; // seconds
uint256 constant VOTE_DELAY = 1 days / AVG_BLOCK_TIME;
uint256 constant VOTE_PERIOD = 4 days / AVG_BLOCK_TIME;
uint256 constant PROPOSAL_THRESHOLD = 100e18; // Number of votes required to create a proposal
uint256 constant GOVERNOR_NUMERATOR = 250;

address constant MULTISIG = address(0xAeB9ef94b6542BE7112f3a295646B5AaAa9Fca13);

address constant FEE_RECEIPIENT = address(
    0xC19a5b6E0a923519603985153515222D59cb3F2e
);

uint256 constant FEE_SCALED = 320139242; // TODO: Derive this

address constant PROXY = address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D);
address constant PROXY_ADMIN = address(
    0x998930C351EcB4918A5c5238B62d5277fE45ab9b
);

address constant AMKT = address(0xF17A3fE536F8F7847F1385ec1bC967b2Ca9caE8D);

contract InitialBountyHelper {
    // Native
    address constant BTC = address(0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599);
    address constant ETH = address(0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0);
    address constant MATIC =
        address(0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0);
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

    function tokens() public returns (TokenInfo[] memory) {
        TokenInfo[] memory tokens = new TokenInfo[](15);

        // The amounts will be determined shortly before the bounty is proposed.
        // The goal is to have the bounty be equivalent the net asset value of AMKT at the time of proposal.
        // 15 assets to be included in the index
        tokens[0] = TokenInfo(BTC, 1e22);
        tokens[1] = TokenInfo(ETH, 1e21);
        tokens[2] = TokenInfo(BNB, 1e20);
        tokens[3] = TokenInfo(SOL, 1e19);
        tokens[4] = TokenInfo(MATIC, 1e18);
        tokens[5] = TokenInfo(SHIB, 1e17);
        tokens[6] = TokenInfo(AVAX, 1e16);
        tokens[7] = TokenInfo(LINK, 1e15);
        tokens[8] = TokenInfo(UNI, 1e14);
        tokens[9] = TokenInfo(LDO, 1e13);
        tokens[10] = TokenInfo(MNT, 1e12);
        tokens[11] = TokenInfo(CRO, 1e11);
        tokens[12] = TokenInfo(QNT, 1e10);
        tokens[13] = TokenInfo(ARB, 1e9);
        tokens[14] = TokenInfo(MKR, 1e8);
        // tokens[15] = TokenInfo(NEAR, 1);
        // tokens[16] = TokenInfo(OP, 1);
        // tokens[17] = TokenInfo(AAVE, 1);
        // tokens[18] = TokenInfo(GRT, 1);

        return tokens;
    }
}
