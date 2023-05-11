// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract HapiCore is OwnableUpgradeable {
    function initialize() public initializer {
        __Ownable_init();
    }
}
