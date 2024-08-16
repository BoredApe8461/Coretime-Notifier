use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod api;

/// Different events to which a user can subscribe to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash)]
#[serde(crate = "rocket::serde")]
pub enum Notifications {
	/// Notifications for interlude phase.
	InterludePhase(PhaseNotification),
	/// Notifications for leadin phase.
	LeadinPhaseStart(PhaseNotification),
	/// Notifications for fixed price phase.
	FixedPhaseStart(PhaseNotification),
	/// Whenever coretime is sold.
	CoretimeSale,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(crate = "rocket::serde")]
pub enum PhaseNotification {
	/// Getting a notification `u64` seconds prior to phase start.
	PriorStart(u64),
	/// Getting a notification `u64` seconds prior to phase end.
	PriorEnd(u64),
}

/// Available options for receiving notification prior to an event happening.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(crate = "rocket::serde")]
pub enum TimeOptions {
	/// Receive a notification day before the phase starts.
	DayAhead = 86400,
	/// Receive a notification 12 hours before the phase starts.
	HalfDayAhead = 43200,
	/// Receive a notification 6 hours before the phase starts.
	QuarterDayAhead = 21600,
	/// Receive a notification one hour before the phase starts.
	HourAhead = 3600,
}

/// The available methods to receive a notification.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(crate = "rocket::serde")]
pub enum Notifier {
	// User will receive notifications via their email.
	Email,
	// User will receive notifications via their telegram.
	Telegram,
	/// If `Null` user will not receive notifications.
	Null,
}
