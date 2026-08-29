// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title ImpactReceiptRegistry
 * @notice On-chain cryptographic receipt and attestation registry.
 * @dev Records evidence hashes, methodology hashes, and prevents duplicate receipt claims.
 *      IMPORTANT: This contract anchors evidence; it DOES NOT declare a carbon credit exists.
 */
contract ImpactReceiptRegistry {
    address public owner;

    struct ReceiptRecord {
        bytes32 evidenceRootHash;
        bytes32 methodologyVersionHash;
        address attester;
        uint256 timestamp;
        uint8 evidenceStatus;
    }

    mapping(bytes32 => ReceiptRecord) public receipts;
    mapping(bytes32 => bool) public isReceiptClaimed;

    event ReceiptRecorded(
        bytes32 indexed receiptId,
        bytes32 indexed evidenceRootHash,
        bytes32 methodologyVersionHash,
        address indexed attester,
        uint8 evidenceStatus,
        uint256 timestamp
    );

    error ReceiptAlreadyClaimed();
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function recordReceipt(
        bytes32 receiptId,
        bytes32 evidenceRootHash,
        bytes32 methodologyVersionHash,
        uint8 evidenceStatus
    ) external {
        if (isReceiptClaimed[receiptId]) revert ReceiptAlreadyClaimed();

        receipts[receiptId] = ReceiptRecord({
            evidenceRootHash: evidenceRootHash,
            methodologyVersionHash: methodologyVersionHash,
            attester: msg.sender,
            timestamp: block.timestamp,
            evidenceStatus: evidenceStatus
        });

        isReceiptClaimed[receiptId] = true;

        emit ReceiptRecorded(
            receiptId,
            evidenceRootHash,
            methodologyVersionHash,
            msg.sender,
            evidenceStatus,
            block.timestamp
        );
    }
}
