pragma solidity =0.8.15;

import "forge-std/Test.sol";
import {MULTISIG} from "src/scripts/Config.sol";

struct GnosisTransaction {
    address to;
    bytes data;
}

interface IGnosisSafe {
    function getThreshold() external view returns (uint256);

    function isOwner(address owner) external view returns (bool);

    function getOwners() external view returns (address[] memory);

    function execTransaction(
        address to,
        uint256 value,
        bytes memory data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        bytes memory signatures
    ) external payable returns (bool success);

    function encodeTransactionData(
        address to,
        uint256 value,
        bytes memory data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        uint256 _nonce
    ) external view returns (bytes memory);

    function nonce() external view returns (uint256);
}

interface IMultiSendCallOnly {
    /// Each transaction is encoded as a packed bytes of
    /// operation has to be uint8(0) in this version (=> 1 byte),
    /// to as a address (=> 20 bytes),
    /// value as a uint256 (=> 32 bytes),
    /// data length as a uint256 (=> 32 bytes),
    /// data as bytes.
    function multiSend(bytes memory transactions) external payable;
}

contract GnosisTest is Test {
    using stdStorage for StdStorage;
    IGnosisSafe safe = IGnosisSafe(MULTISIG);
    IMultiSendCallOnly multiSendCallOnly =
        IMultiSendCallOnly(0x40A2aCCbd92BCA938b02010E17A5b8929b49130D);

    function enableSimulation() public {
        address newOwner = vm.addr(0xB0B);
        vm.store(MULTISIG, bytes32(uint256(4)), bytes32(uint256(1))); // slot for threshold is 4
        assertEq(safe.getThreshold(), 1);
        address[] memory owners = safe.getOwners();
        bytes32 ownerData = vm.load(
            MULTISIG,
            keccak256(abi.encode(owners[0], 2)) // slot for owners is 2
        );
        // zero out previous owner
        vm.store(MULTISIG, keccak256(abi.encode(owners[0], 2)), bytes32(0));
        // swap in new owner
        vm.store(MULTISIG, keccak256(abi.encode(newOwner, 2)), ownerData);
        assertEq(safe.isOwner(newOwner), true);
    }

    function getSignature(
        address to,
        uint256 value,
        bytes memory data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        uint256 nonce
    ) public returns (bytes memory) {
        bytes memory txHashData = safe.encodeTransactionData(
            to,
            value,
            data,
            operation,
            safeTxGas,
            baseGas,
            gasPrice,
            gasToken,
            refundReceiver,
            nonce
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(0xB0B, keccak256(txHashData));
        bytes memory signature = abi.encodePacked(r, s, v);
        return signature;
    }

    function executeBatch(
        GnosisTransaction[] memory batch
    ) public returns (bytes memory) {
        bytes memory data = getBatchExecutionData(batch);
        executeData(address(multiSendCallOnly), 1, data);
        return data;
    }

    function getBatchExecutionData(
        GnosisTransaction[] memory batch
    ) public returns (bytes memory) {
        bytes memory transactions = new bytes(0);
        for (uint256 i = 0; i < batch.length; i++) {
            transactions = abi.encodePacked(
                transactions,
                uint8(0),
                batch[i].to,
                uint256(0), // value is assumed 0
                batch[i].data.length,
                batch[i].data
            );
        }
        // calldata for calling multiSend with transactions
        bytes memory data = abi.encodeWithSelector(
            multiSendCallOnly.multiSend.selector,
            transactions
        );
        return data;
    }

    function executeData(
        address to,
        uint8 operation,
        bytes memory data
    ) public {
        uint256 value = 0;
        uint256 safeTxGas = 0;
        uint256 baseGas = 0;
        uint256 gasPrice = 0;
        address gasToken = address(0);
        address refundReceiver = address(0);
        uint256 nonce = safe.nonce();
        bytes memory signature = getSignature(
            to,
            value,
            data,
            operation,
            safeTxGas,
            baseGas,
            gasPrice,
            gasToken,
            refundReceiver,
            nonce
        );
        vm.prank(vm.addr(0xB0B));
        safe.execTransaction(
            to,
            value,
            data,
            operation,
            safeTxGas,
            baseGas,
            gasPrice,
            gasToken,
            refundReceiver,
            signature
        );
    }
}
