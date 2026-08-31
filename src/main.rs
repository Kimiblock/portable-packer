#[derive(Debug, thiserror::Error)]
pub enum PackerError {}


#[tokio::main]
async fn main() -> Result<(), PackerError> {
	Ok(())
}
