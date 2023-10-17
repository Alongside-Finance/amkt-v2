//File: contracts/test/migration/Dealer.sol
pragma solidity =0.8.18;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {console} from "forge-std/console.sol";

interface LDO {
    function controller() external view returns (address);

    function generateTokens(
        address _owner,
        uint _amount
    ) external returns (bool);
}

contract Dealer is Test {
    address[1] private nonNormalAssets = [
        address(0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32) // LDO
    ];

    function dealToken(address token, address to, uint256 amount) external {
        bool isNormalAsset = _isNormalAsset(token);
        if (isNormalAsset) {
            deal(token, to, amount);
        } else {
            // only LDO is non normal atm
            _dealLDO(token, to, amount);
        }
    }

    function _isNormalAsset(address token) private view returns (bool) {
        for (uint256 i = 0; i < nonNormalAssets.length; i++) {
            if (nonNormalAssets[i] == token) {
                return false;
            }
        }
        return true;
    }

    function _dealLDO(address token, address to, uint256 amount) private {
        LDO ldo = LDO(token);
        vm.prank(ldo.controller());
        ldo.generateTokens(to, amount);
    }
}
