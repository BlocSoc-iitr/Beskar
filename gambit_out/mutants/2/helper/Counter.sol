// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        /// AssignmentMutation(`newNumber` |==> `0`) of: `number = newNumber;`
        number = 0;
    }

    function increment() public {
        number++;
    }
}
