
def fee_to_mint():
    return total_supply / (1-fee) - total_supply

def expected_collateral():
    return total_supply * current_multiplier

total_supply = 100
current_multiplier = 1
collateral = 100
amount_to_mint = 1
fee = 0.1
print("total_supply: ", total_supply)
print("collateral: ", collateral)
print("current_multiplier: ", current_multiplier)

# t = 0 
current_multiplier *= (1-fee)
total_supply += fee_to_mint()
collateral += current_multiplier * amount_to_mint
total_supply += amount_to_mint
print("total_supply: ", total_supply)
print("collateral: ", collateral)
print("expected collateral: ", expected_collateral())
print("current_multiplier: ", current_multiplier)

# t = 0 
current_multiplier *= (1-fee)
total_supply += fee_to_mint()
collateral += current_multiplier * amount_to_mint
total_supply += amount_to_mint
print("total_supply: ", total_supply)
print("collateral: ", collateral)
print("expected collateral: ", expected_collateral())
print("current_multiplier: ", current_multiplier)


total_supply += fee_to_mint()
current_multiplier *= (1-fee)
collateral += current_multiplier * amount_to_mint
total_supply += amount_to_mint
print("total_supply: ", total_supply)
print("collateral: ", collateral)
print("expected collateral: ", expected_collateral())
print("current_multiplier: ", current_multiplier)

