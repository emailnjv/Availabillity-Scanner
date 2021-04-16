#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match dmv_api::run().await {
		Ok(_) => Ok(()),
		Err(e) => std::panic::panic_any(e),
	}
}
