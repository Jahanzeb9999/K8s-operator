# Report


## Gas Optimizations


| |Issue|Instances|
|-|:-|:-:|
| [GAS-1](#GAS-1) | For Operations that will not overflow, you could use unchecked | 3 |
| [GAS-2](#GAS-2) | Functions guaranteed to revert when called by normal users can be marked `payable` | 1 |
### <a name="GAS-1"></a>[GAS-1] For Operations that will not overflow, you could use unchecked

*Instances (3)*:
```solidity
File: example/Test.sol

4: import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

5: import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

6: import "@openzeppelin/contracts/access/Ownable2Step.sol";

```

### <a name="GAS-2"></a>[GAS-2] Functions guaranteed to revert when called by normal users can be marked `payable`
If a function modifier such as `onlyOwner` is used, the function will revert if a normal user tries to pay the function. Marking the function as `payable` will lower the gas cost for legitimate callers because the compiler will not include checks for whether a payment was provided.

*Instances (1)*:
```solidity
File: example/Test.sol

42:     function mint(address account, uint256 amount) external onlyOwner {

```


## Non Critical Issues


| |Issue|Instances|
|-|:-|:-:|
| [NC-1](#NC-1) | Consider disabling `renounceOwnership()` | 1 |
| [NC-2](#NC-2) | Functions should not be longer than 50 lines | 1 |
| [NC-3](#NC-3) | NatSpec is completely non-existent on functions that should have them | 1 |
### <a name="NC-1"></a>[NC-1] Consider disabling `renounceOwnership()`
If the plan for your project does not include eventually giving up all ownership control, consider overwriting OpenZeppelin's `Ownable`'s `renounceOwnership()` function in order to disable it.

*Instances (1)*:
```solidity
File: example/Test.sol

20: contract MawariToken is ERC20, ERC20Burnable, Ownable2Step {

```

### <a name="NC-2"></a>[NC-2] Functions should not be longer than 50 lines
Overly complex code can make understanding functionality more difficult, try to further modularize your code to ensure readability 

*Instances (1)*:
```solidity
File: example/Test.sol

42:     function mint(address account, uint256 amount) external onlyOwner {

```

### <a name="NC-3"></a>[NC-3] NatSpec is completely non-existent on functions that should have them
Public and external functions that aren't view or pure should have NatSpec comments

*Instances (1)*:
```solidity
File: example/Test.sol

47:     function burn(uint256 amount) public override {

```


## Low Issues


| |Issue|Instances|
|-|:-|:-:|
| [L-1](#L-1) | Prevent accidentally burning tokens | 3 |
| [L-2](#L-2) | Solidity version 0.8.20+ may not work on other chains due to `PUSH0` | 1 |
### <a name="L-1"></a>[L-1] Prevent accidentally burning tokens
Minting and burning tokens to address(0) prevention

*Instances (3)*:
```solidity
File: example/Test.sol

33:         _mint(msg.sender, initialSupply);

43:         _mint(account, amount);

48:         _burn(msg.sender, amount);

```

### <a name="L-2"></a>[L-2] Solidity version 0.8.20+ may not work on other chains due to `PUSH0`
The compiler for Solidity 0.8.20 switches the default target EVM version to [Shanghai](https://blog.soliditylang.org/2023/05/10/solidity-0.8.20-release-announcement/#important-note), which includes the new `PUSH0` op code. This op code may not yet be implemented on all L2s, so deployment on these chains will fail. To work around this issue, use an earlier [EVM](https://docs.soliditylang.org/en/v0.8.20/using-the-compiler.html?ref=zaryabs.com#setting-the-evm-version-to-target) [version](https://book.getfoundry.sh/reference/config/solidity-compiler#evm_version). While the project itself may or may not compile with 0.8.20, other projects with which it integrates, or which extend this project may, and those projects will have problems deploying these contracts/libraries.

*Instances (1)*:
```solidity
File: example/Test.sol

2: pragma solidity ^0.8.22;

```


## Medium Issues


| |Issue|Instances|
|-|:-|:-:|
| [M-1](#M-1) | Centralization Risk for trusted owners | 3 |
### <a name="M-1"></a>[M-1] Centralization Risk for trusted owners

#### Impact:
Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.

*Instances (3)*:
```solidity
File: example/Test.sol

20: contract MawariToken is ERC20, ERC20Burnable, Ownable2Step {

31:     ) ERC20(name, symbol) Ownable(msg.sender) {

42:     function mint(address account, uint256 amount) external onlyOwner {

```

