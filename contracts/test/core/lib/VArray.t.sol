pragma solidity =0.8.18;

import "forge-std/Test.sol";
import {VerifiableAddressArray} from "src/lib/VArray.sol";

contract VArrayTest is Test {
    using VerifiableAddressArray for VerifiableAddressArray.VerifiableArray;

    VerifiableAddressArray.VerifiableArray private arr;

    function testAddAndSize() public {
        address testAddress = address(0xdeadbeefdeadbeef);

        arr.add(testAddress);

        assertEq(arr.size(), 1);
        assertEq(arr.includes(testAddress), true);
    }

    function testRevertAddDuplicate() public {
        address testAddress = address(0xdeadbeefdeadbeef);

        arr.add(testAddress);
        vm.expectRevert("VerifiableArray: element already exists");
        arr.add(testAddress);
    }

    function testRemove() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);

        arr.add(testAddress1);
        arr.add(testAddress2);

        arr.remove(testAddress1);

        assertEq(arr.includes(testAddress1), false);
        assertEq(arr.size(), 1);
    }

    function testRevertRemoveNonExistent() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        vm.expectRevert("VerifiableArray: element not found");
        arr.remove(testAddress);
    }

    function testToMemoryArray() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);

        arr.add(testAddress1);
        arr.add(testAddress2);

        address[] memory memArr = arr.toMemoryArray();

        assertEq(memArr.length, 2);
        assertEq(memArr[0], testAddress1);
        assertEq(memArr[1], testAddress2);
    }

    function testEmptyAfterLastRemoval() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        arr.add(testAddress);
        arr.remove(testAddress);
        assert(arr.size() == 0);
        assert(!arr.includes(testAddress));
    }

    function testAdditionAfterRemoval() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        arr.add(testAddress);
        arr.remove(testAddress);
        arr.add(testAddress);
    }

    function testRemoveLastElement() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);

        arr.add(testAddress1);
        arr.add(testAddress2);

        arr.remove(testAddress2);

        assertEq(arr.size(), 1);
        assertEq(arr.includes(testAddress1), true);
        assertEq(!arr.includes(testAddress2), true);
    }

    function testOrderPreservation() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);
        address testAddress3 = address(0xabad1deabad1dea);

        arr.add(testAddress1);
        arr.add(testAddress2);
        arr.add(testAddress3);

        arr.remove(testAddress2); // This should replace testAddress2 with testAddress3 in the array

        address[] memory memArr = arr.toMemoryArray();

        assertEq(memArr.length, 2);
        assertEq(memArr[0], testAddress1);
        assertEq(memArr[1], testAddress3);
    }

    function testRevertNonExistentAddresses() public {
        address testAddress = address(0xdeadbeefdeadbeef);

        assertEq(!arr.includes(testAddress), true); // should return false
        vm.expectRevert("VerifiableArray: element not found");
        arr.remove(testAddress);
    }

    function testAddFuzz(address fuzzedAddress) public {
        arr.add(fuzzedAddress);
    }

    function testRemoveFuzz(address fuzzedAddress) public {
        arr.add(fuzzedAddress);
        arr.remove(fuzzedAddress);
    }

    function testAddRemoveMultipleAddresses() public {
        uint256 size = 100;
        address[] memory testAddresses = new address[](size);
        for (uint i = 0; i < size; i++) {
            testAddresses[i] = address(uint160(i));
            arr.add(testAddresses[i]);
        }
        assertEq(arr.size(), size);
        for (uint i = 0; i < size; i++) {
            assertEq(arr.includes(testAddresses[i]), true);
            arr.remove(testAddresses[i]);
            assertEq(arr.includes(testAddresses[i]), false);
        }
        assertEq(arr.size(), 0);
    }

    function testRemovePreviouslyRemoved() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        arr.add(testAddress);
        arr.remove(testAddress);
        vm.expectRevert("VerifiableArray: element not found");
        arr.remove(testAddress);
    }

    function testAddMaxAddress() public {
        address maxAddress = address(type(uint160).max);
        arr.add(maxAddress);
        assertEq(arr.size(), 1);
        assertEq(arr.includes(maxAddress), true);
    }

    // Test adding and removing addresses in a random order
    function testAddRemoveRandomOrder() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);
        address testAddress3 = address(0xabad1deabad1dea);
        arr.add(testAddress2);
        arr.add(testAddress1);
        arr.add(testAddress3);
        arr.remove(testAddress1);
        arr.remove(testAddress3);
        arr.remove(testAddress2);
        assertEq(arr.size(), 0);
    }

    // Test adding and removing the same address multiple times
    function testAddRemoveSameAddressMultipleTimes() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        for (uint i = 0; i < 5; i++) {
            arr.add(testAddress);
            arr.remove(testAddress);
        }
        assertEq(arr.size(), 0);
    }

    function testStressTest() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        for (uint i = 0; i < 1000; i++) {
            arr.add(testAddress);
            assertEq(arr.size(), 1);
            assertEq(arr.includes(testAddress), true);
            arr.remove(testAddress);
            assertEq(arr.size(), 0);
            assertEq(arr.includes(testAddress), false);
        }
    }

    // Test removing an address from an empty array
    function testRemoveFromEmptyArray() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        vm.expectRevert("VerifiableArray: element not found");
        arr.remove(testAddress);
    }

    // Test adding an address, removing it, and then checking if it's included
    function testAddRemoveCheck() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        arr.add(testAddress);
        arr.remove(testAddress);
        assertEq(arr.includes(testAddress), false);
    }

    function testAddRemoveDifferentCheckSize() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);
        arr.add(testAddress1);
        vm.expectRevert("VerifiableArray: element not found");
        arr.remove(testAddress2);
        assertEq(arr.size(), 1);
    }

    function testAddRemoveReverseOrder() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);
        arr.add(testAddress1);
        arr.add(testAddress2);
        arr.remove(testAddress1);
        arr.remove(testAddress2);
        assertEq(arr.size(), 0);
    }

    function testRemoveDifferentFromSingleElementArray() public {
        address testAddress1 = address(0xdeadbeefdeadbeef);
        address testAddress2 = address(0xcafebabecafebabe);
        arr.add(testAddress1);
        vm.expectRevert("VerifiableArray: element not found");
        arr.remove(testAddress2);
        assertEq(arr.size(), 1);
    }

    function testAddRemoveMaxAddress() public {
        address maxAddress = address(type(uint160).max);
        arr.add(maxAddress);
        assertEq(arr.size(), 1);
        assertEq(arr.includes(maxAddress), true);
        arr.remove(maxAddress);
        assertEq(arr.size(), 0);
        assertEq(arr.includes(maxAddress), false);
    }

    function testAddCheckRemoveCheck() public {
        address testAddress = address(0xdeadbeefdeadbeef);
        arr.add(testAddress);
        assertEq(arr.includes(testAddress), true);
        arr.remove(testAddress);
        assertEq(arr.includes(testAddress), false);
    }
}
