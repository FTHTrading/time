// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title VTIMERewardVault
 * @notice Closed-loop internal utility credit vault with EIP-712 signature verification.
 */
contract VTIMERewardVault {
    string public constant name = "VTIME Utility Vault";
    string public constant symbol = "VTIME";
    uint8 public constant decimals = 18;

    address public owner;
    address public attester;

    uint256 public constant DAILY_ACCOUNT_CAP = 200 * 1e18;
    mapping(address => uint256) public balances;
    mapping(address => uint256) public nonces;
    mapping(bytes32 => bool) public executedClaims;

    bytes32 public DOMAIN_SEPARATOR;
    bytes32 public constant CLAIM_TYPEHASH = keccak256(
        "RewardClaim(address recipient,uint256 amount,bytes32 receiptId,bytes32 evidenceHash,uint256 nonce,uint256 deadline)"
    );

    event ClaimExecuted(
        address indexed recipient,
        uint256 amount,
        bytes32 indexed receiptId,
        bytes32 evidenceHash
    );

    error InvalidSignature();
    error ClaimExpired();
    error ClaimAlreadyExecuted();
    error ExceedsDailyCap();
    error Unauthorized();

    constructor(address _attester) {
        owner = msg.sender;
        attester = _attester;

        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes("1")),
                block.chainid,
                address(this)
            )
        );
    }

    function claimReward(
        uint256 amount,
        bytes32 receiptId,
        bytes32 evidenceHash,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        if (block.timestamp > deadline) revert ClaimExpired();
        if (executedClaims[receiptId]) revert ClaimAlreadyExecuted();
        if (amount > DAILY_ACCOUNT_CAP) revert ExceedsDailyCap();

        uint256 currentNonce = nonces[msg.sender]++;
        bytes32 structHash = keccak256(
            abi.encode(CLAIM_TYPEHASH, msg.sender, amount, receiptId, evidenceHash, currentNonce, deadline)
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, structHash));
        address signer = ecrecover(digest, v, r, s);

        if (signer != attester || signer == address(0)) revert InvalidSignature();

        executedClaims[receiptId] = true;
        balances[msg.sender] += amount;

        emit ClaimExecuted(msg.sender, amount, receiptId, evidenceHash);
    }
}
