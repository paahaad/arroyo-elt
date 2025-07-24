// This is how dependencies are defined in the Arroyo UDF plugin
/*
[dependencies]
serde_json = "1.0.141"
base64 = "0.22.1"
borsh = { version = "=1.5.7", features = ["derive"] }
solana-sdk = "2.3.1"
*/

use arroyo_udf_plugin::udf;
use serde_json::{self, Value, json};
use base64::engine::Engine as _;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::fmt;

// This is the struct that will be used to parse the minted account data
#[derive(Debug, BorshDeserialize)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub token_total_supply: u64,
}

#[udf]
fn parse_pump_mint(log_json: &str) -> Option<String> {
    let data: Value = match serde_json::from_str(log_json) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let logs = match data["params"]["result"]["value"]["logs"].as_array() {
        Some(l) => l,
        None => return None,
    };

    if !logs.iter().any(|l| l.as_str().map_or(false, |s| s.contains("Instruction: Create"))) {
        return None;
    }
 
    let data_line = logs.iter().find(|l| l.as_str().map_or(false, |s| s.starts_with("Program data: ")))?;
    let data_str = data_line.as_str().unwrap().split_once(":").unwrap().1;

    // Decode base64 to raw bytes
    let bytes = base64::decode(data_str.trim()).expect("Failed to decode base64");

    // Strip the first 8 bytes (discriminator)
    let data = &bytes[8..];

    // Deserialize
    let event = match CreateEvent::try_from_slice(data){
        Ok(e) => e,
        Err(error) => {
            println!("error in decoding-> {error}");
            return None;
        }
    };

    let json_result = json!({
        "name": event.name,
        "symbol": event.symbol,
        "uri": event.uri,
        "mint": event.mint.to_string(),
        "bonding_curve": event.bonding_curve.to_string(),
        "user": event.user.to_string(),
        "creator": event.creator.to_string(),
        "timestamp": event.timestamp.to_string(),
        "virtual_token_reserves": event.virtual_token_reserves.to_string(),
        "virtual_sol_reserves": event.virtual_sol_reserves.to_string(),
        "real_token_reserves": event.real_token_reserves.to_string(),
        "token_total_supply": event.token_total_supply.to_string()
    });

    return Some(json_result.to_string());

}