//! ## Tracker Service
//!
//! Responsible for tracking the Coretime chain and triggering the notification service
//! when needed.
use subxt::{blocks::Block, OnlineClient, PolkadotConfig};

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
		return Ok(());
	};

	let sale_info_query = coretime_chain::storage().broker().sale_info();
	let sale_info = client
		.storage()
		.at_latest()
		.await?
		.fetch(&sale_info_query)
		.await?
		.ok_or("Failed to query sale info")?;

	let config_query = coretime_chain::storage().broker().configuration();
	let config = client
		.storage()
		.at_latest()
		.await?
		.fetch(&config_query)
		.await?
		.ok_or("Failed to query sale info")?;

	// There shouldn't be a need to convert these to timestamps. We will just follow the latest
	// block, and once it is one of the key ones, we will trigger a notification.
	//
	// NOTE: these values should be updated whenever a rotation to a new sale happens.
	let leadin_start: u32 = sale_info.sale_start;
	let interlude_start: u32 = leadin_start.saturating_sub(config.interlude_length);
	let fixed_phase_start: u32 = leadin_start.saturating_add(config.leadin_length);

	let mut blocks_sub = client
		.blocks()
		.subscribe_finalized()
		.await
		.map_err(|_| "Failed to subscribe to finalized blocks")?;

	// Wait for new finalized blocks, then check if an event we are waiting for happened.
	while let Some(Ok(block)) = blocks_sub.next().await {
		println!("{}", block.header().number);

		// Track everything we want to track:
		track_coretime_sales(&block);
		track_interlude_phase(&block, interlude_start);
		track_leadin_phase(&block, leadin_start);
		track_fixed_phase(&block, fixed_phase_start);
	}

	Ok(())
}

fn track_coretime_sales(block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>) {
	// Check if a sale was made.
	todo!()
}

fn track_interlude_phase(
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
	interlude_start: u32,
) {
	// Check if interlude started
	todo!()
}

fn track_leadin_phase(
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
	leadin_start: u32,
) {
	// Check if leadin started
	todo!()
}

fn track_fixed_phase(
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
	fixed_phase_start: u32,
) {
	// Check if fixed phase started
	todo!()
}
