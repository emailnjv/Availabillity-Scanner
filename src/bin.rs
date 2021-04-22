//! A simple tool born out of frustration.
//!
//! This script allows unfortunate residents of New Jersey that are in current need of an appointment at their DMV location for whatever reason to get a text message when an appointment has become available.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match dmv_api::run().await {
		Ok(_) => Ok(()),
		Err(e) => std::panic::panic_any(e),
	}
}
