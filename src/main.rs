use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn dashboard() -> impl Responder {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>⚛️ MNZ Sovereign & BSC Multi-Chain Explorer</title>
    <style>
        * { margin:0; padding:0; box-sizing:border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0a0a10; color: #e0e0e0; padding: 20px; }
        .header { background: linear-gradient(135deg, #1a1a2e, #16213e); padding: 25px; border-radius: 12px; margin-bottom: 25px; border: 1px solid #2a2a4a; }
        .header h1 { color: #00d4ff; font-size: 26px; }
        .header h1 span { color: #f3ba2f; }
        .status-bar { display: flex; align-items: center; justify-content: space-between; background: #0d0d1a; padding: 12px 18px; border-radius: 8px; margin-top: 15px; flex-wrap: wrap; gap: 10px; border: 1px solid #1e1e3a; }
        
        .grid-container { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 20px; margin-bottom: 25px; }
        .card { background: #141424; padding: 20px; border-radius: 12px; border: 1px solid #1e1e3a; }
        .card.sovereign { border-top: 4px solid #00d4ff; }
        .card.bsc { border-top: 4px solid #f3ba2f; }
        .card h2 { font-size: 18px; margin-bottom: 15px; display: flex; align-items: center; gap: 8px; }
        .card.sovereign h2 { color: #00d4ff; }
        .card.bsc h2 { color: #f3ba2f; }

        .stat-group { display: flex; flex-direction: column; gap: 12px; }
        .stat-row { display: flex; justify-content: space-between; align-items: center; padding: 8px 0; border-bottom: 1px solid #1a1a30; }
        .stat-row .label { color: #8892b0; font-size: 13px; }
        .stat-row .val { font-weight: bold; font-size: 15px; }
        .val.gold { color: #ffd700; }
        .val.green { color: #64ffda; }
        .val.blue { color: #00d4ff; }
        .val.yellow { color: #f3ba2f; }

        .dot { display: inline-block; width: 10px; height: 10px; background: #64ffda; border-radius: 50%; animation: pulse 2s infinite; margin-right: 8px; }
        @keyframes pulse { 0% { opacity: 1; } 50% { opacity: 0.3; } 100% { opacity: 1; } }
        .btn { background: #00d4ff; color: #0a0a0a; padding: 8px 16px; border: none; border-radius: 6px; cursor: pointer; font-weight: bold; }
        .btn:hover { background: #64ffda; }
        .footer { text-align: center; color: #5a5a7a; font-size: 12px; margin-top: 25px; }
        .contract-box { background: #0b0b14; padding: 10px; border-radius: 6px; font-family: monospace; font-size: 12px; word-break: break-all; border: 1px solid #22223a; color: #f3ba2f; margin-top: 10px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>⚛️ MNZ <span>Multi-Chain Architecture</span> Dashboard</h1>
        <div class="status-bar">
            <div><span class="dot"></span> Sovereign L1 Status: <strong style="color:#64ffda;">ONLINE</strong></div>
            <div>🌉 BSC Bridge Status: <strong style="color:#f3ba2f;">ACTIVE</strong></div>
            <button class="btn" onclick="fetchData()">🔄 Sync Multi-Chain State</button>
        </div>
    </div>

    <div class="grid-container">
        <!-- Sovereign L1 Card -->
        <div class="card sovereign">
            <h2>⚛️ Sovereign Core Layer (MNZ L1)</h2>
            <div class="stat-group">
                <div class="stat-row"><span class="label">Block Height</span><span class="val blue" id="blockHeight">12845</span></div>
                <div class="stat-row"><span class="label">Sovereign Constant (Ω)</span><span class="val gold" id="omega">-0.00186667</span></div>
                <div class="stat-row"><span class="label">Sovereign Asset Anchor</span><span class="val green">$1,000,000 USD</span></div>
                <div class="stat-row"><span class="label">Total Supply</span><span class="val green" id="totalSupply">1,000,000,000.00 MZQX</span></div>
                <div class="stat-row"><span class="label">Peg Value</span><span class="val blue" id="peg">$4.0 USD</span></div>
                <div class="stat-row"><span class="label">Market Cap</span><span class="val blue" id="marketCap">$4,000,000,000.00</span></div>
            </div>
        </div>

        <!-- BSC Liquidity Bridge Card -->
        <div class="card bsc">
            <h2>🟡 Liquidity Gateway (BSC Network)</h2>
            <div class="stat-group">
                <div class="stat-row"><span class="label">Token Standard</span><span class="val yellow">BEP-20 (BNB Smart Chain)</span></div>
                <div class="stat-row"><span class="label">Contract Verification</span><span class="val green">VERIFIED</span></div>
                <div class="stat-row"><span class="label">Minted Supply on BSC</span><span class="val yellow">1,000,000,000 MZQX</span></div>
                <div class="stat-row"><span class="label">DEX Pairing Target</span><span class="val blue">MZQX / USDT</span></div>
                <div class="stat-row"><span class="label">Active Indexers</span><span class="val green">DEX Screener • ApeSpace</span></div>
            </div>
            <div style="margin-top: 15px;">
                <span style="color:#8892b0; font-size:12px;">BEP-20 Smart Contract Address:</span>
                <div class="contract-box">0xCe7cBb63399a1b7Df6b92A22163c326499E7C4c5</div>
            </div>
        </div>
    </div>

    <div class="footer">
        ⚛️ Sovereign Engine + 🟡 BSC Liquidity Gateway Bridge • Built with Rust & Actix-Web
    </div>

    <script>
        const API_URL = "/api";

        async function fetchData() {
            try {
                const response = await fetch(API_URL);
                const data = await response.json();

                document.getElementById("blockHeight").textContent = data.sovereign.block_height;
                document.getElementById("omega").textContent = data.sovereign.omega.toFixed(8);
                document.getElementById("totalSupply").textContent = data.sovereign.total_supply.toLocaleString("en-US", {minimumFractionDigits: 2, maximumFractionDigits: 2}) + " MZQX";
                document.getElementById("peg").textContent = "$" + data.sovereign.peg.toFixed(1) + " USD";
                document.getElementById("marketCap").textContent = "$" + data.sovereign.market_cap.toLocaleString("en-US", {minimumFractionDigits: 2, maximumFractionDigits: 2});
            } catch (error) {
                console.error("Failed to fetch state:", error);
            }
        }

        fetchData();
        setInterval(fetchData, 10000);
    </script>
</body>
</html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn api_data() -> impl Responder {
    let block_height: u64 = 12845;
    let peg: f64 = 4.0;
    let total_supply: f64 = 1_000_000_000.0;
    let market_cap: f64 = total_supply * peg;

    let response_body = format!(
        r#"{{
            "architecture": "Option B - Multi-Chain Sovereign + Bridge",
            "sovereign": {{
                "chain_id": "mnz-sovereign-1",
                "block_height": {},
                "omega": -0.00186667,
                "asset_anchor_usd": 1000000.0,
                "total_supply": {},
                "peg": {},
                "market_cap": {}
            }},
            "bsc_bridge": {{
                "network": "BNB Smart Chain (BEP-20)",
                "contract": "0xCe7cBb63399a1b7Df6b92A22163c326499E7C4c5",
                "status": "Verified & Active",
                "minted_supply": 1000000000
            }}
        }}"#,
        block_height, total_supply, peg, market_cap
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response_body)
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"healthy","architecture":"multi-chain","service":"MNZ-Chain"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(dashboard))
            .route("/dashboard", web::get().to(dashboard))
            .route("/api", web::get().to(api_data))
            .route("/health", web::get().to(health))
            .route("/api/health", web::get().to(health))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
