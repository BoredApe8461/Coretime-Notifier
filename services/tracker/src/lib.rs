//! ## Tracker Service
//!
//! Responsible for tracking the Coretime chain and triggering the notification service
//! when needed.
use crate::coretime_chain::runtime_types::pallet_broker::types::{ConfigRecord, SaleInfoRecord};
use subxt::{blocks::Block, OnlineClient, PolkadotConfig};
use types::ParaId;

const LOG_TARGET: &str = "tracker";
const RPC: &str = "wss://sys.ibp.network/coretime-kusama/";

#[subxt::subxt(runtime_metadata_path = "../../artifacts/kusama-coretime.scale")]
mod coretime_chain {}
use coretime_chain::broker::events as broker_events;

type Balance = u128;
type BlockNumber = u32;
type RelayBlockNumber = u32;

pub async fn track() -> Result<(), Box<dyn std::error::Error>> {
	let result = OnlineClient::<PolkadotConfig>::from_url(RPC).await;
	let Ok(client) = result else {
		log::error!(
			target: LOG_TARGET,
			"Failed to create an online client: {:?}",
			result
		);

		return Err("Failed to create an online client".into());
	};

	let sale_info = sale_info(&client).await?;
	let config = coretime_config(&client).await?;

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
		// Track everything we want to track:
		track_coretime_sales(&client, &block).await;
		track_interlude_phase(&block, interlude_start);
		track_leadin_phase(&block, leadin_start);
		track_fixed_phase(&block, fixed_phase_start);
	}

	Ok(())
}

async fn track_coretime_sales(
	client: &OnlineClient<PolkadotConfig>,
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
) -> Result<(), Box<dyn std::error::Error>> {
	// Check if a sale was made.
	let events = block.events().await.map_err(|_| "Failed to get events")?;
	let has = events.has::<broker_events::Purchased>().map_err(|_| "Event search failed")?;
	if has {
		let sale_info = sale_info(&client).await?;
		let available_cores = sale_info.cores_offered - sale_info.cores_sold;

		// TODO: if a specific number of cores are left trigger notification service.
	}

	Ok(())
}

fn track_interlude_phase(
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
	interlude_start: u32,
) {
	// Check if interlude started
	if block.header().number == interlude_start {
		// TODO: Trigger notifier
	}
}

fn track_leadin_phase(
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
	leadin_start: u32,
) {
	// Check if leadin started
	if block.header().number == leadin_start {
		// TODO: Trigger notifier
	}
}

fn track_fixed_phase(
	block: &Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
	fixed_phase_start: u32,
) {
	// Check if fixed phase started
	if block.header().number == fixed_phase_start {
		// TODO: Trigger notifier
	}
}

async fn sale_info(
	client: &OnlineClient<PolkadotConfig>,
) -> Result<SaleInfoRecord<Balance, BlockNumber>, Box<dyn std::error::Error>> {
	let sale_info_query = coretime_chain::storage().broker().sale_info();

	client
		.storage()
		.at_latest()
		.await?
		.fetch(&sale_info_query)
		.await?
		.ok_or("Failed to query sale info".into())
}

async fn coretime_config(
	client: &OnlineClient<PolkadotConfig>,
) -> Result<ConfigRecord<BlockNumber, RelayBlockNumber>, Box<dyn std::error::Error>> {
	let config_query = coretime_chain::storage().broker().configuration();

	client
		.storage()
		.at_latest()
		.await?
		.fetch(&config_query)
		.await?
		.ok_or("Failed to query sale info".into())
}
