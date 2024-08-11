//! ## Tracker Service
//!
//! Responsible for tracking the Coretime chain and triggering the notification service
//! when needed.
use subxt::{OnlineClient, PolkadotConfig};

const LOG_TARGET: &str = "tracker";
const RPC: &str = "wss://sys.ibp.network/coretime-kusama/";

#[subxt::subxt(runtime_metadata_path = "../../artifacts/kusama-coretime.scale")]
mod coretime_chain {}

pub async fn track() -> Result<(), Box<dyn std::error::Error>> {
    let result = OnlineClient::<PolkadotConfig>::from_url(RPC).await;
    let Ok(client) = result else {
        log::error!(
            target: LOG_TARGET,
            "Failed to create online client: {:?}",
            result
        );
        // TODO: return error
        return Ok(())
    };

    let sale_info_query = coretime_chain::storage().broker().sale_info();
	let sale_info = client
		.storage()
		.at_latest()
		.await?
		.fetch(&sale_info_query)
		.await?
		.ok_or("Failed to query sale info")?;
    
    // Based on the sale start we will derive other time related data.
    

    Ok(())
}

fn track_coretime_sales() {
    todo!()
}

fn track_interlude_phase() {
    todo!()
}
