import sympy as sp

startingSupply = 100e18
collateral = 100e18
timestampDiff = 100 * 60 * 60 * 24 * 365 

# Define the unknown variable feeScaled
perSecondFeeSymbol = sp.Symbol('perSecondFee')

# Define the equation
endSupplySymbol = sp.Symbol('endSupply')
startingValue = startingSupply / collateral
endingValue = startingValue * 0.9905
endSupply = sp.solve(sp.Eq(startingSupply * startingValue, endSupplySymbol * endingValue), endSupplySymbol)[0]
inflatedAmount = endSupply - startingSupply
feeMultiplier = 1e18 - (timestampDiff * perSecondFeeSymbol)
perSecondFee = sp.solve(sp.Eq(inflatedAmount, (startingSupply / feeMultiplier - startingSupply)),perSecondFeeSymbol)[0]
print("The solution for perSecondFee is:", perSecondFee)
print("Yearly inflation is:", inflatedAmount)
print("Yearly inflation is:", endSupply)
