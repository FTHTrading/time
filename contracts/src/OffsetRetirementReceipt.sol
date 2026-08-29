// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title OffsetRetirementReceipt
 * @notice Soulbound non-transferable milestone receipts for verified carbon retirement certificates.
 */
contract OffsetRetirementReceipt {
    string public name = "ACNC Carbon Retirement Receipt";
    string public symbol = "ACNC-OFFSET";

    address public owner;
    uint256 public nextTokenId = 1;

    struct Certificate {
        string registry;
        string serialNumber;
        uint256 tonnesCo2e;
        bytes32 evidenceHash;
        uint256 timestamp;
    }

    mapping(uint256 => address) public tokenOwners;
    mapping(uint256 => Certificate) public certificates;
    mapping(bytes32 => bool) public registeredSerials;

    event CertificateMinted(
        uint256 indexed tokenId,
        address indexed recipient,
        string registry,
        string serialNumber,
        uint256 tonnesCo2e
    );

    error NonTransferable();
    error SerialAlreadyRegistered();
    error Unauthorized();

    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function mintRetirement(
        address recipient,
        string calldata registry,
        string calldata serialNumber,
        uint256 tonnesCo2e,
        bytes32 evidenceHash
    ) external onlyOwner returns (uint256) {
        bytes32 serialHash = keccak256(abi.encodePacked(registry, serialNumber));
        if (registeredSerials[serialHash]) revert SerialAlreadyRegistered();

        uint256 tokenId = nextTokenId++;
        tokenOwners[tokenId] = recipient;
        certificates[tokenId] = Certificate({
            registry: registry,
            serialNumber: serialNumber,
            tonnesCo2e: tonnesCo2e,
            evidenceHash: evidenceHash,
            timestamp: block.timestamp
        });

        registeredSerials[serialHash] = true;

        emit CertificateMinted(tokenId, recipient, registry, serialNumber, tonnesCo2e);
        return tokenId;
    }
}
