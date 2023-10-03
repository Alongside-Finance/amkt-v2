import sympy as sp

startingSupply = 100e18
collateral = 100e18
timestampDiff =  60 * 60 * 24 * 365 

# Define the unknown variable feeScaled
perSecondFeeSymbol = sp.Symbol('perSecondFee')

# Define the equation
endSupplySymbol = sp.Symbol('endSupply')
startingValue = startingSupply / collateral
endingValue = startingValue * 0.9905
endSupply = sp.solve(sp.Eq(startingSupply * startingValue, endSupplySymbol * endingValue), endSupplySymbol)[0]
inflatedAmount = endSupply - startingSupply
perSecondFee = sp.solve(sp.Eq(endSupply, perSecondFeeSymbol * timestampDiff * startingSupply / 1e18), perSecondFeeSymbol)[0]
endSupplyUsingDerivedFee = perSecondFee * timestampDiff * startingSupply / 1e18

print("The solution for perSecondFee is:", perSecondFee)
print("Inflated amount is:", inflatedAmount)
print("End supply is:", endSupply)
print("Timestamp diff is:", timestampDiff) 
print("Inflated supply using fee is:", endSupplyUsingDerivedFee)