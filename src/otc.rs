use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDealRequest {
    pub deal_id: String,
    pub buyer: String,
    pub token_address: String,
    pub token_amount: String,
    pub fiat_amount: String,
    pub currency_code: String,
    pub duration_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OTCResponse {
    pub status: String,
    pub deal_id: String,
    pub message: String,
}

pub async fn handle_create_deal(req: web::Json<CreateDealRequest>) -> impl Responder {
    HttpResponse::Ok().json(OTCResponse {
        status: "success".to_string(),
        deal_id: req.deal_id.clone(),
        message: "OTC deal initialized for escrow deposit".to_string(),
    })
}
