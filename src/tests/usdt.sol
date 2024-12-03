// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDT is ERC20 {
    constructor() ERC20("Mock USDT", "USDT") {
        _mint(msg.sender, 1000000 * 10 ** decimals()); // Генеруємо 1 млн USDT
    }
}

contract ProjectToken is ERC20 {
    constructor() ERC20("Project Token", "PTK") {
        _mint(msg.sender, 1000000 * 10 ** decimals()); // Генеруємо 1 млн PTK
    }
}
