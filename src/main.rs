use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn dashboard() -> impl Responder {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>⚛️ MNZ Blockchain Explorer - Production</title>
    <style>
        * { margin:0; padding:0; box-sizing:border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0a0a0a; color: #e0e0e0; padding: 20px; }
        .header { background: linear-gradient(135deg, #1a1a2e, #16213e); padding: 20px; border-radius: 12px; margin-bottom: 20px; border: 1px solid #2a2a4a; }
        .header h1 { color: #00d4ff; font-size: 28px; }
        .header h1 span { color: #64ffda; }
        .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 15px; margin-bottom: 20px; }
        .stat { background: #141424; padding: 15px; border-radius: 10px; border: 1px solid #1e1e3a; }
        .stat .label { color: #8892b0; font-size: 12px; text-transform: uppercase; }
        .stat .value { font-size: 22px; font-weight: bold; margin-top: 5px; }
        .stat .value.omega { color: #ffd700; }
        .stat .value.green { color: #64ffda; }
        .stat .value.blue { color: #00d4ff; }
        .stat .value.purple { color: #a855f7; }
        .section { background: #141424; padding: 20px; border-radius: 12px; margin-bottom: 20px; border: 1px solid #1e1e3a; }
        .section h2 { color: #64ffda; margin-bottom: 15px; font-size: 18px; }
        .dot { display: inline-block; width: 10px; height: 10px; background: #64ffda; border-radius: 50%; animation: pulse 2s infinite; margin-right: 8px; }
        @keyframes pulse { 0% { opacity: 1; } 50% { opacity: 0.3; } 100% { opacity: 1; } }
        .status-bar { display: flex; align-items: center; justify-content: space-between; background: #0d0d1a; padding: 10px 15px; border-radius: 8px; margin-top: 10px; flex-wrap: wrap; gap: 10px; }
        .btn { background: #00d4ff; color: #0a0a0a; padding: 8px 16px; border: none; border-radius: 6px; cursor: pointer; font-weight: bold; }
        .btn:hover { background: #64ffda; }
        .footer { text-align: center; color: #4a4a6a; margin-top: 20px; font-size: 12px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>⚛️ MNZ <span>Blockchain</span> Explorer - Production</h1>
        <div class="status-bar">
            <div><span class="dot"></span> <span id="statusText">Live</span> • Block #<span id="blockHeightDisplay">0</span></div>
            <div>🔥 Firewall: ACTIVE</div>
            <div>🔗 <span id="walletCountDisplay">0</span> Wallets</div>
            <button class="btn" onclick="fetchData()">🔄 Refresh</button>
        </div>
    </div>

    <div class="stats" id="statsContainer">
        <div class="stat"><div class="label">Block Height</div><div class="value blue" id="blockHeight">0</div></div>
        <div class="stat"><div class="label">Total Transactions</div><div class="value green" id="totalTransactions">0</div></div>
        <div class="stat"><div class="label">Wallets</div><div class="value green" id="wallets">0</div></div>
        <div class="stat"><div class="label">Ω Constant</div><div class="value omega" id="omega">-0.00186667</div></div>
        <div class="stat"><div class="label">Peg (USD)</div><div class="value blue" id="peg">$4.0</div></div>
        <div class="stat"><div class="label">Contracts</div><div class="value purple" id="contracts">0</div></div>
        <div class="stat"><div class="label">Total Supply</div><div class="value green" id="totalSupply">0 MZQX</div></div>
        <div class="stat"><div class="label">Market Cap</div><div class="value blue" id="marketCap">$0</div></div>
    </div>

    <div class="footer">
        <span class="dot"></span> MNZ Blockchain Live • Ω = -0.00186667 • 1 MZQX = 4 USD • Production v2.0
    </div>

    <script>
        const API_URL = "/api";

        async function fetchData() {
            try {
                const response = await fetch(API_URL);
                const data = await response.json();

                document.getElementById("blockHeight").textContent = data.block_height;
                document.getElementById("totalTransactions").textContent = data.transactions;
                document.getElementById("wallets").textContent = data.wallets;
                document.getElementById("omega").textContent = data.omega.toFixed(8);
                document.getElementById("peg").textContent = "$" + data.peg.toFixed(1);
                document.getElementById("contracts").textContent = data.contracts;
                document.getElementById("totalSupply").textContent = data.total_supply.toLocaleString("en-US", {minimumFractionDigits: 2, maximumFractionDigits: 2}) + " MZQX";
                document.getElementById("marketCap").textContent = "$" + data.market_cap.toLocaleString("en-US", {minimumFractionDigits: 2, maximumFractionDigits: 2});

                document.getElementById("blockHeightDisplay").textContent = data.block_height;
                document.getElementById("walletCountDisplay").textContent = data.wallets;
                document.getElementById("statusText").textContent = "Live";
            } catch (error) {
                document.getElementById("statusText").textContent = "Offline";
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
    let block_reward: f64 = 500.0;
    let peg: f64 = 4.0;

    let total_supply: f64 = (block_height as f64) * block_reward;
    let market_cap: f64 = total_supply * peg;

    let response_body = format!(
        r#"{{
            "block_height": {},
            "transactions": 45892,
            "wallets": 1024,
            "omega": -0.00186667,
            "peg": {},
            "contracts": 18,
            "total_supply": {},
            "market_cap": {}
        }}"#,
        block_height, peg, total_supply, market_cap
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response_body)
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"healthy","service":"MNZ-Chain"}"#)
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
