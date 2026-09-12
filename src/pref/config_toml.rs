pub async fn get(path: &std::path::PathBuf) -> super::config::Config {
	use super::config::Config;

	let mut file = tokio::fs::OpenOptions::new()
		.read(true)
		.write(false)
		.append(false)
		.truncate(false)
		.open(&path)
		.await
		.expect("Could not open TOML config");

	let content = {
		use tokio::io::AsyncReadExt;

		let mut content = String::new();

		file
			.read_to_string(&mut content)
			.await
			.expect("Could not read TOML config");

		content
	};

	let config: Config = toml::from_str(&content)
		.expect("Could not deserialise TOML config");

	config
}
