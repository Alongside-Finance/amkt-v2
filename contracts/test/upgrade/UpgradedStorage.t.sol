pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {InitialBountyHelper, MULTISIG, FEE_RECEIPIENT, INFLATION_RATE, AMKT_PROXY, PROXY_ADMIN} from "src/scripts/Config.sol";
import {TokenInfo} from "src/Common.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {UpgradedTest} from "test/upgrade/helpers/Upgraded.t.sol";
import {ITransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {fmul} from "src/lib/FixedPoint.sol";

// old layout
// cast storage 0x88f84864fd0839a7753199b01acb89c4714319f2 --rpc-url https://eth.llamarpc.com/
// | Name                | Type                                            | Slot | Offset | Bytes | Value                                                                         |
// |---------------------|-------------------------------------------------|------|--------|-------|-------------------------------------------------------------------------------|
// | _initialized        | uint8                                           | 0    | 0      | 1     | 1                                                                             |
// | _initializing       | bool                                            | 0    | 1      | 1     | 1                                                                             |
// | __gap               | uint256[50]                                     | 1    | 0      | 1600  | 0                                                                             |
// | _balances           | mapping(address => uint256)                     | 51   | 0      | 32    | 0                                                                             |
// | _allowances         | mapping(address => mapping(address => uint256)) | 52   | 0      | 32    | 0                                                                             |
// | _totalSupply        | uint256                                         | 53   | 0      | 32    | 0                                                                             |
// | _name               | string                                          | 54   | 0      | 32    | 47686211349250439718325492568846024043110354150518125751593386513137622056988 |
// | _symbol             | string                                          | 55   | 0      | 32    | 47686211349250439718325492568846024043110354150518125751593386513137622056988 |
// | __gap               | uint256[45]                                     | 56   | 0      | 1440  | 0                                                                             |
// | _owner              | address                                         | 101  | 0      | 20    | 833204653418720119172618286816296123974065860883                              |
// | __gap               | uint256[49]                                     | 102  | 0      | 1568  | 0                                                                             |
// | proposedOwner       | address                                         | 151  | 0      | 20    | 0                                                                             |
// | __gap               | uint256[50]                                     | 152  | 0      | 1600  | 0                                                                             |
// | _paused             | bool                                            | 202  | 0      | 1     | 0                                                                             |
// | __gap               | uint256[49]                                     | 203  | 0      | 1568  | 0                                                                             |
// | feeRatePerDayScaled | uint256                                         | 252  | 0      | 32    | 0                                                                             |
// | feeTimestamp        | uint256                                         | 253  | 0      | 32    | 1665698027                                                                    |
// | feeReceiver         | address                                         | 254  | 0      | 20    | 1                                                                             |
// | methodologist       | address                                         | 255  | 0      | 20    | 0                                                                             |
// | minter              | address                                         | 256  | 0      | 20    | 0                                                                             |
// | methodology         | string                                          | 257  | 0      | 32    | 0                                                                             |
// | supplyCeiling       | uint256                                         | 258  | 0      | 32    | 0                                                                             |
// | isRestricted        | mapping(address => bool)                        | 259  | 0      | 32    | 0

// new layout
// forge inspect IndexToken storage
// | Name                             | Type                                                          | Slot | Offset | Bytes |
// |----------------------------------|---------------------------------------------------------------|------|--------|-------|
// | _initialized                     | uint8                                                         | 0    | 0      | 1     |
// | _initializing                    | bool                                                          | 0    | 1      | 1     |
// | __gap                            | uint256[50]                                                   | 1    | 0      | 1600  |
// | _balances                        | mapping(address => uint256)                                   | 51   | 0      | 32    |
// | _allowances                      | mapping(address => mapping(address => uint256))               | 52   | 0      | 32    |
// | _totalSupply                     | uint256                                                       | 53   | 0      | 32    |
// | _name                            | string                                                        | 54   | 0      | 32    |
// | _symbol                          | string                                                        | 55   | 0      | 32    |
// | __gap                            | uint256[45]                                                   | 56   | 0      | 1440  |
// | _HASHED_NAME                     | bytes32                                                       | 101  | 0      | 32    | *COLLISION*
// | _HASHED_VERSION                  | bytes32                                                       | 102  | 0      | 32    | *COLLISION*
// | __gap                            | uint256[50]                                                   | 103  | 0      | 1600  |
// | _nonces                          | mapping(address => struct CountersUpgradeable.Counter)        | 153  | 0      | 32    |
// | _PERMIT_TYPEHASH_DEPRECATED_SLOT | bytes32                                                       | 154  | 0      | 32    |
// | __gap                            | uint256[49]                                                   | 155  | 0      | 1568  |
// | _delegates                       | mapping(address => address)                                   | 204  | 0      | 32    |
// | _checkpoints                     | mapping(address => struct ERC20VotesUpgradeable.Checkpoint[]) | 205  | 0      | 32    |
// | _totalSupplyCheckpoints          | struct ERC20VotesUpgradeable.Checkpoint[]                     | 206  | 0      | 32    |
// | __gap                            | uint256[47]                                                   | 207  | 0      | 1504  |                                                                        |
// | __gap                            | uint256[6]                                                    | 254  | 0      | 192   |

contract UpgradedStorageTest is UpgradedTest {
    function testStorageSlotUnstructured() public {
        assertEq(
            vm.load(address(AMKT), keccak256("Alongside::Token::MinterSlot")),
            bytes32(uint256(uint160(AMKT.minter())))
        );
    }

    //// SLOT 0
    // uint8 __initialized
    // bool _initializing
    function testStorageSlot0() public {
        // Define the boolean values for _initialized and _initializing
        bool initialized = true;
        bool initializing = false;

        // Combine the boolean values into a single bytes32 value
        bytes32 expectedValue = ((
            initialized ? bytes32(uint256(1)) : bytes32(uint256(0))
        ) | ((initializing ? bytes32(uint256(1)) : bytes32(uint256(0))) << 8));

        // Check the storage slot 0
        assertEq(vm.load(address(AMKT), _bytes(0)), expectedValue);
    }

    //// SLOT 1-50
    // uint256[50] __gap

    //// SLOT 51
    // mapping(address => uint256) _balances
    function testStorageSlot51() public {
        // address[] memory addressesToCheck = new address[](2);
        // uint256[] memory balancesToCheck = new uint256[](2);
        // addressesToCheck[0] = address(
        //     0x209ADBAad63c3008B5C2edb941B991Ef9Bb35027
        // );
        // balancesToCheck[0] = 2275e17;
        // addressesToCheck[1] = address(
        //     0x5c90090405d0dFfe53F385925E7F0DA064C4CA05
        // );
        // balancesToCheck[1] = 100e18;
        // // Iterate through the arrays and check the balances
        // for (uint256 i = 0; i < addressesToCheck.length; i++) {
        //     address userAddress = addressesToCheck[i];
        //     uint256 expectedBalance = balancesToCheck[i];
        //     assertEq(
        //         vm.load(address(AMKT), _derive(userAddress, uint256(51))),
        //         _bytes(expectedBalance)
        //     );
        // }
    }

    //// SLOT 52
    // mapping(address => mapping(address => uint256)) _allowances
    function testStorageSlot52() public {
        // Define the list of addresses and their corresponding allowances
        address[] memory owners = new address[](1);
        address[] memory spenders = new address[](1);
        uint256[] memory allowances = new uint256[](1);
        owners[0] = address(0x155d1164FF74eaC667Dd2136Aee881A1381DC764);
        spenders[0] = address(0xa3A7B6F88361F48403514059F1F16C8E78d60EeC);
        allowances[0] = 375028980941933331255;

        // Iterate through the list of addresses and check the allowances
        for (uint256 i = 0; i < owners.length; i++) {
            // Access the storage slot for the outer mapping
            bytes32 outerMappingSlot = _derive(owners[i], uint256(52));

            // Access the storage slot for the inner mapping
            bytes32 innerMappingSlot = keccak256(
                abi.encode(spenders[i], outerMappingSlot)
            );

            // Check the allowance value
            uint256 allowance = uint256(
                vm.load(address(AMKT), innerMappingSlot)
            );
            assertEq(allowance, allowances[i]);
        }
    }

    //// SLOT 53
    // uint256 _totalSupply
    function testStorageSlot53() public {
        assertEq(
            vm.load(address(AMKT), _bytes(53)),
            _bytes(AMKT.totalSupply())
        );
    }

    //// SLOT 54
    // string _name
    function testStorageSlot54() public {
        assertEq(
            vm.load(address(AMKT), _bytes(54)),
            _stringToBytes32(AMKT.name())
        );
    }

    //// SLOT 55
    // string _symbol
    function testStorageSlot55() public {
        assertEq(
            vm.load(address(AMKT), _bytes(55)),
            _stringToBytes32(AMKT.symbol())
        );
    }

    //// SLOT 56-100
    // uint256[50] __gap

    //// SLOT 101
    // bytes32 _HASHED_NAME
    function testStorageSlot101() public {
        assertEq(
            vm.load(address(AMKT), bytes32(uint256(101))),
            keccak256(bytes(AMKT.name()))
        );
    }

    //// SLOT 102
    // bytes32 _HASHED_VERSION
    // bytes32 _HASHED_NAME
    function testStorageSlot102() public {
        assertEq(
            vm.load(address(AMKT), bytes32(uint256(102))),
            keccak256(bytes("2"))
        );
    }

    //// SLOT 103-152
    // uint256[50] __gap

    //// SLOT 153
    // mapping(address => struct CountersUpgradeable.Counter) _nonces
    /// forge-config: default.fuzz.runs = 20
    function testStorageSlot153(address user) public {
        uint256 expectedNonce = 0;

        // Access the storage slot for the mapping
        bytes32 mappingSlot = _derive(user, uint256(153));

        // Access the storage slot for the Counter struct
        bytes32 counterSlot = keccak256(abi.encodePacked(mappingSlot));

        // Check the value of the Counter struct
        uint256 nonce = uint256(vm.load(address(AMKT), counterSlot));
        assertEq(nonce, expectedNonce);
    }

    //// SLOT 154
    // bytes32 _PERMIT_TYPEHASH_DEPRECATED_SLOT
    function testStorageSlot154() public {
        assertEq(vm.load(address(AMKT), _bytes(154)), _bytes(0));
    }

    //// SLOT 155-203
    // uint256[49] __gap

    //// SLOT 204
    // mapping(address => address) _delegates
    /// forge-config: default.fuzz.runs = 20
    function testStorageSlot204(address user) public {
        assertEq(vm.load(address(AMKT), _derive(user, 204)), _bytes(0));
    }

    //// SLOT 205
    // mapping(address => struct ERC20VotesUpgradeable.Checkpoint[]) _checkpoints
    /// forge-config: default.fuzz.runs = 20
    function testStorageSlot205(address user) public {
        uint256[] memory expectedFromBlocks = new uint256[](2);
        uint256[] memory expectedVotes = new uint256[](2);
        expectedFromBlocks[0] = 0;
        expectedVotes[0] = 0;
        expectedFromBlocks[1] = 0;
        expectedVotes[1] = 0;

        // Access the storage slot for the mapping
        bytes32 mappingSlot = _derive(user, uint256(205));

        // Access the storage slot for the array of Checkpoint structs
        bytes32 arraySlot = keccak256(abi.encodePacked(mappingSlot));

        // Iterate through the array of Checkpoint structs and check their values
        for (uint256 i = 0; i < expectedFromBlocks.length; i++) {
            bytes32 checkpointSlot = bytes32(uint256(arraySlot) + i);
            uint256 fromBlock = uint256(
                vm.load(address(AMKT), checkpointSlot)
            ) & 0xFFFFFFFF;
            uint256 votes = uint256(vm.load(address(AMKT), checkpointSlot)) >>
                32;
            assertEq(fromBlock, expectedFromBlocks[i]);
            assertEq(votes, expectedVotes[i]);
        }
    }

    //// SLOT 206
    // struct ERC20VotesUpgradeable.Checkpoint[] _totalSupplyCheckpoints
    function testStorageSlot206() public {
        // bytes32 totalSupplyCheckpointsHash = keccak256(
        //     abi.encodePacked(uint256(206))
        // );
        // assertEq(
        //     vm.load(address(AMKT), totalSupplyCheckpointsHash),
        //     (bytes32(batchExecutionBlock - 1)) |
        //         (bytes32(AMKT.totalSupply()) << 32)
        // ); // block.number is casted to 32 bits by SafeCastUpgradeable.toUint32
        // assertEq(
        //     vm.load(address(AMKT), totalSupplyCheckpointsHash),
        //     bytes32(
        //         abi.encodePacked(
        //             uint224(AMKT.totalSupply()),
        //             uint32(batchExecutionBlock - 1)
        //         )
        //     )
        // ); // check using encodePacked too
        // for (uint256 i = 0; i < 5; i++) {
        //     assertEq(
        //         vm.load(
        //             address(AMKT),
        //             bytes32(uint256(totalSupplyCheckpointsHash) + i + 1)
        //         ),
        //         bytes32(uint256(0))
        //     );
        // } // check 5 more indices to make sure they are all zero
    }

    //// SLOT 207 - 253
    // uint256[47] __gap

    /// SLOT 254 - 259
    // uint256[6] __gap

    function _bytes(uint256 x) internal pure returns (bytes32) {
        return bytes32(uint256(x));
    }

    function _derive(
        address key,
        uint256 slot
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(key, slot));
    }

    function _stringToBytes32(
        string memory source
    ) public returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        uint256 length = tempEmptyStringTest.length;
        assertLt(length, 32); // this fn only works for strings less than length 32
        if (length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
        // Set the last byte as the length of the string * 2
        result |= bytes32(length * 2);
    }
}
