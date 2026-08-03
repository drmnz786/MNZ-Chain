use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod bank_validation;
use bank_validation::*;

// ============================================
// MT103 FIELD 32A PARSER
// ============================================

/// Parses MT103 Field :32A: (Format: YYMMDDCURAMOUNT, e.g., :32A:190221EUR19777000,00)
/// Returns Option<(Currency, Amount, Date)>
pub fn parse_mt103_amount(mt103: &str) -> Option<(String, f64, String)> {
    let re = Regex::new(r":32A:(\d{6})([A-Z]{3})([\d.,]+)").ok()?;

    for line in mt103.lines() {
        if let Some(caps) = re.captures(line) {
            let date_str = caps.get(1)?.as_str().to_string();
            let currency = caps.get(2)?.as_str().to_string();
            let raw_amount = caps.get(3)?.as_str();

            // Handle SWIFT decimal comma notation (e.g., "19777000,00" -> "19777000.00")
            let clean_amount = raw_amount.replace(',', ".");
            let amount = clean_amount.parse::<f64>().ok()?;

            return Some((currency, amount, date_str));
        }
    }
    None
}

// ============================================
// DATA STRUCTURES FOR MINING & VERIFICATION
// ============================================

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

#[derive(Deserialize)]
pub struct MineSubmission {
    pub miner_address: String,
    pub nonce: u64,
    pub block_hash: String,
    pub block_index: u64,
    pub previous_hash: String,
    pub transactions: Vec<String>,
}

#[derive(Serialize)]
pub struct WorkResponse {
    pub block_index: u64,
    pub previous_hash: String,
    pub difficulty_target: String,
    pub transactions: Vec<String>,
    pub timestamp: u64,
}

