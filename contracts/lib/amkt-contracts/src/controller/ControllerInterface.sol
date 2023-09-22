// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;
import "../factory/MembersInterface.sol";
import "../factory/FactoryInterface.sol";

interface ControllerInterface {
    event MembersSet(MembersInterface indexed members);
    event FactorySet(FactoryInterface indexed factory);
    event FactoryPause(bool pause);

    function mint(address to, uint256 amount) external returns (bool);

    function burn(address from, uint256 value) external returns (bool);

    function factoryPause(bool pause) external returns (bool);

    function isIssuer(address addr) external view returns (bool);

    function isMerchant(address addr) external view returns (bool);

    function getToken() external view returns (address);
}
