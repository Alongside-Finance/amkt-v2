// SPDX-License-Identifier: MIT
pragma solidity =0.8.15;

/// O(1) insertion / deletion to an array
/// can also check inclusion
library VerifiableAddressArray {
    struct VerifiableArray {
        address[] elements;
        mapping(address => uint256) indexOf;
        // use an additaonal SSTORE to save a SLOAD at user runtime
        mapping(address => bool) included;
    }

    function size(VerifiableArray storage arr) internal view returns (uint256) {
        return arr.elements.length;
    }

    function add(VerifiableArray storage arr, address element) internal {
        if (includes(arr, element)) {
            revert("VerifiableArray: element already exists");
        } else {
            arr.included[element] = true;
            arr.indexOf[element] = arr.elements.length;
            arr.elements.push(element);
        }
    }

    function remove(VerifiableArray storage arr, address element) internal {
        if (!includes(arr, element)) {
            revert("VerifiableArray: element not found");
        }

        uint256 _size = size(arr);

        if (_size == 1) {
            delete arr.included[element];
            delete arr.indexOf[element];
            arr.elements.pop();
            return;
        }

        uint index = arr.indexOf[element];
        address lastElement = arr.elements[_size - 1];

        arr.indexOf[lastElement] = index;

        delete arr.included[element];
        delete arr.indexOf[element];

        arr.elements[index] = lastElement;
        arr.elements.pop();
    }

    function includes(
        VerifiableArray storage arr,
        address element
    ) internal view returns (bool) {
        return arr.included[element];
    }

    function toStorageArray(
        VerifiableArray storage arr
    ) internal view returns (address[] storage) {
        return arr.elements;
    }

    function toMemoryArray(
        VerifiableArray storage arr
    ) internal view returns (address[] memory) {
        address[] storage stor = toStorageArray(arr);
        uint256 len = stor.length;

        address[] memory mem = new address[](len);

        for (uint256 i; i < len; i++) {
            mem[i] = stor[i];
        }

        return mem;
    }
}
