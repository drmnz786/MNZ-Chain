// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title MNZOtcDesk
 * @notice Institutional OTC Escrow Engine with MT103 SWIFT Hash Attestation
 */
contract MNZOtcDesk is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    enum DealStatus {
        Created,
        Locked,
        ProofSubmitted,
        Settled,
        Cancelled,
        Disputed
    }

    struct Deal {
        bytes32 dealId;
        address seller;
        address buyer;
        address tokenAddress;
        uint256 tokenAmount;
        uint256 fiatAmount;
        bytes3 currencyCode;
        bytes32 mt103ProofHash;
        uint256 createdAt;
        uint256 expiryTime;
        DealStatus status;
    }

    mapping(bytes32 => Deal) public deals;
    mapping(address => bool) public authorizedVerifiers;

    event DealCreated(bytes32 indexed dealId, address indexed seller, address indexed buyer, address tokenAddress, uint256 tokenAmount, uint256 fiatAmount, bytes3 currencyCode);
    event DealLocked(bytes32 indexed dealId, address indexed buyer);
    event ProofSubmitted(bytes32 indexed dealId, bytes32 mt103ProofHash);
    event DealSettled(bytes32 indexed dealId, address indexed recipient);
    event DealCancelled(bytes32 indexed dealId);
    event DisputeRaised(bytes32 indexed dealId);
    event VerifierUpdated(address indexed verifier, bool status);

    modifier onlyVerifier() {
        require(authorizedVerifiers[msg.sender] || owner() == msg.sender, "OTC: Unauthorized verifier");
        _;
    }

    constructor(address initialOwner) Ownable(initialOwner) {
        authorizedVerifiers[initialOwner] = true;
    }

    function setVerifier(address verifier, bool status) external onlyOwner {
        authorizedVerifiers[verifier] = status;
        emit VerifierUpdated(verifier, status);
    }

    function createDeal(
        bytes32 dealId,
        address buyer,
        address tokenAddress,
        uint256 tokenAmount,
        uint256 fiatAmount,
        bytes3 currencyCode,
        uint256 durationSeconds
    ) external nonReentrant {
        require(deals[dealId].dealId == bytes32(0), "OTC: Deal ID exists");
        require(tokenAmount > 0 && fiatAmount > 0, "OTC: Invalid amounts");
        require(durationSeconds >= 3600, "OTC: Expiry must be >= 1 hour");

        IERC20(tokenAddress).safeTransferFrom(msg.sender, address(this), tokenAmount);

        deals[dealId] = Deal({
            dealId: dealId,
            seller: msg.sender,
            buyer: buyer,
            tokenAddress: tokenAddress,
            tokenAmount: tokenAmount,
            fiatAmount: fiatAmount,
            currencyCode: currencyCode,
            mt103ProofHash: bytes32(0),
            createdAt: block.timestamp,
            expiryTime: block.timestamp + durationSeconds,
            status: DealStatus.Created
        });

        emit DealCreated(dealId, msg.sender, buyer, tokenAddress, tokenAmount, fiatAmount, currencyCode);
    }

    function lockDeal(bytes32 dealId) external {
        Deal storage deal = deals[dealId];
        require(deal.status == DealStatus.Created, "OTC: Deal not open");
        require(block.timestamp < deal.expiryTime, "OTC: Deal expired");
        if (deal.buyer != address(0)) {
            require(msg.sender == deal.buyer, "OTC: Designated buyer only");
        } else {
            deal.buyer = msg.sender;
        }
        deal.status = DealStatus.Locked;
        emit DealLocked(dealId, msg.sender);
    }

    function submitMt103Proof(bytes32 dealId, bytes32 mt103ProofHash) external {
        Deal storage deal = deals[dealId];
        require(deal.status == DealStatus.Locked, "OTC: Deal not locked");
        require(msg.sender == deal.buyer || authorizedVerifiers[msg.sender], "OTC: Not buyer or verifier");
        require(mt103ProofHash != bytes32(0), "OTC: Invalid proof hash");
        deal.mt103ProofHash = mt103ProofHash;
        deal.status = DealStatus.ProofSubmitted;
        emit ProofSubmitted(dealId, mt103ProofHash);
    }

    function settleDeal(bytes32 dealId) external nonReentrant {
        Deal storage deal = deals[dealId];
        require(
            deal.status == DealStatus.ProofSubmitted || deal.status == DealStatus.Locked,
            "OTC: Invalid state for settlement"
        );
        require(
            msg.sender == deal.seller || authorizedVerifiers[msg.sender],
            "OTC: Only seller or verifier can settle"
        );
        deal.status = DealStatus.Settled;
        IERC20(deal.tokenAddress).safeTransfer(deal.buyer, deal.tokenAmount);
        emit DealSettled(dealId, deal.buyer);
    }

    function cancelDeal(bytes32 dealId) external nonReentrant {
        Deal storage deal = deals[dealId];
        require(msg.sender == deal.seller || owner() == msg.sender, "OTC: Unauthorized");
        require(
            deal.status == DealStatus.Created || 
            (deal.status == DealStatus.Locked && block.timestamp >= deal.expiryTime),
            "OTC: Cannot cancel active locked deal"
        );
        deal.status = DealStatus.Cancelled;
        IERC20(deal.tokenAddress).safeTransfer(deal.seller, deal.tokenAmount);
        emit DealCancelled(dealId);
    }

    function raiseDispute(bytes32 dealId) external {
        Deal storage deal = deals[dealId];
        require(msg.sender == deal.seller || msg.sender == deal.buyer, "OTC: Not party to deal");
        require(deal.status == DealStatus.ProofSubmitted || deal.status == DealStatus.Locked, "OTC: Cannot dispute");
        deal.status = DealStatus.Disputed;
        emit DisputeRaised(dealId);
    }

    function resolveDispute(bytes32 dealId, address recipient) external onlyOwner nonReentrant {
        Deal storage deal = deals[dealId];
        require(deal.status == DealStatus.Disputed, "OTC: Deal not disputed");
        require(recipient == deal.seller || recipient == deal.buyer, "OTC: Invalid recipient");
        deal.status = DealStatus.Settled;
        IERC20(deal.tokenAddress).safeTransfer(recipient, deal.tokenAmount);
        emit DealSettled(dealId, recipient);
    }
}
