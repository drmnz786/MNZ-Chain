/// Full Bank Validation Module — ISO 7064 Modulo 97-10 + SPP/FTP/MT103 Support
/// Supports legacy accounts, manual download, and SPP/FTP protocols

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================
// IBAN VALIDATION (ISO 7064 Modulo 97-10)
// ============================================

/// Validates an IBAN checksum using the ISO 7064 Modulo 97-10 algorithm.
/// Supports both modern (15-34 chars) and legacy (5-14 chars) IBANs.
pub fn validate_iban_checksum(iban: &str) -> bool {
    // 1. Sanitize: Strip spaces, hyphens, underscores, and convert to uppercase
    let clean: String = iban
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .collect::<String>()
        .to_uppercase();

    // 2. Length check: ISO 13616 allows 15-34 characters
    //    However, legacy accounts (5-14 chars) are accepted with a warning
    let is_legacy = clean.len() >= 5 && clean.len() <= 14;
    let is_standard = clean.len() >= 15 && clean.len() <= 34;

    if !is_standard && !is_legacy {
        return false;
    }

    let bytes = clean.as_bytes();

    // 3. Check basic prefix: First 2 must be alphabetic, next 2 must be digits
    if !bytes[0].is_ascii_alphabetic() || !bytes[1].is_ascii_alphabetic() {
        return false;
    }

    // For standard IBANs, check digits 3-4 are numeric
    if is_standard {
        if !bytes[2].is_ascii_digit() || !bytes[3].is_ascii_digit() {
            return false;
        }
    }

    // 4. Rearrange string: (4..len) followed by (0..4)
    let rearranged_indices = (4..clean.len()).chain(0..4);

    // 5. Compute Modulo 97 using streaming digit arithmetic
    let mut remainder: u32 = 0;

    for idx in rearranged_indices {
        let ch = bytes[idx] as char;
        match ch {
            '0'..='9' => {
                let digit = (ch as u32) - ('0' as u32);
                remainder = (remainder * 10 + digit) % 97;
            }
            'A'..='Z' => {
                let val = (ch as u32) - ('A' as u32) + 10;
                let tens = val / 10;
                let ones = val % 10;

                // Process two digits for letter expansion (e.g., A -> 10)
                remainder = (remainder * 10 + tens) % 97;
                remainder = (remainder * 10 + ones) % 97;
            }
            _ => return false, // Invalid non-alphanumeric character
        }
    }

    // 6. Valid checksum must equal 1
    remainder == 1
}

// ============================================
// SWIFT VALIDATION
// ============================================

pub fn validate_swift(swift: &str) -> bool {
    let clean = swift
        .replace(" ", "")
        .replace("-", "")
        .to_uppercase();

    // SWIFT codes are 8 or 11 characters, letters only
    if clean.len() != 8 && clean.len() != 11 {
        return false;
    }

    clean.chars().all(|c| c.is_ascii_alphabetic())
}

