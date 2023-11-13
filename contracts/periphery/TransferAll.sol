pragma solidity =0.8.18;

interface IERC20 {
    function balanceOf(address account) external view returns (uint256);

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) external returns (bool);
}

contract TransferAll {
    function transferAll(
        IERC20 token,
        address from,
        address to,
        uint256 minimum
    ) external {
        uint256 balance = token.balanceOf(from);
        require(balance >= minimum, "TransferAll: balance too low");
        token.transferFrom(from, to, balance);
    }
}
