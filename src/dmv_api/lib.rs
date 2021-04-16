use std::fmt::Error;

mod client;
mod sms;
mod structs;
mod utils;

pub async fn run() -> Result<(), Error> {
	let client = client::Client::new();
	client.run().await;
	Ok(())
}
