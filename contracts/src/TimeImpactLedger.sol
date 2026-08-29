// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title TimeImpactLedger
 * @notice Immutable anchor for cryptographically sealed focus and impact receipts.
 * @dev Enforces duplicate hash prevention and integer-based accounting.
 */
contract TimeImpactLedger {
    address public owner;

    struct AnchorRecord {
        bytes32 evidenceHash;
        uint256 timestamp;
        address submitter;
        uint8 dataStatus; // 0: USER_ENTERED, 1: RECEIPT_BACKED, 2: METERED, 3: ATTESTED, 4: REGISTRY_VERIFIED
    }

    mapping(bytes32 => AnchorRecord) public records;
    mapping(bytes32 => bool) public isAnchored;

    event RecordAnchored(
        bytes32 indexed recordId,
        bytes32 indexed evidenceHash,
        address indexed submitter,
        uint8 dataStatus,
        uint256 timestamp
    );

    error RecordAlreadyExists();
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function anchorRecord(
        bytes32 recordId,
        bytes32 evidenceHash,
        uint8 dataStatus
    ) external {
        if (isAnchored[recordId]) revert RecordAlreadyExists();

        records[recordId] = AnchorRecord({
            evidenceHash: evidenceHash,
            timestamp: block.timestamp,
            submitter: msg.sender,
            dataStatus: dataStatus
        });

        isAnchored[recordId] = true;

        emit RecordAnchored(recordId, evidenceHash, msg.sender, dataStatus, block.timestamp);
    }
}
