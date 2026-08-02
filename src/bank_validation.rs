/// Full Bank Validation Module — ISO 7064 Modulo 97-10 + SPP/FTP/MT103 Support
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================
// IBAN VALIDATION (ISO 7064 Modulo 97-10)
// ============================================

pub fn validate_iban_checksum(iban: &str) -> bool {
    let clean: String = iban
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .collect::<String>()
        .to_uppercase();

    let is_legacy = clean.len() >= 5 && clean.len() <= 14;
    let is_standard = clean.len() >= 15 && clean.len() <= 34;

    if !is_standard && !is_legacy {
        return false;
    }

    let bytes = clean.as_bytes();

    if !bytes[0].is_ascii_alphabetic() || !bytes[1].is_ascii_alphabetic() {
        return false;
    }

    if is_standard && (!bytes[2].is_ascii_digit() || !bytes[3].is_ascii_digit()) {
        return false;
    }

    let rearranged_indices = (4..clean.len()).chain(0..4);
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
                remainder = (remainder * 10 + tens) % 97;
                remainder = (remainder * 10 + ones) % 97;
            }
            _ => return false,
        }
    }

    remainder == 1
}

pub fn validate_swift(swift: &str) -> bool {
    let clean = swift.replace(' ', "").replace('-', "").to_uppercase();
    if clean.len() != 8 && clean.len() != 11 {
        return false;
    }
    // SWIFT/BIC codes can contain alphanumeric characters (e.g. MIDLGB22)
    clean.chars().all(|c| c.is_ascii_alphanumeric())
}

// ============================================
// BANK RECORD
// ============================================

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

// ============================================
// GLOBAL BANK DATABASE
// ============================================

lazy_static! {
    pub static ref GLOBAL_BANKS: HashMap<String, BankRecord> = {
        let mut m = HashMap::new();

        // HSBC
        m.insert("HSBC".to_string(), BankRecord {
            name: "HSBC Bank PLC".to_string(),
            country: "United Kingdom".to_string(),
            swift_codes: vec!["MIDLGB22".to_string(), "MIDLGB22XXX".to_string(), "HSBCHKHH".to_string()],
            iban_prefixes: vec!["GB".to_string(), "HK".to_string()],
            jurisdiction: "United Kingdom".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        // Deutsche
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

        // UBS
        m.insert("UBS".to_string(), BankRecord {
            name: "UBS AG".to_string(),
            country: "Switzerland".to_string(),
            swift_codes: vec!["UBSWCHZH80A".to_string(), "UBSWCHZH".to_string()],
            iban_prefixes: vec!["CH".to_string()],
            jurisdiction: "Switzerland".to_string(),
            is_swift_net: true,
            supports_spp: true,
            supports_ftp: true,
        });

        // Barclays
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

        // JPMorgan
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

        m
    };
}

// ============================================
// FIND BANKS
// ============================================

pub fn find_banks(swift: &str, iban: &str) -> Vec<BankRecord> {
    let clean_swift = swift.replace(' ', "").replace('-', "").to_uppercase();
    let clean_iban = iban.replace(' ', "").replace('-', "").replace('_', "").to_uppercase();

    let mut matched: Vec<BankRecord> = Vec::new();

    for (_, bank) in GLOBAL_BANKS.iter() {
        let mut matched_swift = false;
        let mut matched_iban = false;

        for sw in &bank.swift_codes {
            let sw_clean = sw.replace(' ', "").replace('-', "");
            if clean_swift == sw_clean || (clean_swift.len() >= 8 && sw_clean.len() >= 8 && clean_swift[..8] == sw_clean[..8]) {
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

    matched.sort_by(|a, b| a.name.cmp(&b.name));
    matched.dedup_by(|a, b| a.name == b.name);
    matched
}

// ============================================
// GENERATE MT103 DOWNLOAD URL
// ============================================

pub fn generate_mt103_download_url(transaction_ref: &str, bank_swift: &str) -> String {
    let hash = format!("{:x}", md5::compute(format!("{}{}", transaction_ref, bank_swift)));
    format!("/mt103/download/{}", hash)
}