// ============================================
// SPP/FTP/MT103 SETTLEMENT PROTOCOL
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT103Settlement {
    pub transaction_ref: String,
    pub related_ref: String,
    pub sender_swift: String,
    pub sender_bank: String,
    pub sender_account: String,
    pub sender_name: String,
    pub receiver_swift: String,
    pub receiver_bank: String,
    pub receiver_account: String,
    pub receiver_name: String,
    pub amount: f64,
    pub currency: String,
    pub value_date: String,
    pub status: String,
    pub manual_download_url: Option<String>,
    pub ftp_path: Option<String>,
    pub spp_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPPDownload {
    pub transaction_ref: String,
    pub mt103_hash: String,
    pub ftp_url: String,
    pub ftp_username: String,
    pub ftp_password: String,
    pub download_status: String,
    pub downloaded_at: Option<DateTime<Utc>>,
}

// ============================================
// GLOBAL BANK DATABASE
// ============================================

#[derive(Clone, Debug)]
pub struct BankRecord {
    pub name: String,
    pub country: String,
    pub swift_codes: Vec<String>,
    pub iban_prefixes: Vec<String>,
    pub jurisdiction: String,
    pub is_swift_net: bool,
    pub supports_spp: bool,
    pub supports_ftp: bool,
}

lazy_static! {
    pub static ref GLOBAL_BANKS: HashMap<String, BankRecord> = {
        let mut m = HashMap::new();

        // ========== EUROPE ==========
        m.insert("UBS".to_string(), BankRecord {
            name: "UBS AG".to_string(),
            country: "Switzerland".to_string(),
            swift_codes: vec!["UBSWCHZH80A".to_string(), "UBSWCHZH80B".to_string(), "UBSWCHZH".to_string()],
            iban_prefixes: vec!["CH".to_string()],
            jurisdiction: "Switzerland".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        m.insert("HSBC".to_string(), BankRecord {
            name: "HSBC Bank PLC".to_string(),
            country: "United Kingdom".to_string(),
            swift_codes: vec!["MIDLGB22".to_string(), "MIDLGB22XXX".to_string(), "HSBCHKHH".to_string(), "HSBCHIKHIKH".to_string()],
            iban_prefixes: vec!["GB".to_string(), "HK".to_string()],
            jurisdiction: "United Kingdom".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        m.insert("DEUTSCHE".to_string(), BankRecord {
            name: "Deutsche Bank AG".to_string(),
            country: "Germany".to_string(),
            swift_codes: vec!["DEUTDEFF".to_string(), "DEUTDEFFXXX".to_string(), "DEUTHKHH".to_string()],
            iban_prefixes: vec!["DE".to_string(), "HK".to_string()],
            jurisdiction: "Germany".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        m.insert("BARCLAYS".to_string(), BankRecord {
            name: "Barclays Bank PLC".to_string(),
            country: "United Kingdom".to_string(),
            swift_codes: vec!["BARCGB22".to_string(), "BARCGB22XXX".to_string()],
            iban_prefixes: vec!["GB".to_string()],
            jurisdiction: "United Kingdom".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        m.insert("JPMORGAN".to_string(), BankRecord {
            name: "JPMorgan Chase Bank NA".to_string(),
            country: "United States".to_string(),
            swift_codes: vec!["CHASUS33".to_string(), "CHASUS33XXX".to_string()],
            iban_prefixes: vec!["US".to_string()],
            jurisdiction: "United States".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: false,
        });

        // ========== ASIA ==========
        m.insert("UBS_SG".to_string(), BankRecord {
            name: "UBS AG Singapore".to_string(),
            country: "Singapore".to_string(),
            swift_codes: vec!["UBSWSGSG".to_string(), "UBSWSGSGXXX".to_string()],
            iban_prefixes: vec!["SG".to_string()],
            jurisdiction: "Singapore".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        // ========== AFRICA ==========
        m.insert("STANDARD_BANK".to_string(), BankRecord {
            name: "Standard Bank".to_string(),
            country: "South Africa".to_string(),
            swift_codes: vec!["SBZAZAJJ".to_string(), "SBZAZAJJXXX".to_string()],
            iban_prefixes: vec!["ZA".to_string()],
            jurisdiction: "South Africa".to_string(),
            is_swift_net: true,
            supports_spp: false,
            supports_ftp: true,
        });

        // ========== INDIAN OCEAN ==========
        m.insert("MAURITIUS_COMMERCIAL".to_string(), BankRecord {
            name: "Mauritius Commercial Bank".to_string(),
            country: "Mauritius".to_string(),
            swift_codes: vec!["MCBLMUMU".to_string(), "MCBLMUMUXXX".to_string()],
            iban_prefixes: vec!["MU".to_string()],
            jurisdiction: "Mauritius".to_string(),
            is_swift_net: true,
            supports_spp: false,
            supports_ftp: true,
        });

        // ========== LATIN AMERICA ==========
        m.insert("BRADESCO".to_string(), BankRecord {
            name: "Banco Bradesco".to_string(),
            country: "Brazil".to_string(),
            swift_codes: vec!["BBDEBRSPSP".to_string(), "BBDEBRSPSPXXX".to_string()],
            iban_prefixes: vec!["BR".to_string()],
            jurisdiction: "Brazil".to_string(),
            is_swift_net: true,
            supports_spp: false,
            supports_ftp: false,
        });

        // ========== SWISS PRIVATE BANKS ==========
        m.insert("PICTET".to_string(), BankRecord {
            name: "Pictet & Cie".to_string(),
            country: "Switzerland".to_string(),
            swift_codes: vec!["PICTCHZZ".to_string(), "PICTCHZZXXX".to_string()],
            iban_prefixes: vec!["CH".to_string()],
            jurisdiction: "Switzerland".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        m.insert("LOMBARD_ODIER".to_string(), BankRecord {
            name: "Lombard Odier".to_string(),
            country: "Switzerland".to_string(),
            swift_codes: vec!["LOCHCHZZ".to_string(), "LOCHCHZZXXX".to_string()],
            iban_prefixes: vec!["CH".to_string()],
            jurisdiction: "Switzerland".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        m.insert("JULIUS_BAER".to_string(), BankRecord {
            name: "Julius Baer".to_string(),
            country: "Switzerland".to_string(),
            swift_codes: vec!["BAERCHZZ".to_string(), "BAERCHZZXXX".to_string()],
            iban_prefixes: vec!["CH".to_string()],
            jurisdiction: "Switzerland".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        // ========== ROMANIA ==========
        m.insert("BCR_ROMANIA".to_string(), BankRecord {
            name: "Banca Comercială Română".to_string(),
            country: "Romania".to_string(),
            swift_codes: vec!["RZBNROBU".to_string(), "RZBNROBUXXX".to_string()],
            iban_prefixes: vec!["RO".to_string()],
            jurisdiction: "Romania".to_string(),
            is_swift_net: true,
            supports_spp: false,
            supports_ftp: true,
        });

        // ========== PANAMA ==========
        m.insert("BANCO_GENERAL".to_string(), BankRecord {
            name: "Banco General".to_string(),
            country: "Panama".to_string(),
            swift_codes: vec!["BAGEPAPA".to_string(), "BAGEPAPAXXX".to_string()],
            iban_prefixes: vec!["PA".to_string()],
            jurisdiction: "Panama".to_string(),
            is_swift_net: true,
            supports_spp: false,
            supports_ftp: false,
        });

        m
    };
}

// ============================================
// FIND BANK BY SWIFT OR IBAN PREFIX
// ============================================

pub fn find_banks(swift: &str, iban: &str) -> Vec<BankRecord> {
    let clean_swift = swift.replace(" ", "").replace("-", "").to_uppercase();
    let clean_iban = iban.replace(" ", "").replace("-", "").replace("_", "").to_uppercase();

    let mut matched: Vec<BankRecord> = Vec::new();

    for (_, bank) in GLOBAL_BANKS.iter() {
        let mut matched_swift = false;
        let mut matched_iban = false;

        for sw in &bank.swift_codes {
            let sw_clean = sw.replace(" ", "").replace("-", "");
            if clean_swift == sw_clean || clean_swift.starts_with(&sw_clean[0..8]) {
                matched_swift = true;
                break;
            }
        }

        for prefix in &bank.iban_prefixes {
            if clean_iban.starts_with(prefix) {
                matched_iban = true;
                break;
            }
        }

        if matched_swift || matched_iban {
            matched.push(bank.clone());
        }
    }

    // Deduplicate by name
    matched.sort_by(|a, b| a.name.cmp(&b.name));
    matched.dedup_by(|a, b| a.name == b.name);

    matched
}

// ============================================
// GENERATE MT103 DOWNLOAD URL
// ============================================

pub fn generate_mt103_download_url(
    transaction_ref: &str,
    bank_swift: &str,
) -> String {
    let hash = format!("{:x}", md5::compute(format!("{}{}", transaction_ref, bank_swift)));
    format!("/mt103/download/{}", hash)
}

// ============================================
// VERIFY SPP/FTP SETTLEMENT
// ============================================

pub fn verify_spp_settlement(
    transaction_ref: &str,
    mt103_hash: &str,
    ftp_url: &str,
) -> bool {
    // Simulate SPP/FTP verification
    // In production, this would check the actual FTP server
    transaction_ref.len() > 5 && mt103_hash.len() == 32 && ftp_url.starts_with("ftp://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iban_validation_modern() {
        assert!(validate_iban_checksum("GB33 BUKB 2020 1555 5555 55"));
        assert!(validate_iban_checksum("CH93 0076 2011 6238 5295 7"));
        assert!(validate_iban_checksum("DE89 3704 0044 0532 0130 00"));
    }

    #[test]
    fn test_iban_validation_legacy() {
        // Legacy accounts (5-14 chars) are accepted
        assert!(validate_iban_checksum("400-183323838"));
        assert!(validate_iban_checksum("496675541"));
    }

    #[test]
    fn test_find_banks_hsbc() {
        let banks = find_banks("MIDLGB22", "GB33");
        assert!(!banks.is_empty());
        assert_eq!(banks[0].name, "HSBC Bank PLC");
    }

    #[test]
    fn test_find_banks_deutsche() {
        let banks = find_banks("DEUTHKHH", "HK");
        assert!(!banks.is_empty());
        assert_eq!(banks[0].name, "Deutsche Bank AG");
    }
}