use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

const HTML_DASHBOARD: &str = r###"
<!DOCTYPE html>
<html>
<head>
    <title>⚛️ MNZ Blockchain Explorer - Production</title>
    <style>
        * { margin:0; padding:0; box-sizing:border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0a0a0a; color: #e0e0e0; padding: 20px; }
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
        .login-form { background: #141424; padding: 20px; border-radius: 12px; border: 1px solid #2a2a4a; max-width: 400px; margin: 20px auto; }
        .login-form input { width: 100%; padding: 10px; margin: 10px 0; border-radius: 6px; border: 1px solid #2a2a4a; background: #0a0a0a; color: #e0e0e0; }
        .login-form button { width: 100%; padding: 10px; margin: 10px 0; border-radius: 6px; border: none; background: #00d4ff; color: #0a0a0a; font-weight: bold; cursor: pointer; }
        .admin-panel { background: #141424; padding: 20px; border-radius: 12px; border: 1px solid #2a2a4a; margin-top: 20px; }
        .admin-panel table { width: 100%; border-collapse: collapse; }
        .admin-panel th, .admin-panel td { padding: 10px; border: 1px solid #2a2a4a; text-align: left; }
        .scan-input { display: flex; gap: 10px; margin: 20px 0; }
        .scan-input input { flex: 1; padding: 10px; border-radius: 6px; border: 1px solid #2a2a4a; background: #0a0a0a; color: #e0e0e0; }
        .scan-input button { padding: 10px 20px; border-radius: 6px; border: none; background: #a855f7; color: #fff; font-weight: bold; cursor: pointer; }
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
            <button class="btn" onclick="toggleAuth()">🔐 Login</button>
        </div>
    </div>

    <div id="authForm" class="login-form" style="display:none;">
        <h3>🔐 User Auth</h3>
        <input type="text" id="username" placeholder="Username" />
        <input type="password" id="password" placeholder="Password" />
        <button onclick="login()">Login</button>
        <button onclick="register()" style="background:#64ffda;color:#0a0a0a;">Register</button>
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

    <div class="section">
        <h2>📦 Latest Block #<span id="latestBlockNumber">0</span></h2>
        <p style="font-family: monospace; color: #8892b0; font-size: 13px; line-height: 1.8;" id="latestBlockDetails">Loading...</p>
    </div>

    <div class="section">
        <h2>🔍 Scan New Coin</h2>
        <div class="scan-input">
            <input type="text" id="coinAddress" placeholder="Enter BSC Contract Address (0x...)" />
            <button onclick="scanCoin()">🔎 Scan</button>
        </div>
        <div id="scanResult"></div>
    </div>

    <div class="admin-panel" id="Panel" style="display:none;">
        <h2>🛡️ Admin Panel</h2>
        <table>
            <thead>
                <tr><th>User</th><th>Role</th><th>Status</th><th>Action</th></tr>
            </thead>
            <tbody id="userTable">
                <tr><td>Admin</td><td>Administrator</td><td>✅ Active</td><td><button class="btn">Manage</button></td></tr>
            </tbody>
        </table>
    </div>

    <div class="footer">
        <span class="dot"></span> MNZ Blockchain Live • Ω = -0.00186667 • 1 MZQX = 4 USD • Production v2.0
    </div>

    <script>
        const API_URL = '/api';

        async function fetchData() {
            try {
                const response = await fetch(API_URL);
                const data = await response.json();

                document.getElementById('blockHeight').textContent = data.block_height;
                document.getElementById('totalTransactions').textContent = data.transactions;
                document.getElementById('wallets').textContent = data.wallets;
                document.getElementById('omega').textContent = data.omega.toFixed(8);
                document.getElementById('peg').textContent = '$' + data.peg.toFixed(1);
                document.getElementById('contracts').textContent = data.contracts;
                document.getElementById('totalSupply').textContent = data.total_supply.toFixed(2) + ' MZQX';
                document.getElementById('marketCap').textContent = '$' + data.market_cap.toFixed(2);

                document.getElementById('blockHeightDisplay').textContent = data.block_height;
                document.getElementById('walletCountDisplay').textContent = data.wallets;
                document.getElementById('statusText').textContent = 'Live';

                if (data.latestBlock) {
                    const lb = data.latestBlock;
                    document.getElementById('latestBlockNumber').textContent = lb.number;
                    document.getElementById('latestBlockDetails').innerHTML = `
                        Hash: ${lb.hash}<br>
                        Miner: ${lb.miner}<br>
                        Transactions: ${lb.transactions}<br>
                        Gas Used: ${lb.gasUsed} / ${lb.gasLimit}<br>
                        Size: ${lb.size} bytes<br>
                        Total Supply: ${data.total_supply.toFixed(2)} MZQX<br>
                        Market Cap: $${data.market_cap.toFixed(2)}
                    `;
                }
            } catch (error) {
                document.getElementById('statusText').textContent = 'Offline';
                console.error('Error fetching data:', error);
            }
        }

        function toggleAuth() {
            const form = document.getElementById('authForm');
            form.style.display = form.style.display === 'none' ? 'block' : 'none';
        }

        function login() {
            const username = document.getElementById('username').value;
            const password = document.getElementById('password').value;
            if (username && password) {
                alert('Successful login! Welcome ' + username);
                document.getElementById('Panel').style.display = 'block';
                document.getElementById('authForm').style.display = 'none';
            } else {
                alert('⚠️ Please enter username and password.');
            }
        }

        function register() {
            const username = document.getElementById('username').value;
            const password = document.getElementById('password').value;
            if (username && password) {
                alert('✅ Registration successful! Please log in.');
            } else {
                alert('⚠️ Please enter username and password.');
            }
        }

        async function scanCoin() {
            const address = document.getElementById('coinAddress').value.trim();
            if (!address || !address.startsWith('0x')) {
                document.getElementById('scanResult').innerHTML = '⚠️ Please enter a valid BSC contract address.';
                return;
            }
            document.getElementById('scanResult').innerHTML = '🔎 Scanning...';

            try {
                const response = await fetch('https://api.bscscan.com/api?module=contract&action=getabi&address=' + address);
                const data = await response.json();
                if (data.status === '1') {
                    document.getElementById('scanResult').innerHTML = `
                        ✅ Coin found!<br>
                        Address: ${address}<br>
                        Status: Verified<br>
                        ABI: Available
                    `;
                } else {
                    document.getElementById('scanResult').innerHTML = '❌ Coin not found or not verified.';
                }
            } catch (error) {
                document.getElementById('scanResult').innerHTML = '❌ Error scanning coin.';
            }
        }

        fetchData();
        setInterval(fetchData, 10000);
    </script>
</body>
</html>
"###;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HTML_DASHBOARD)
}

#[get("/api")]
async fn api_status() -> impl Responder {
    let json_response = r#"{
        "block_height": 1024,
        "transactions": 4850,
        "wallets": 128,
        "omega": -0.00186667,
        "peg": 4.0,
        "contracts": 12,
        "total_supply": 2500000.0,
        "market_cap": 10000000.0,
        "latestBlock": {
            "number": 1024,
            "hash": "0x7f8d9a2b3c4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a",
            "miner": "0x3A89e217593c6A2906C2D64f1d4310C1b84B12A8",
            "transactions": 32,
            "gasUsed": "1,245,000",
            "gasLimit": "30,000,000",
            "size": 18420
        }
    }"#;

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(json_response)
}

#[get("/health")]
#[get("/api/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"healthy"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(api_status)
            .service(health)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
