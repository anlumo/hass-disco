use clap::Parser;
use hass_rs::{client, HassError};
use rand::Rng;
use serde_json::json;
use std::time::Duration;
use tokio::{
    sync::mpsc::{error::TryRecvError, unbounded_channel},
    time::sleep,
};

mod config;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct DiscoFlags {
    #[clap(long, short, default_value = "disco.toml")]
    config: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut rng = rand::thread_rng();
    let cmdline = DiscoFlags::parse();

    let config = config::get_config(&cmdline.config)?;

    log::info!(
        "Connecting to {} port {}",
        config.server.host,
        config.server.port.unwrap_or(8123)
    );
    let mut client = loop {
        match client::connect(
            &config.server.host,
            config.server.port.unwrap_or(8123),
            config.server.tls.unwrap_or(false),
        )
        .await
        {
            Ok(client) => break client,
            Err(HassError::CantConnectToGateway) => {
                log::warn!("Failed to connect, trying again in 3 seconds...");
                sleep(Duration::from_secs(3)).await;
            }
            Err(err) => return Err(err.into()),
        }
    };

    client
        .auth_with_longlivedtoken(&config.server.hass_token)
        .await?;
    log::debug!("WebSocket connection and authenthication works");

    let (sender, mut receiver) = unbounded_channel::<bool>();

    let input = config.entities.input;
    client
        .subscribe_event("state_changed", move |ws_event| {
            let data = ws_event.event.data;
            if data.entity_id == input {
                if let Some(state) = data.new_state {
                    sender.send(&state.state == "on").unwrap();
                }
            }
        })
        .await?;

    let entities = config.entities.disco;

    loop {
        let state = receiver.recv().await;
        if Some(true) == state {
            loop {
                let idx = rng.gen_range(0..entities.len());
                let angle = rng.gen_range(0..360);
                log::debug!("Setting light {idx} to angle {angle}");

                client
                    .call_service(
                        "light".to_owned(),
                        "turn_on".to_owned(),
                        Some(json!({
                            "entity_id": entities[idx],
                            "hs_color": [angle, 100],
                        })),
                    )
                    .await?;

                match receiver.try_recv() {
                    Ok(false) => break,
                    Ok(true) => {}
                    Err(TryRecvError::Empty) => {}
                    Err(err) => {
                        log::error!("Err: {err:?}");
                        return Err(err.into());
                    }
                }

                if Ok(false) == receiver.try_recv() {
                    break;
                }
            }
            for entity in &entities {
                log::debug!("Turn off {entity}");
                client
                    .call_service(
                        "light".to_owned(),
                        "turn_off".to_owned(),
                        Some(json!({
                            "entity_id": entity,
                        })),
                    )
                    .await?;
            }
        }
    }
}
