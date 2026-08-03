use regex::Regex;

#[derive(Debug, Clone)]
pub struct BankInfo {
    pub bic_prefix: &'static str,
    pub country_code: &'static str,
    pub name: &'static str,
    pub country: &'static str,
}

pub static BANK_DATABASE: &[BankInfo] = &[
    // --- BULGARIA ---
    BankInfo { bic_prefix: "PIRB", country_code: "BG", name: "Piraeus Bank Bulgaria AD", country: "Bulgaria" },
    BankInfo { bic_prefix: "UNCR", country_code: "BG", name: "UniCredit Bulbank", country: "Bulgaria" },
    BankInfo { bic_prefix: "STSA", country_code: "BG", name: "DSK Bank AD", country: "Bulgaria" },
    BankInfo { bic_prefix: "FINV", country_code: "BG", name: "First Investment Bank (Fibank)", country: "Bulgaria" },
    BankInfo { bic_prefix: "UBBS", country_code: "BG", name: "United Bulgarian Bank (UBB)", country: "Bulgaria" },

    // --- UNITED KINGDOM ---
    BankInfo { bic_prefix: "MIDL", country_code: "GB", name: "HSBC Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "BARC", country_code: "GB", name: "Barclays Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "NWBK", country_code: "GB", name: "National Westminster Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "LOYD", country_code: "GB", name: "Lloyds Bank PLC", country: "United Kingdom" },
    BankInfo { bic_prefix: "SCBL", country_code: "GB", name: "Standard Chartered Bank", country: "United Kingdom" },

    // --- GERMANY ---
    BankInfo { bic_prefix: "DEUT", country_code: "DE", name: "Deutsche Bank AG", country: "Germany" },
    BankInfo { bic_prefix: "COBA", country_code: "DE", name: "Commerzbank AG", country: "Germany" },
    BankInfo { bic_prefix: "BYLA", country_code: "DE", name: "BayernLB", country: "Germany" },

    // --- FRANCE ---
    BankInfo { bic_prefix: "BNPA", country_code: "FR", name: "BNP Paribas", country: "France" },
    BankInfo { bic_prefix: "CRLY", country_code: "FR", name: "Crédit Lyonnais (LCL)", country: "France" },
    BankInfo { bic_prefix: "SOGE", country_code: "FR", name: "Société Générale", country: "France" },

    // --- UNITED STATES ---
    BankInfo { bic_prefix: "CHAS", country_code: "US", name: "JPMorgan Chase Bank N.A.", country: "United States" },
    BankInfo { bic_prefix: "CITI", country_code: "US", name: "Citibank N.A.", country: "United States" },
    BankInfo { bic_prefix: "BOFA", country_code: "US", name: "Bank of America N.A.", country: "United States" },

    // --- SWITZERLAND ---
    BankInfo { bic_prefix: "UBSW", country_code: "CH", name: "UBS AG", country: "Switzerland" },
    BankInfo { bic_prefix: "CRES", country_code: "CH", name: "Credit Suisse", country: "Switzerland" },

    // --- SPAIN & ITALY ---
    BankInfo { bic_prefix: "BSCH", country_code: "ES", name: "Banco Santander S.A.", country: "Spain" },
    BankInfo { bic_prefix: "BBVA", country_code: "ES", name: "Banco Bilbao Vizcaya Argentaria", country: "Spain" },
    BankInfo { bic_prefix: "ISPX", country_code: "IT", name: "Intesa Sanpaolo S.p.A.", country: "Italy" },

    // --- ASIA / UAE ---
    BankInfo { bic_prefix: "DBSS", country_code: "SG", name: "DBS Bank Ltd", country: "Singapore" },
    BankInfo { bic_prefix: "EBIL", country_code: "AE", name: "Emirates NBD Bank PJSC", country: "United Arab Emirates" },
    BankInfo { bic_prefix: "FABI", country_code: "AE", name: "First Abu Dhabi Bank", country: "United Arab Emirates" },
];

/// Validates SWIFT/BIC syntax structure (8 or 11 alphanumeric characters)
pub fn validate_swift(swift: &str) -> bool {
    let re = Regex::new(r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$").unwrap();
    re.is_match(swift)
}

/// MOD-97 IBAN Checksum Validator (ISO 7064)
pub fn validate_iban_checksum(iban: &str) -> bool {
    if iban.len() < 5 || iban.len() > 34 {
        return false;
    }

    let rearranged = format!("{}{}", &iban[4..], &iban[..4]);
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

/// Matches SWIFT/IBAN against registered bank dictionary entries
pub fn find_banks(swift: &str, iban: &str) -> Vec<&'static BankInfo> {
    let clean_swift = swift.to_uppercase();
    let clean_iban = iban.to_uppercase();

    let swift_prefix = if clean_swift.len() >= 4 { &clean_swift[..4] } else { "" };
    let iban_country = if clean_iban.len() >= 2 { &clean_iban[..2] } else { "" };

    BANK_DATABASE
        .iter()
        .filter(|b| {
            let swift_match = !swift_prefix.is_empty() && b.bic_prefix == swift_prefix;
            let country_match = !iban_country.is_empty() && b.country_code == iban_country;
            swift_match || (country_match && clean_swift.contains(b.bic_prefix))
        })
        .collect()
}

/// Generates a unique 32-character SHA256 identifier for MT103 downloads
pub fn generate_mt103_download_url(iban: &str, swift: &str) -> String {
    use sha2::{Digest, Sha256};
    let input = format!("{}:{}", iban, swift);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    format!("/mt103/download/{}", &hash[..32])
}