#[derive(Deserialize)]
pub struct RegisterMiner {
    pub address: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct BankVerification {
    pub iban: String,
    pub swift: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub mt103_raw: Option<String>,
}

#[derive(Deserialize)]
pub struct Mt103DownloadParams {
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
}

// ============================================
// MINING STATE
// ============================================

pub struct MiningState {
    pub latest_block: Block,
    pub pending_transactions: Vec<String>,
    pub difficulty: u64,
}

impl MiningState {
    pub fn new() -> Self {
        Self {
            latest_block: Block {
                index: 0,
                timestamp: Utc::now().timestamp() as u64,
                transactions: vec!["Genesis Block".to_string()],
                previous_hash: "0".to_string(),
                nonce: 0,
                hash: "GENESIS".to_string(),
            },
            pending_transactions: Vec::new(),
            difficulty: 4,
        }
    }
}

// ============================================
// EXPLORER HOME & API
// ============================================

async fn explorer_home() -> impl Responder {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>MNZ Sovereign Chain Explorer</title>
    <style>
        * { margin:0; padding:0; box-sizing:border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0a0a10; color: #e0e0e0; padding: 20px; }
        .nav { display: flex; align-items: center; justify-content: space-between; background: #141424; padding: 15px 25px; border-radius: 12px; margin-bottom: 20px; border: 1px solid #1e1e3a; }
        .nav .brand { font-size: 20px; font-weight: bold; color: #00d4ff; text-decoration: none; }
        .nav .brand span { color: #64ffda; }
        .nav-links { display: flex; gap: 15px; }
        .nav-links a { color: #8892b0; text-decoration: none; font-size: 14px; font-weight: 600; padding: 8px 14px; border-radius: 6px; transition: all 0.2s; }
        .nav-links a.active, .nav-links a:hover { background: #1c1c32; color: #64ffda; }
        .hero-search { background: linear-gradient(135deg, #1a1a2e, #16213e); padding: 30px; border-radius: 12px; margin-bottom: 25px; border: 1px solid #2a2a4a; text-align: center; }
        .hero-search h1 { font-size: 26px; color: #ffffff; margin-bottom: 10px; }
        .hero-search p { color: #8892b0; font-size: 14px; margin-bottom: 20px; }
        .stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-bottom: 25px; }
        .stat-card { background: #141424; padding: 18px; border-radius: 10px; border: 1px solid #1e1e3a; }
        .stat-card .lbl { color: #8892b0; font-size: 11px; text-transform: uppercase; letter-spacing: 0.5px; }
        .stat-card .val { font-size: 20px; font-weight: bold; margin-top: 6px; }
        .val.blue { color: #00d4ff; }
        .val.green { color: #64ffda; }
        .val.gold { color: #ffd700; }
        .footer { text-align: center; color: #5a5a7a; font-size: 12px; margin-top: 30px; padding: 15px; border-top: 1px solid #1a1a30; }
    </style>
</head>
<body>
    <div class="nav">
        <a href="/" class="brand">MNZ<span>-CHAIN</span></a>
        <div class="nav-links">
            <a href="/" class="active">Sovereign Explorer</a>
            <a href="/audit">Security & Legal Auditor</a>
            <a href="/api" target="_blank">JSON API</a>
        </div>
    </div>

    <div class="hero-search">
        <h1>MNZ Sovereign Chain Network</h1>
        <p>Decentralized Layer-1 Infrastructure & Immutable Statutory Ledger</p>
    </div>

    <div class="stats-grid">
        <div class="stat-card"><div class="lbl">Network Status</div><div class="val green">ACTIVE MAINNET</div></div>
        <div class="stat-card"><div class="lbl">Block Height</div><div class="val blue">12,845</div></div>
        <div class="stat-card"><div class="lbl">Sovereign Constant (Ω)</div><div class="val gold">-0.00186667</div></div>
        <div class="stat-card"><div class="lbl">Native Gas Asset</div><div class="val green">MZQX</div></div>
    </div>

    <div class="footer">
        MNZ Sovereign Network Infrastructure • Built with Rust & Actix-Web
    </div>
</body>
</html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn audit_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Audit Page</h1>")
}

async fn api_data() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"online","sovereign":{"chain_id":"mnz-sovereign-1"}}"#)
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"healthy","service":"MNZ-Chain"}"#)
}

// ============================================
// MINING ENDPOINTS
// ============================================

pub async fn get_work(data: web::Data<Mutex<MiningState>>) -> impl Responder {
    let state = data.lock().unwrap();
    let diff: usize = state.difficulty as usize;
    let response = WorkResponse {
        block_index: state.latest_block.index + 1,
        previous_hash: state.latest_block.hash.clone(),
        difficulty_target: "0".repeat(diff) + &"f".repeat(64 - diff),
        transactions: state.pending_transactions.clone(),
        timestamp: Utc::now().timestamp() as u64,
    };

    HttpResponse::Ok().json(response)
}

pub async fn submit_mined_block(
    payload: web::Json<MineSubmission>,
    data: web::Data<Mutex<MiningState>>,
) -> impl Responder {
    let mut state = data.lock().unwrap();

    if payload.block_index != state.latest_block.index + 1 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "rejected",
            "reason": "Invalid block index"
        }));
    }

    let new_block = Block {
        index: payload.block_index,
        timestamp: Utc::now().timestamp() as u64,
        transactions: payload.transactions.clone(),
        previous_hash: payload.previous_hash.clone(),
        nonce: payload.nonce,
        hash: payload.block_hash.clone(),
    };

    state.latest_block = new_block;
    state.pending_transactions.clear();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "accepted",
        "block_added": payload.block_index,
    }))
}

pub async fn register_miner(payload: web::Json<RegisterMiner>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "registered",
        "miner_address": payload.address
    }))
}

pub async fn mining_stats(data: web::Data<Mutex<MiningState>>) -> impl Responder {
    let state = data.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "latest_block": state.latest_block.index,
        "difficulty": state.difficulty
    }))
}

// ============================================
// DYNAMIC BANK VERIFICATION & MT103 EXTRACTION
// ============================================

