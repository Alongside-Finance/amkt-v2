
import sympy as sp

perSecondFeeSymbol = sp.Symbol('perSecondFee')

lastKnownTimestamp = 0
blockTimestamp = 60 * 60 * 24 * 365
startingSupply = 100e18
startingTokenValue = 1e18
timestampDiff = blockTimestamp - lastKnownTimestamp

inflationEq = startingSupply * timestampDiff * perSecondFeeSymbol
endingSupply = startingSupply + inflationEq
endingValue = startingTokenValue * 0.9905

# derive perSecondFee by utilizing the fact that the starting NAV and ending NAV should be equal, and that ending token value should be worth 0.9905 of the starting token value
perSecondFee = sp.solve(sp.Eq(startingSupply * startingTokenValue, endingSupply * endingValue), perSecondFeeSymbol)

print(perSecondFee)

