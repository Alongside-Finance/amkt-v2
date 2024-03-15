contract Constants {
    // STATIC
    address constant WBTC = address(0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599);
    address constant WSTETH =
        address(0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0);
    address constant MATIC =
        address(0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0);
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
    address constant WORMHOLE_BNB =
        address(0x418D75f65a02b3D53B2418FB8E1fe493759c7605);
    address constant WORMHOLE_SOL =
        address(0xD31a59c85aE9D8edEFeC411D448f90841571b89c);
    address constant WORMHOLE_AVAX =
        address(0x85f138bfEE4ef8e540890CFb48F620571d67Eda3);
    address constant _21CO_BNB =
        address(0x1bE9d03BfC211D83CFf3ABDb94A75F9Db46e1334);
    address constant _21CO_SOL =
        address(0xb80a1d87654BEf7aD8eB6BBDa3d2309E31D4e598);
    address constant _21CO_AVAX =
        address(0x399508A43d7E2b4451cd344633108b4d84b33B03);
    address constant ASTETH =
        address(0x27C2B9fd547EAd2c05C305BeE2399A55811257c2);
    address constant _21CO_XRP =
        address(0x0d3bd40758dF4F79aaD316707FcB809CD4815Ffe);
    address constant _21CO_ADA =
        address(0x9c05d54645306d4C4EAd6f75846000E1554c0360);
    address constant _21CO_DOGE =
        address(0xD2aEE1CE2b4459dE326971DE036E82f1318270AF);
    address constant _21CO_DOT =
        address(0xF4ACCD20bFED4dFFe06d4C85A7f9924b1d5dA819);
    address constant _21CO_LTC =
        address(0x9F2825333aa7bC2C98c061924871B6C016e385F3);
    address constant _21CO_BCH =
        address(0xFf4927e04c6a01868284F5C3fB9cba7F7ca4aeC0);
    address constant STETH =
        address(0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84);
    address constant VAULT =
        address(0xf3bCeDaB2998933c6AAD1cB31430D8bAb329dD8C);

    address constant INVOKEABLE_BOUNTY =
        address(0xE13Ee59C41c67696754277cDC73710f6D65Ef2Ac);
    address constant ACTIVE_BOUNTY =
        address(0x0DAF7e851f6054085432229150c1706988aBc562);

    address constant FULFILLER_SAFE =
        address(0x5ae65506979C00D70A13E7cE9eBf984d31660e5c);
    address constant QUOTER =
        address(0xE3BE63E1B959c152212ce1dD45D0d2f749eB227c);

    // CONFIGS
    uint256 constant PREVIOUS_TOTAL_SUPPLY = 19824299042318053468632;
    uint256 constant BOUNTY_DEADLINE = 1712448000; // April 7, 2024 0:0:0 GMT
    uint256 constant FORK_BLOCK = 19436800;

    uint256 constant WBTC_UNITS = 198925;
    uint256 constant WSTETH_UNITS = 5239265214571544;
    uint256 constant WORMHOLE_BNB_UNITS = 0;
    uint256 constant WORMHOLE_SOL_UNITS = 0;
    uint256 constant MATIC_UNITS = 1001998918595707392;
    uint256 constant LINK_UNITS = 59426878106494536;
    uint256 constant SHIB_UNITS = 59648495568490195845120;
    uint256 constant WORMHOLE_AVAX_UNITS = 0;
    uint256 constant UNI_UNITS = 60604703429056984;
    uint256 constant ASTETH_UNITS = 6077994279075689;
    uint256 constant _21CO_XRP_UNITS = 5540519;
    uint256 constant _21CO_ADA_UNITS = 3598490;
    uint256 constant _21CO_DOGE_UNITS = 1451935823;
    uint256 constant _21CO_DOT_UNITS = 1304797370;
    uint256 constant _21CO_LTC_UNITS = 752108;
    uint256 constant _21CO_BCH_UNITS = 199067;
    uint256 constant _21CO_BNB_UNITS = 1513655;
    uint256 constant _21CO_SOL_UNITS = 44894999;
    uint256 constant _21CO_AVAX_UNITS = 38191836757154176;

    uint256 constant FULFILLER_MISSING_WBTC_BALANCE = 1962606; // 8 decimals
    uint256 constant FULFILLER_MISSING_MATIC_BALANCE = 635184227465916332298; // 18 decimals
    uint256 constant FULFILLER_MISSING_LINK_BALANCE = 34867406970524462631; // 18 decimals
    uint256 constant FULFILLER_MISSING_21CO_BNB_BALANCE = 30007169192; // 8 decimals
    uint256 constant FULFILLER_MISSING_21CO_SOL_BALANCE = 890011905505; // 9 decimals
    uint256 constant FULFILLER_MISSING_21CO_AVAX_BALANCE =
        757126392849218983902; // 18 decimals
}