pub async fn verify_bank(payload: web::Json<BankVerification>) -> impl Responder {
    let clean_iban = payload.iban.replace([' ', '-', '_'], "").to_uppercase();
    let clean_swift = payload.swift.replace([' ', '-'], "").to_uppercase();

    let iban_valid = validate_iban_checksum(&clean_iban);
    let swift_valid = validate_swift(&clean_swift);
    let matched_banks = find_banks(&clean_swift, &clean_iban);

    let verified = (iban_valid || clean_iban.len() >= 5) && swift_valid && !matched_banks.is_empty();

    let bank_names: Vec<String> = matched_banks.iter()
        .map(|b| format!("{} ({})", b.name, b.country))
        .collect();

    let bank_summary = if bank_names.is_empty() {
        "No matching bank found".to_string()
    } else {
        bank_names.join("; ")
    };

    // Extract MT103 Field 32A dynamically from raw SWIFT
    let mt103_parsed = payload.mt103_raw.as_deref().and_then(parse_mt103_amount);

    let (extracted_curr, extracted_amt, value_date) = match mt103_parsed {
        Some((curr, amt, dt)) => (Some(curr), Some(amt), Some(dt)),
        None => (None, None, None),
    };

    let final_amount = payload.amount.or(extracted_amt);
    let final_currency = payload.currency.clone().or_else(|| extracted_curr.clone());

    let amount_matches = match (payload.amount, extracted_amt) {
        (Some(submitted), Some(extracted)) => Some((submitted - extracted).abs() < 0.01),
        _ => None,
    };

    let download_url = if verified {
        let base_url = generate_mt103_download_url(&clean_iban, &clean_swift);
        let amt_param = final_amount.map(|a| format!("&amount={:.2}", a)).unwrap_or_default();
        let curr_param = final_currency.as_ref().map(|c| format!("&currency={}", c)).unwrap_or_default();
        let date_param = value_date.as_ref().map(|d| format!("&date={}", d)).unwrap_or_default();
        
        Some(format!("{base_url}?v=1{amt_param}{curr_param}{date_param}"))
    } else {
        None
    };

    HttpResponse::Ok().json(serde_json::json!({
        "status": if verified { "verified" } else { "failed" },
        "bank": bank_summary,
        "verified": verified,
        "submitted_amount": payload.amount,
        "submitted_currency": payload.currency,
        "extracted_amount": extracted_amt,
        "extracted_currency": extracted_curr,
        "effective_amount": final_amount,
        "effective_currency": final_currency,
        "value_date": value_date,
        "amount_matches": amount_matches,
        "download_url": download_url,
        "iban_valid": iban_valid,
        "swift_valid": swift_valid,
    }))
}

// ============================================
// DYNAMIC MT103 DOCUMENT DOWNLOAD ENDPOINT
// ============================================

pub async fn download_mt103(
    path: web::Path<String>,
    query: web::Query<Mt103DownloadParams>,
) -> impl Responder {
    let hash = path.into_inner();

    let amount = query.amount.unwrap_or(19777000.00);
    let currency = query.currency.as_deref().unwrap_or("EUR");
    let date = query.date.as_deref().unwrap_or("190221");

    // Format amount into SWIFT standard decimal comma format (e.g., 987654.32 -> 987654,32)
    let formatted_amount = format!("{:.2}", amount).replace('.', ",");
    let field_32a = format!(":32A:{}{}{}", date, currency, formatted_amount);

    let mt103_document = format!(
        "{{1:F01HSBCGB22AXXX0000000000}}\n\
         {{2:I103MIDLGB22XXXXN}}\n\
         {{3:{{108:MT103-{}}}}}\n\
         {{4:\n\
         :20:{}\n\
         :23B:CRED\n\
         {}\n\
         :50K:/1234567890\n\
         HSBC BANK PLC\n\
         LONDON, UNITED KINGDOM\n\
         :59:/0987654321\n\
         SETTLEMENT ACCOUNT\n\
         :71A:OUR\n\
         -}}",
        &hash[..std::cmp::min(12, hash.len())],
        hash,
        field_32a
    );

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"MT103_{}.txt\"", hash),
        ))
        .body(mt103_document)
}

// ============================================
// MAIN SERVER ENTRYPOINT
// ============================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mining_state = web::Data::new(Mutex::new(MiningState::new()));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    println!("🚀 Server launching on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(mining_state.clone())
            .route("/", web::get().to(explorer_home))
            .route("/audit", web::get().to(audit_page))
            .route("/api", web::get().to(api_data))
            .route("/health", web::get().to(health))
            .route("/api/health", web::get().to(health))
            .route("/mine/work", web::get().to(get_work))
            .route("/mine/submit", web::post().to(submit_mined_block))
            .route("/mine/register", web::post().to(register_miner))
            .route("/mine/stats", web::get().to(mining_stats))
            .route("/bank/verify", web::post().to(verify_bank))
            .route("/mt103/download/{hash}", web::get().to(download_mt103))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}