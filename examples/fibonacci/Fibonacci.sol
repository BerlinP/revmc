// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Fibonacci {
    function fib(uint256 number) external pure returns (uint256) {
        if (number == 0) {
            return 0;
        } else if (number == 1 || number == 2) {
            return 1;
        } else {
            uint256 fi_1 = 1;
            uint256 fi_2 = 1;

            for (uint256 i = 2; i < number; i++) {
                uint256 fi = fi_2 + fi_1;
                fi_2 = fi_1;
                fi_1 = fi;
            }

            return fi_1;
        }
    }
}