// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        /// AssignmentMutation(`newNumber` |==> `1`) of: `number = newNumber;`
        number = 1;
    }

    function increment() public {
        number++;
    }
}
