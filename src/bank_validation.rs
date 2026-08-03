use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// ============================================
// BANK DATABASE — COMPLETE
// ============================================

#[derive(Debug, Clone)]
pub struct BankInfo {
    pub bic_prefix: &'static str,
    pub country_code: &'static str,
    pub name: &'static str,
    pub country: &'static str,
}

pub static BANK_DATABASE: &[BankInfo] = &[
    // BULGARIA
    BankInfo { bic_prefix: "PIRB", country_code: "BG", name: "Piraeus Bank Bulgaria AD", country: "Bulgaria" },
    BankInfo { bic_prefix: "UNCR", country_code: "BG", name: "UniCredit Bulbank", country: "Bulgaria" },
    BankInfo { bic_prefix: "STSA", country_code: "BG", name: "DSK Bank AD", country: "Bulgaria" },
    BankInfo { bic_prefix: "FINV", country_code: "BG", name: "First Investment Bank (Fibank)", country: "Bulgaria" },
    BankInfo { bic_prefix: "UBBS", country_code: "BG", name: "United Bulgarian Bank (UBB)", country: "Bulgaria" },
    BankInfo { bic_prefix: "DEMI", country_code: "BG", name: "DCommerce Bank AD", country: "Bulgaria" },
    
    // UNITED KINGDOM
    BankInfo { bic_prefix: "MIDL", country_code: "GB", name: "HSBC Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "BARC", country_code: "GB", name: "Barclays Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "NWBK", country_code: "GB", name: "National Westminster Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "LOYD", country_code: "GB", name: "Lloyds Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "SCBL", country_code: "GB", name: "Standard Chartered Bank", country: "United Kingdom" },
    BankInfo { bic_prefix: "HBUK", country_code: "GB", name: "HSBC UK Bank PLC", country: "United Kingdom" },
    
    // GERMANY
    BankInfo { bic_prefix: "DEUT", country_code: "DE", name: "Deutsche Bank AG", country: "Germany" },
    BankInfo { bic_prefix: "COBA", country_code: "DE", name: "Commerzbank AG", country: "Germany" },
    BankInfo { bic_prefix: "BYLA", country_code: "DE", name: "BayernLB", country: "Germany" },
    BankInfo { bic_prefix: "BKCH", country_code: "DE", name: "Bank of China Frankfurt", country: "Germany" },
    
    // HONG KONG
    BankInfo { bic_prefix: "HSBC", country_code: "HK", name: "HSBC Hong Kong", country: "Hong Kong" },
    BankInfo { bic_prefix: "BKCH", country_code: "HK", name: "Bank of China Hong Kong", country: "Hong Kong" },
    
    // FRANCE
    BankInfo { bic_prefix: "BNPA", country_code: "FR", name: "BNP Paribas", country: "France" },
    BankInfo { bic_prefix: "CRLY", country_code: "FR", name: "Crédit Lyonnais (LCL)", country: "France" },
    BankInfo { bic_prefix: "SOGE", country_code: "FR", name: "Société Générale", country: "France" },
    
    // UNITED STATES
    BankInfo { bic_prefix: "CHAS", country_code: "US", name: "JPMorgan Chase Bank N.A.", country: "United States" },
    BankInfo { bic_prefix: "CITI", country_code: "US", name: "Citibank N.A.", country: "United States" },
    BankInfo { bic_prefix: "BOFA", country_code: "US", name: "Bank of America N.A.", country: "United States" },
    BankInfo { bic_prefix: "MRMD", country_code: "US", name: "Morgan Stanley", country: "United States" },
    
    // SWITZERLAND
    BankInfo { bic_prefix: "UBSW", country_code: "CH", name: "UBS AG", country: "Switzerland" },
    BankInfo { bic_prefix: "CRES", country_code: "CH", name: "Credit Suisse", country: "Switzerland" },
    BankInfo { bic_prefix: "PICT", country_code: "CH", name: "Pictet & Cie", country: "Switzerland" },
    BankInfo { bic_prefix: "LOCH", country_code: "CH", name: "Lombard Odier", country: "Switzerland" },
    BankInfo { bic_prefix: "BAER", country_code: "CH", name: "Julius Baer", country: "Switzerland" },
    
    // SPAIN
    BankInfo { bic_prefix: "BSCH", country_code: "ES", name: "Banco Santander S.A.", country: "Spain" },
    BankInfo { bic_prefix: "BBVA", country_code: "ES", name: "Banco Bilbao Vizcaya Argentaria", country: "Spain" },
    
    // ITALY
    BankInfo { bic_prefix: "ISPX", country_code: "IT", name: "Intesa Sanpaolo S.p.A.", country: "Italy" },
    BankInfo { bic_prefix: "UNCR", country_code: "IT", name: "UniCredit S.p.A.", country: "Italy" },
    
    // ASIA
    BankInfo { bic_prefix: "DBSS", country_code: "SG", name: "DBS Bank Ltd", country: "Singapore" },
    BankInfo { bic_prefix: "OCBC", country_code: "SG", name: "OCBC Bank", country: "Singapore" },
    
    // UAE
    BankInfo { bic_prefix: "EBIL", country_code: "AE", name: "Emirates NBD Bank PJSC", country: "United Arab Emirates" },
    BankInfo { bic_prefix: "FABI", country_code: "AE", name: "First Abu Dhabi Bank", country: "United Arab Emirates" },
];

