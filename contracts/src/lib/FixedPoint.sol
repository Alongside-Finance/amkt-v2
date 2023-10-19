// SPDX-License-Identifier: GPL-3.0
pragma solidity =0.8.18;

uint256 constant SCALAR = 1e18;

function fmul(uint256 a, uint256 b) pure returns (uint256 ret) {
    ret = (a * b) / SCALAR;
}

function fdiv(uint256 a, uint256 b) pure returns (uint256 ret) {
    ret = (a * SCALAR) / b;
}

function finv(uint256 a) pure returns (uint256 ret) {
    ret = fdiv(SCALAR, a);
}
