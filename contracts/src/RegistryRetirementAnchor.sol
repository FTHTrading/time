// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title RegistryRetirementAnchor
 * @notice Anchors third-party carbon registry retirement certificates.
 * @dev Marks each serial as used once and prevents replay.
 *      IMPORTANT: The third-party registry (Gold Standard, Verra, Puro.earth) is the sole authoritative
 *      source of truth. This contract simply anchors that ACNC referenced a verified cancellation.
 */
contract RegistryRetirementAnchor {
    address public owner;

    struct RetirementAnchor {
        string registry;
        string serialNumber;
        uint256 tonnesCo2e;
        bytes32 evidenceHash;
        address beneficiary;
        uint256 timestamp;
    }

    mapping(bytes32 => RetirementAnchor) public anchors;
    mapping(bytes32 => bool) public isSerialUsed;

    event RetirementAnchored(
        bytes32 indexed serialHash,
        string registry,
        string serialNumber,
        uint256 tonnesCo2e,
        address indexed beneficiary,
        bytes32 evidenceHash,
        uint256 timestamp
    );

    error SerialAlreadyAnchored();
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function anchorRetirement(
        string calldata registry,
        string calldata serialNumber,
        uint256 tonnesCo2e,
        address beneficiary,
        bytes32 evidenceHash
    ) external onlyOwner returns (bytes32) {
        bytes32 serialHash = keccak256(abi.encodePacked(registry, serialNumber));
        if (isSerialUsed[serialHash]) revert SerialAlreadyAnchored();

        anchors[serialHash] = RetirementAnchor({
            registry: registry,
            serialNumber: serialNumber,
            tonnesCo2e: tonnesCo2e,
            evidenceHash: evidenceHash,
            beneficiary: beneficiary,
            timestamp: block.timestamp
        });

        isSerialUsed[serialHash] = true;

        emit RetirementAnchored(
            serialHash,
            registry,
            serialNumber,
            tonnesCo2e,
            beneficiary,
            evidenceHash,
            block.timestamp
        );

        return serialHash;
    }
}
