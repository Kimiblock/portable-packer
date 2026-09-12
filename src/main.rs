#[derive(Debug, thiserror::Error)]
pub enum PackerError {}


#[tokio::main]
async fn main() -> Result<(), PackerError> {
	let preference = portable_packer::pref::cmdline::get_pref().await;
	println!("Got user preference: {preference:#?}");

	Ok(())
}
