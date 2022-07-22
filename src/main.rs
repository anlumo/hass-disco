use hass_rs::client;
use lazy_static::lazy_static;
use serde_json::json;
//use tokio::time::sleep;
use std::{env::var/*, time::Duration*/};
use rand::Rng;

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJkYzQwZjA4NjM0ZjA0NzM1YTY1Y2U2ZTZlYzZlY2JkOCIsImlhdCI6MTY1ODQzNTUxNywiZXhwIjoxOTczNzk1NTE3fQ.Sl9E9yJ_SO5U18VFRmy9clDi5ZF5boYwLvrapBQaF3A

lazy_static! {
    static ref TOKEN: String = var("HASS_TOKEN").expect("please set up the HASS_TOKEN env variable before running this");
}

const ENTITY: [&str; 9] = [
    "light.ewelight_zb_cl01_5310eb55_level_light_color_on_off",
    "light.ewelight_zb_cl01_65300f48_level_light_color_on_off",
    "light.ewelight_zb_cl01_69b00195_level_light_color_on_off",
    "light.ewelight_zb_cl01_7208c93f_level_light_color_on_off",
    "light.ewelight_zb_cl01_aedd6700_level_light_color_on_off",
    "light.ewelight_zb_cl01_b1a72bb7_level_light_color_on_off",
    "light.ewelight_zb_cl01_d4a83ca6_level_light_color_on_off",
    "light.ewelight_zb_cl01_e0977c73_level_light_color_on_off",
    "light.ewelight_zb_cl01_ec570fed_level_light_color_on_off",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut rng = rand::thread_rng();

    log::info!("Creating the Websocket Client and Authenticate the session");
    let mut client = client::connect("151.217.102.206", 8123, false).await?;

    client.auth_with_longlivedtoken(&*TOKEN).await?;
    log::info!("WebSocket connection and authethication works");

    log::debug!("Get Hass Config");

    loop {
        let idx = rng.gen_range(0..ENTITY.len());
        let angle = rng.gen_range(0..360);
        log::debug!("Setting light {idx} to angle {angle}");

        client.call_service("light".to_owned(), "turn_on".to_owned(), Some(json!({
            "entity_id": ENTITY[idx],
            "hs_color": [angle, 100],
        }))).await?;
        // sleep(Duration::from_secs(2)).await;
    }
}
