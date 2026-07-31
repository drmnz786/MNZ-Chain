use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Mutex;
use chrono::DateTime;
use serde_json::json;

const OMEGA: f64 = -0.0018666666666666666;
const PEG_USD: f64 = 4.0;
const RESONANCE_FREQ_MHZ: f64 = 2.212;

struct AppState {
    block_height: Mutex<u64>,
    transactions: Mutex<u64>,
    wallets: Mutex<u64>,
    total_supply: Mutex<f64>,
    market_cap: Mutex<f64>,
    liquidity: Mutex<f64>,
    price: Mutex<f64>,
}

#[derive(Serialize)]
struct BlockchainStatus {
    chain: String,
    omega: f64,
    peg: f64,
    frequency: f64,
    block_height: u64,
    transactions: u64,
    wallets: u64,
    reserves: u64,
    firewall: String,
    total_supply: f64,
    market_cap: f64,
    liquidity: f64,
    price: f64,
    liquidity_usd: String,
    contracts: u64,
    latest_block: BlockResponse,
    recent_transactions: Vec<TransactionResponse>,
}

#[derive(Serialize)]
struct BlockResponse {
    number: u64,
    hash: String,
    parent: String,
    timestamp: u64,
    time: String,
    transactions: u64,
    size: u64,
    gas_used: f64,
    gas_limit: f64,
    miner: String,
    difficulty: u64,
    total_supply: f64,
    market_cap: f64,
}

#[derive(Serialize, Clone)]
struct TransactionResponse {
    hash: String,
    from: String,
    to: String,
    value: String,
    timestamp: String,
}

#[get("/api")]
async fn get_status(data: web::Data<AppState>) -> impl Responder {
    let block_height = *data.block_height.lock().unwrap();
    let transactions = *data.transactions.lock().unwrap();
    let wallets = *data.wallets.lock().unwrap();
    let total_supply = *data.total_supply.lock().unwrap();
    let market_cap = *data.market_cap.lock().unwrap();
    let liquidity = *data.liquidity.lock().unwrap();
    let price = *data.price.lock().unwrap();

    let status = BlockchainStatus {
        chain: "MNZ Chain / MZQX BEP-20".to_string(),
        omega: OMEGA,
        peg: PEG_USD,
        frequency: RESONANCE_FREQ_MHZ,
        block_height,
        transactions,
        wallets,
        reserves: 16_000_000,
        firewall: "ACTIVE".to_string(),
        total_supply,
        market_cap,
        liquidity,
        price,
        liquidity_usd: format!("${:.0}", liquidity),
        contracts: 3,
        latest_block: BlockResponse {
            number: block_height,
            hash: format!("0x{:016x}{:016x}{:016x}", block_height, 0x6a6bc4c1, 0x0b),
            parent: "0x0000000000000000000000006a6bc4c1".to_string(),
            timestamp: 1785340910,
            time: "2026-07-29 16:01:50".to_string(),
            transactions: 2,
            size: 512,
            gas_used: 28.5,
            gas_limit: 30000000.0,
            miner: format!("MNZ Miner {}", block_height),
            difficulty: 1000000,
            total_supply,
            market_cap,
        },
        recent_transactions: vec![
            TransactionResponse {
                hash: "0xbc4881447633ef32831ec93613d859a33c73a70fa0de78db28e3a83f8f0bad71".to_string(),
                from: "0xF56B3747828B51C25175795E0E5c14284E0c5F3f".to_string(),
                to: "0x5eaAc666f01AB5f0f3E5848CcBdaA90C39f05cB4".to_string(),
                value: "3.87525 USDT".to_string(),
                timestamp: "2026-07-29 21:59:21".to_string(),
            },
            TransactionResponse {
                hash: "0x89c90a149b5f30d0411675db297dc3f80571c43b66c55f80d85d89085284cb23".to_string(),
                from: "0xF56B3747828B51C25175795E0E5c14284E0c5F3f".to_string(),
                to: "0x58271e0218233E011c585790a4a9617f2Aff60bC".to_string(),
                value: "100.00 MZQX".to_string(),
                timestamp: "2026-07-31 16:09:32".to_string(),
            },
        ],
    };

    HttpResponse::Ok().json(status)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("ðŸš€ Munthazar Protocol v2.0 - Node Active")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        block_height: Mutex::new(1456960),
        transactions: Mutex::new(170),
        wallets: Mutex::new(41),
        total_supply: Mutex::new(1000000000.0),
        market_cap: Mutex::new(3810000000.0),
        liquidity: Mutex::new(1000000.0),
        price: Mutex::new(3.8088),
    });

    println!("ðŸš€ Starting Munthazar Protocol Server on port 8080...");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(get_status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
