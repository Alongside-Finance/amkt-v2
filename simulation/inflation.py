
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
perSecondFee = sp.solve(sp.Eq(startingSupply * startingTokenValue, endingSupply * endingValue), perSecondFeeSymbol)

print(perSecondFee)