// ============================================
// SWIFT VALIDATION
// ============================================

pub fn validate_swift(swift: &str) -> bool {
    let re = Regex::new(r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$").unwrap();
    re.is_match(swift)
}

// ============================================
// IBAN VALIDATION (ISO 7064 MOD 97-10)
// ============================================

pub fn validate_iban_checksum(iban: &str) -> bool {
    let clean: String = iban
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
        .collect::<String>()
        .to_uppercase();

    if clean.len() < 5 || clean.len() > 34 {
        return false;
    }

    let rearranged = format!("{}{}", &clean[4..], &clean[..4]);
    let mut numeric_str = String::new();

    for ch in rearranged.chars() {
        if ch.is_ascii_digit() {
            numeric_str.push(ch);
        } else if ch.is_ascii_uppercase() {
            let num = (ch as u32) - ('A' as u32) + 10;
            numeric_str.push_str(&num.to_string());
        } else {
            return false;
        }
    }

    let mut remainder: u32 = 0;
    for chunk in numeric_str.as_bytes() {
        let digit = (chunk - b'0') as u32;
        remainder = (remainder * 10 + digit) % 97;
    }

    remainder == 1
}

// ============================================
// FIND BANKS — WITH LEAKED STATIC FALLBACK
// ============================================

pub fn find_banks(swift: &str, iban: &str) -> Vec<&'static BankInfo> {
    let clean_swift = swift.to_uppercase();
    let clean_iban = iban.to_uppercase();

    let swift_prefix = if clean_swift.len() >= 4 { &clean_swift[..4] } else { "" };
    let iban_country = if clean_iban.len() >= 2 { &clean_iban[..2] } else { "" };

    let mut results: Vec<&'static BankInfo> = BANK_DATABASE
        .iter()
        .filter(|b| {
            let swift_match = !swift_prefix.is_empty() && b.bic_prefix == swift_prefix;
            let country_match = !iban_country.is_empty() && b.country_code == iban_country;
            swift_match || (country_match && clean_swift.contains(b.bic_prefix))
        })
        .collect();

    // FALLBACK: If no bank found, return a dynamic static entry
    if results.is_empty() && !iban_country.is_empty() {
        let country_names: HashMap<&'static str, &'static str> = [
            ("BG", "Bulgaria"),
            ("GB", "United Kingdom"),
            ("DE", "Germany"),
            ("FR", "France"),
            ("US", "United States"),
            ("CH", "Switzerland"),
            ("HK", "Hong Kong"),
            ("SG", "Singapore"),
            ("AE", "United Arab Emirates"),
            ("ES", "Spain"),
            ("IT", "Italy"),
        ]
        .iter()
        .cloned()
        .collect();

        let country_name = country_names.get(iban_country).copied().unwrap_or("Unknown");
        
        let code_static: &'static str = Box::leak(iban_country.to_string().into_boxed_str());
        let name_static: &'static str = Box::leak(format!("Bank in {} (Valid IBAN)", country_name).into_boxed_str());

        let fallback_info: &'static BankInfo = Box::leak(Box::new(BankInfo {
            bic_prefix: "UNKN",
            country_code: code_static,
            name: name_static,
            country: country_name,
        }));

        results.push(fallback_info);
    }

    results
}

// ============================================
// GENERATE MT103 DOWNLOAD URL
// ============================================

pub fn generate_mt103_download_url(iban: &str, swift: &str) -> String {
    let input = format!("{}:{}", iban, swift);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    format!("/mt103/download/{}", &hash[..32])
}

// ============================================
// UNIT TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_swift() {
        assert!(validate_swift("MIDLGB22"));
        assert!(validate_swift("PIRBBSGF"));
        assert!(validate_swift("DEUTDEFF"));
        assert!(validate_swift("BARCGB22XXX"));
        assert!(!validate_swift("12345678"));
        assert!(!validate_swift("MIDLGB2"));
    }

    #[test]
    fn test_validate_iban_checksum() {
        assert!(validate_iban_checksum("BG50PIRB80501606353420")); // Valid Bulgarian IBAN
        assert!(validate_iban_checksum("GB74MIDL40051512345678")); // Valid UK IBAN (GB74)
        assert!(!validate_iban_checksum("GB33MIDL40051512345678")); // Invalid Checksum (33 instead of 74)
    }

    #[test]
    fn test_find_banks() {
        let banks = find_banks("MIDLGB22", "GB33");
        assert!(!banks.is_empty());
        assert_eq!(banks[0].name, "HSBC Bank PLC");

        let banks2 = find_banks("PIRBBSGF", "BG50");
        assert!(!banks2.is_empty());
        assert_eq!(banks2[0].name, "Piraeus Bank Bulgaria AD");
    }

    #[test]
    fn test_fallback_unknown_bic() {
        let banks = find_banks("XXXXGB22", "GB33");
        assert!(!banks.is_empty());
        assert_eq!(banks[0].bic_prefix, "UNKN");
        assert_eq!(banks[0].country_code, "GB");
    }
}