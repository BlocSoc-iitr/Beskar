// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        /// DeleteExpressionMutation(`number = newNumber` |==> `assert(true)`) of: `number = newNumber;`
        assert(true);
    }

    function increment() public {
        number++;
    }
}
