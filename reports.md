# Report


## Gas Optimizations


| |Issue|Instances|
|-|:-|:-:|
| [GAS-1](#GAS-1) | Don't use `_msgSender()` if not supporting EIP-2771 | 2 |
| [GAS-2](#GAS-2) | Cache array length outside of loop | 1 |
| [GAS-3](#GAS-3) | For Operations that will not overflow, you could use unchecked | 16 |
| [GAS-4](#GAS-4) | Use Custom Errors instead of Revert Strings to save Gas | 2 |
| [GAS-5](#GAS-5) | Avoid contract existence checks by using low level calls | 1 |
| [GAS-6](#GAS-6) | Functions guaranteed to revert when called by normal users can be marked `payable` | 9 |
| [GAS-7](#GAS-7) | `++i` costs less gas compared to `i++` or `i += 1` (same for `--i` vs `i--` or `i -= 1`) | 1 |
| [GAS-8](#GAS-8) | Using `private` rather than `public` for constants, saves gas | 3 |
| [GAS-9](#GAS-9) | Use of `this` instead of marking as `public` an `external` function | 2 |
| [GAS-10](#GAS-10) | Increments/decrements can be unchecked in for-loops | 1 |
| [GAS-11](#GAS-11) | Use != 0 instead of > 0 for unsigned integer comparison | 2 |
### <a name="GAS-1"></a>[GAS-1] Don't use `_msgSender()` if not supporting EIP-2771
Use `msg.sender` if the code does not implement [EIP-2771 trusted forwarder](https://eips.ethereum.org/EIPS/eip-2771) support

*Instances (2)*:
```solidity
File: example/Test.sol

102:         require(hasRole(DEFAULT_ADMIN_ROLE, _msgSender()) || hasRole(DEVELOPER_ROLE, _msgSender()), NotAuthorized());

276:         require(hasRole(DEFAULT_ADMIN_ROLE, _msgSender()), NotAuthorized());

```

### <a name="GAS-2"></a>[GAS-2] Cache array length outside of loop
If not cached, the solidity compiler will always read the length of the array during each iteration. That is, if it is a storage array, this is an extra sload operation (100 additional extra gas for each iteration except for the first) and if it is a memory array, this is an extra mload operation (3 additional gas for each iteration except for the first).

*Instances (1)*:
```solidity
File: example/Test.sol

156:         for (uint256 i; i < data.length; i++) {

```

### <a name="GAS-3"></a>[GAS-3] For Operations that will not overflow, you could use unchecked

*Instances (16)*:
```solidity
File: example/Test.sol

4: import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721EnumerableUpgradeable.sol";

5: import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

6: import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

7: import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721BurnableUpgradeable.sol";

8: import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";

9: import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

10: import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";

11: import "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";

127:         require(totalSupply() + quantity <= MAX_SUPPLY, MaxSupplyExceeded());

136:                 ++i;

140:         emit BatchNFTMinted(to, startTokenId, nextTokenId - 1, quantity);

156:         for (uint256 i; i < data.length; i++) {

208:         return MAX_SUPPLY - totalSupply();

258:             revert("NFTs are permanently non-transferable");

275:     function _authorizeUpgrade(address /* newImplementation */ ) internal view override {

307:             nextTokenId = tokenId + 1;

```

### <a name="GAS-4"></a>[GAS-4] Use Custom Errors instead of Revert Strings to save Gas
Custom errors are available from solidity version 0.8.4. Custom errors save [**~50 gas**](https://gist.github.com/IllIllI000/ad1bd0d29a0101b25e57c293b4b0c746) each time they're hit by [avoiding having to allocate and store the revert string](https://blog.soliditylang.org/2021/04/21/custom-errors/#errors-in-depth). Not defining the strings also save deployment gas

Additionally, custom errors can be used inside and outside of contracts (including interfaces and libraries).

Source: <https://blog.soliditylang.org/2021/04/21/custom-errors/>:

> Starting from [Solidity v0.8.4](https://github.com/ethereum/solidity/releases/tag/v0.8.4), there is a convenient and gas-efficient way to explain to users why an operation failed through the use of custom errors. Until now, you could already use strings to give more information about failures (e.g., `revert("Insufficient funds.");`), but they are rather expensive, especially when it comes to deploy cost, and it is difficult to use dynamic information in them.

Consider replacing **all revert strings** with custom errors in the solution, and particularly those that have multiple occurrences:

*Instances (2)*:
```solidity
File: example/Test.sol

228:             revert("ERC721Metadata: URI query for nonexistent token");

258:             revert("NFTs are permanently non-transferable");

```

### <a name="GAS-5"></a>[GAS-5] Avoid contract existence checks by using low level calls
Prior to 0.8.10 the compiler inserted extra code, including `EXTCODESIZE` (**100 gas**), to check for contract existence for external function calls. In more recent solidity versions, the compiler will not insert these checks if the external call has a return value. Similar behavior can be achieved in earlier versions by using low-level calls, since low level calls never check for contract existence

*Instances (1)*:
```solidity
File: example/Test.sol

162:             (bool success, bytes memory result) = address(this).delegatecall(data[i]);

```

### <a name="GAS-6"></a>[GAS-6] Functions guaranteed to revert when called by normal users can be marked `payable`
If a function modifier such as `onlyOwner` is used, the function will revert if a normal user tries to pay the function. Marking the function as `payable` will lower the gas cost for legitimate callers because the compiler will not include checks for whether a payment was provided.

*Instances (9)*:
```solidity
File: example/Test.sol

106:     function mint() external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant whenNotPaused {

113:     function mintTo(address recipient) external onlyAdminOrDev nonReentrant whenNotPaused {

126:     function batchMintTo(address to, uint256 quantity) external onlyAdminOrDev nonReentrant whenNotPaused {

175:     function adminBurn(uint256 tokenId) external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant whenNotPaused {

183:     function setTokenURI(string memory newTokenURI) external onlyRole(DEFAULT_ADMIN_ROLE) {

193:     function addDeveloper(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {

202:     function removeDeveloper(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {

212:     function pause() external onlyRole(PAUSER_ROLE) {

216:     function unpause() external onlyRole(PAUSER_ROLE) {

```

### <a name="GAS-7"></a>[GAS-7] `++i` costs less gas compared to `i++` or `i += 1` (same for `--i` vs `i--` or `i -= 1`)
Pre-increments and pre-decrements are cheaper.

For a `uint256 i` variable, the following is true with the Optimizer enabled at 10k:

**Increment:**

- `i += 1` is the most expensive form
- `i++` costs 6 gas less than `i += 1`
- `++i` costs 5 gas less than `i++` (11 gas less than `i += 1`)

**Decrement:**

- `i -= 1` is the most expensive form
- `i--` costs 11 gas less than `i -= 1`
- `--i` costs 5 gas less than `i--` (16 gas less than `i -= 1`)

Note that post-increments (or post-decrements) return the old value before incrementing or decrementing, hence the name *post-increment*:

```solidity
uint i = 1;  
uint j = 2;
require(j == i++, "This will be false as i is incremented after the comparison");
```
  
However, pre-increments (or pre-decrements) return the new value:
  
```solidity
uint i = 1;  
uint j = 2;
require(j == ++i, "This will be true as i is incremented before the comparison");
```

In the pre-increment case, the compiler has to create a temporary variable (when used) for returning `1` instead of `2`.

Consider using pre-increments and pre-decrements where they are relevant (meaning: not where post-increments/decrements logic are relevant).

*Saves 5 gas per instance*

*Instances (1)*:
```solidity
File: example/Test.sol

156:         for (uint256 i; i < data.length; i++) {

```

### <a name="GAS-8"></a>[GAS-8] Using `private` rather than `public` for constants, saves gas
If needed, the values can be read from the verified contract source code, or if there are multiple values there can be a single getter function that [returns a tuple](https://github.com/code-423n4/2022-08-frax/blob/90f55a9ce4e25bceed3a74290b854341d8de6afa/src/contracts/FraxlendPair.sol#L156-L178) of the values of all currently-public constants. Saves **3406-3606 gas** in deployment gas due to the compiler not having to create non-payable getter functions for deployment calldata, not having to store the bytes of the value outside of where it's used, and not adding another entry to the method ID table

*Instances (3)*:
```solidity
File: example/Test.sol

43:     bytes32 public constant DEVELOPER_ROLE = keccak256("DEVELOPER_ROLE");

44:     bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

54:     uint256 public constant MAX_SUPPLY = 5000;

```

### <a name="GAS-9"></a>[GAS-9] Use of `this` instead of marking as `public` an `external` function
Using `this.` is like making an expensive external call. Consider marking the called function as public

*Saves around 2000 gas per instance*

*Instances (2)*:
```solidity
File: example/Test.sol

225:         try this.ownerOf(tokenId) returns (address) {

285:         try this.ownerOf(tokenId) returns (address) {

```

### <a name="GAS-10"></a>[GAS-10] Increments/decrements can be unchecked in for-loops
In Solidity 0.8+, there's a default overflow check on unsigned integers. It's possible to uncheck this in for-loops and save some gas at each iteration, but at the cost of some code readability, as this uncheck cannot be made inline.

[ethereum/solidity#10695](https://github.com/ethereum/solidity/issues/10695)

The change would be:

```diff
- for (uint256 i; i < numIterations; i++) {
+ for (uint256 i; i < numIterations;) {
 // ...  
+   unchecked { ++i; }
}  
```

These save around **25 gas saved** per instance.

The same can be applied with decrements (which should use `break` when `i == 0`).

The risk of overflow is non-existent for `uint256`.

*Instances (1)*:
```solidity
File: example/Test.sol

156:         for (uint256 i; i < data.length; i++) {

```

### <a name="GAS-11"></a>[GAS-11] Use != 0 instead of > 0 for unsigned integer comparison

*Instances (2)*:
```solidity
File: example/Test.sol

128:         require(quantity > 0, InvalidQuantity());

184:         require(bytes(newTokenURI).length > 0, InvalidTokenURI());

```


## Non Critical Issues


| |Issue|Instances|
|-|:-|:-:|
| [NC-1](#NC-1) | Use `string.concat()` or `bytes.concat()` instead of `abi.encodePacked` | 1 |
| [NC-2](#NC-2) | `constant`s should be defined rather than using magic numbers | 2 |
| [NC-3](#NC-3) | Functions should not be longer than 50 lines | 16 |
| [NC-4](#NC-4) | Use scientific notation for readability reasons for large multiples of ten | 1 |
| [NC-5](#NC-5) | Use Underscores for Number Literals (add an underscore every 3 digits) | 2 |
### <a name="NC-1"></a>[NC-1] Use `string.concat()` or `bytes.concat()` instead of `abi.encodePacked`
Solidity version 0.8.4 introduces `bytes.concat()` (vs `abi.encodePacked(<bytes>,<bytes>)`)

Solidity version 0.8.12 introduces `string.concat()` (vs `abi.encodePacked(<str>,<str>), which catches concatenation errors (in the event of a `bytes` data mixed in the concatenation)`)

*Instances (1)*:
```solidity
File: example/Test.sol

226:             return string(abi.encodePacked(_tokenURI, Strings.toString(tokenId), ".json"));

```

### <a name="NC-2"></a>[NC-2] `constant`s should be defined rather than using magic numbers
Even [assembly](https://github.com/code-423n4/2022-05-opensea-seaport/blob/9d7ce4d08bf3c3010304a0476a785c70c0e90ae7/contracts/lib/TokenTransferrer.sol#L35-L39) can benefit from using readable constants instead of hex/numeric literals

*Instances (2)*:
```solidity
File: example/Test.sol

92:         nextTokenId = 1000001;

157:             require(data[i].length >= 4, InvalidQuantity());

```

### <a name="NC-3"></a>[NC-3] Functions should not be longer than 50 lines
Overly complex code can make understanding functionality more difficult, try to further modularize your code to ensure readability 

*Instances (16)*:
```solidity
File: example/Test.sol

77:     function initialize(string memory name, string memory symbol, string memory tokenURI_, address pauserAddress)

106:     function mint() external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant whenNotPaused {

113:     function mintTo(address recipient) external onlyAdminOrDev nonReentrant whenNotPaused {

126:     function batchMintTo(address to, uint256 quantity) external onlyAdminOrDev nonReentrant whenNotPaused {

175:     function adminBurn(uint256 tokenId) external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant whenNotPaused {

183:     function setTokenURI(string memory newTokenURI) external onlyRole(DEFAULT_ADMIN_ROLE) {

193:     function addDeveloper(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {

202:     function removeDeveloper(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {

207:     function getRemainingSupply() external view returns (uint256) {

216:     function unpause() external onlyRole(PAUSER_ROLE) {

224:     function tokenURI(uint256 tokenId) public view virtual override returns (string memory) {

248:     function _update(address to, uint256 tokenId, address auth)

264:     function _increaseBalance(address account, uint128 value)

275:     function _authorizeUpgrade(address /* newImplementation */ ) internal view override {

284:     function _exists(uint256 tokenId) internal view returns (bool) {

301:     function _mintToken(address to) private returns (uint256 tokenId) {

```

### <a name="NC-4"></a>[NC-4] Use scientific notation for readability reasons for large multiples of ten
The more a number has zeros, the harder it becomes to see with the eyes if it's the intended value. To ease auditing and bug bounty hunting, consider using the scientific notation

*Instances (1)*:
```solidity
File: example/Test.sol

92:         nextTokenId = 1000001;

```

### <a name="NC-5"></a>[NC-5] Use Underscores for Number Literals (add an underscore every 3 digits)

*Instances (2)*:
```solidity
File: example/Test.sol

54:     uint256 public constant MAX_SUPPLY = 5000;

92:         nextTokenId = 1000001;

```


## Low Issues


| |Issue|Instances|
|-|:-|:-:|
| [L-1](#L-1) | Initializers could be front-run | 9 |
| [L-2](#L-2) | Upgradeable contract is missing a `__gap[50]` storage variable to allow for new storage variables in later versions | 19 |
| [L-3](#L-3) | Upgradeable contract not initialized | 28 |
### <a name="L-1"></a>[L-1] Initializers could be front-run
Initializers could be front-run, allowing an attacker to either set their own values, take ownership of the contract, and in the best case forcing a re-deployment

*Instances (9)*:
```solidity
File: example/Test.sol

77:     function initialize(string memory name, string memory symbol, string memory tokenURI_, address pauserAddress)

79:         initializer

81:         __ERC721_init(name, symbol);

82:         __ERC721Enumerable_init();

83:         __UUPSUpgradeable_init();

84:         __ERC721Burnable_init();

85:         __ReentrancyGuard_init();

86:         __Pausable_init();

87:         __AccessControlDefaultAdminRules_init(0, msg.sender);

```

### <a name="L-2"></a>[L-2] Upgradeable contract is missing a `__gap[50]` storage variable to allow for new storage variables in later versions
See [this](https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps) link for a description of this storage variable. While some contracts may not currently be sub-classed, adding the variable now protects against forgetting to add it in the future.

*Instances (19)*:
```solidity
File: example/Test.sol

4: import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721EnumerableUpgradeable.sol";

5: import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

6: import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

7: import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721BurnableUpgradeable.sol";

8: import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";

9: import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

10: import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";

11: import "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";

15:     ERC721Upgradeable,

16:     ERC721EnumerableUpgradeable,

17:     ERC721BurnableUpgradeable,

18:     UUPSUpgradeable,

19:     ReentrancyGuardUpgradeable,

20:     PausableUpgradeable,

21:     AccessControlDefaultAdminRulesUpgradeable

83:         __UUPSUpgradeable_init();

235:         override(ERC721Upgradeable, ERC721EnumerableUpgradeable, AccessControlDefaultAdminRulesUpgradeable)

251:         override(ERC721Upgradeable, ERC721EnumerableUpgradeable)

267:         override(ERC721Upgradeable, ERC721EnumerableUpgradeable)

```

### <a name="L-3"></a>[L-3] Upgradeable contract not initialized
Upgradeable contracts are initialized via an initializer function rather than by a constructor. Leaving such a contract uninitialized may lead to it being taken over by a malicious user

*Instances (28)*:
```solidity
File: example/Test.sol

4: import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721EnumerableUpgradeable.sol";

5: import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

6: import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

7: import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721BurnableUpgradeable.sol";

8: import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";

9: import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

10: import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";

11: import "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";

15:     ERC721Upgradeable,

16:     ERC721EnumerableUpgradeable,

17:     ERC721BurnableUpgradeable,

18:     UUPSUpgradeable,

19:     ReentrancyGuardUpgradeable,

20:     PausableUpgradeable,

21:     AccessControlDefaultAdminRulesUpgradeable

74:         _disableInitializers();

77:     function initialize(string memory name, string memory symbol, string memory tokenURI_, address pauserAddress)

79:         initializer

81:         __ERC721_init(name, symbol);

82:         __ERC721Enumerable_init();

83:         __UUPSUpgradeable_init();

84:         __ERC721Burnable_init();

85:         __ReentrancyGuard_init();

86:         __Pausable_init();

87:         __AccessControlDefaultAdminRules_init(0, msg.sender);

235:         override(ERC721Upgradeable, ERC721EnumerableUpgradeable, AccessControlDefaultAdminRulesUpgradeable)

251:         override(ERC721Upgradeable, ERC721EnumerableUpgradeable)

267:         override(ERC721Upgradeable, ERC721EnumerableUpgradeable)

```


## Medium Issues


| |Issue|Instances|
|-|:-|:-:|
| [M-1](#M-1) | Centralization Risk for trusted owners | 7 |
| [M-2](#M-2) | Direct `supportsInterface()` calls may cause caller to revert | 1 |
### <a name="M-1"></a>[M-1] Centralization Risk for trusted owners

#### Impact:
Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.

*Instances (7)*:
```solidity
File: example/Test.sol

106:     function mint() external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant whenNotPaused {

175:     function adminBurn(uint256 tokenId) external onlyRole(DEFAULT_ADMIN_ROLE) nonReentrant whenNotPaused {

183:     function setTokenURI(string memory newTokenURI) external onlyRole(DEFAULT_ADMIN_ROLE) {

193:     function addDeveloper(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {

202:     function removeDeveloper(address account) external onlyRole(DEFAULT_ADMIN_ROLE) {

212:     function pause() external onlyRole(PAUSER_ROLE) {

216:     function unpause() external onlyRole(PAUSER_ROLE) {

```

### <a name="M-2"></a>[M-2] Direct `supportsInterface()` calls may cause caller to revert
Calling `supportsInterface()` on a contract that doesn't implement the ERC-165 standard will result in the call reverting. Even if the caller does support the function, the contract may be malicious and consume all of the transaction's available gas. Call it via a low-level [staticcall()](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/f959d7e4e6ee0b022b41e5b644c79369869d8411/contracts/utils/introspection/ERC165Checker.sol#L119), with a fixed amount of gas, and check the return code, or use OpenZeppelin's [`ERC165Checker.supportsInterface()`](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/f959d7e4e6ee0b022b41e5b644c79369869d8411/contracts/utils/introspection/ERC165Checker.sol#L36-L39).

*Instances (1)*:
```solidity
File: example/Test.sol

238:         return super.supportsInterface(interfaceId);

```


## High Issues


| |Issue|Instances|
|-|:-|:-:|
| [H-1](#H-1) | Using `delegatecall` inside a loop | 1 |
### <a name="H-1"></a>[H-1] Using `delegatecall` inside a loop

#### Impact:
When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.

*Instances (1)*:
```solidity
File: example/Test.sol

156:         for (uint256 i; i < data.length; i++) {

```

