//File: contracts/test/migration/Dealer.sol
pragma solidity =0.8.18;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {console} from "forge-std/console.sol";

contract Dealer is Test {
    address[2][] private nonNormalAssetsAndWhales = [
        [
            address(0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32),
            address(0x820fb25352BB0c5E03E07AFc1d86252fFD2F0A18)
        ]
    ];

    function dealToken(address token, address to, uint256 amount) external {
        (bool isNormalAsset, address whale) = _isNormalAsset(token);
        if (isNormalAsset) {
            deal(token, to, amount);
        } else {
            _dealNonNormalAsset(token, to, amount, whale);
        }
    }

    function _isNormalAsset(
        address token
    ) private view returns (bool, address) {
        for (uint256 i = 0; i < nonNormalAssetsAndWhales.length; i++) {
            if (nonNormalAssetsAndWhales[i][0] == token) {
                return (false, nonNormalAssetsAndWhales[i][1]);
            }
        }
        return (true, address(0));
    }

    function _dealNonNormalAsset(
        address token,
        address to,
        uint256 amount,
        address whale
    ) private {
        vm.prank(whale);
        IERC20(token).transfer(to, amount);
    }
}
