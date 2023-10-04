
import sympy as sp
import math 

def derivePerSecondInflationScaled(daysInterval, valueLoss):
    perSecondInflationSymbol = sp.Symbol('perSecondInflation')
    lastKnownTimestamp = 0
    blockTimestamp = 60 * 60 * 24 * daysInterval
    startingSupply = 1e18
    startingTokenValue = 1e18
    timestampDiff = blockTimestamp - lastKnownTimestamp

    inflationEq = startingSupply * timestampDiff * perSecondInflationSymbol
    endingSupply = startingSupply + inflationEq
    endingValue = startingTokenValue * valueLoss

    # derive perSecondFee by utilizing the fact that the starting NAV and ending NAV should be equal, and that ending token value should be worth 0.9905 of the starting token value
    perSecondInflation = sp.solve(sp.Eq(startingSupply * startingTokenValue, endingSupply * endingValue), perSecondInflationSymbol)[0]
    perSecondInflationScaled = math.floor(perSecondInflation * 1e18) 
    assert perSecondInflationScaled == 304132280 # SHOULD match value in src/scripts/Config.sol
    return perSecondInflationScaled

def calculateInflationAccrued(initialSupply, days, perSecondInflationScaled):
    seconds = 60 * 60 * 24 * days
    inflationAccrued = round(initialSupply * seconds * perSecondInflationScaled / 1e18)
    return inflationAccrued

perSecondInflationScaled =  derivePerSecondInflationScaled(365, 1 - 0.0095)

inflation1000 = calculateInflationAccrued(1e18, 1000, perSecondInflationScaled)
assert inflation1000 == 26277028992000000

inflation730 = calculateInflationAccrued(1e18, 730, perSecondInflationScaled)
assert inflation730 == 19182231164160000

inflation365 = calculateInflationAccrued(1e18, 365, perSecondInflationScaled)
assert inflation365 == 9591115582080000

inflation30 = calculateInflationAccrued(1e18, 30, perSecondInflationScaled)
assert inflation30 ==  788310869760000

inflation7 = calculateInflationAccrued(1e18, 7, perSecondInflationScaled)
assert inflation7 == 183939202944000

inflation1 = calculateInflationAccrued(1e18, 1, perSecondInflationScaled)
assert inflation1 == 26277028992000

