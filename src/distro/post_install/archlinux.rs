/**
	This struct implements post install function for Arch Linux
*/
pub struct ArchPost {
	pkgdir:	std::sync::Arc<std::path::PathBuf>,
}

impl Default for ArchPost {
	fn default() -> Self {
		let pkgdir = std::env::var("pkgdir").expect("Could not get pkgdir from env");

		Self {
			pkgdir:	std::sync::Arc::new(
				std::path::PathBuf::from(pkgdir)
			),
		}
	}
}


#[derive(thiserror::Error, Debug)]
pub enum ArchError {
	#[error("I/O error removing .desktop files: {0:#?}")]
	DesktopFileRmIOError(std::io::Error),

	#[error("I/O error installing .desktop file: {0:#?}")]
	DesktopFileInstallIOError(std::io::Error),

	#[error("I/O error installing app-private overlay")]
	OverlayInstallIOError(std::io::Error),

	#[error("I/O error removing binaries")]
	BinaryRemoveIOError(std::io::Error),

	#[error("I/O error creating stub binaries")]
	BinaryInstallIOError(std::io::Error),
}

async fn binary(
	pkgdir:		std::path::PathBuf,
	pkgname:	&str,
	app_id:		&str,
	overlay:	bool,
) -> Result<(), ArchError> {
	let binary_path = {
		let mut path = pkgdir.to_path_buf();
		path.push("usr");
		path.push("bin");
		path
	};

	let overlay_path = {
		let mut path = pkgdir;
		path.push("usr");
		path.push("lib");
		path.push("portable");
		path.push("info");
		path.push(app_id);
		path.push("bin");
		path
	};

	if binary_path.exists() {
		if overlay {
			if let Some(v) = overlay_path.parent() {
				tokio::fs::create_dir_all(v)
					.await
					.map_err(ArchError::OverlayInstallIOError)
					?;
			};

			tokio::fs::rename(
				&binary_path,
				&overlay_path,
			)
				.await
				.map_err(ArchError::OverlayInstallIOError)
				?;
		} else {
			tokio::fs::remove_dir_all(
				&binary_path,
			)
				.await
				.map_err(ArchError::BinaryRemoveIOError)
				?;
		}
	};

	tokio::fs::create_dir_all(&binary_path)
		.await
		.map_err(ArchError::BinaryInstallIOError)
		?;

	let shell_content = {
		let mut content = String::new();

		content.push_str("#!/usr/bin/bash");
		content.push_str("\n");

		content.push_str("export PORTABLE_CONF=");
		content.push_str(app_id);
		content.push_str("\n");

		content.push_str("exec portable --file-forwarding -- $@");
		content
	};

	let binary_path = {
		let mut path = binary_path;
		path.push(pkgname);
		path
	};

	let mut file = tokio::fs::OpenOptions::new()
		.read(false)
		.write(true)
		.create_new(true)
		.open(binary_path)
		.await
		.map_err(ArchError::BinaryInstallIOError)
		?;

	use tokio::io::AsyncWriteExt;

	file.write(
		shell_content.as_bytes()
	)
		.await
		.map_err(ArchError::BinaryInstallIOError)
		?;

	Ok(())
}

async fn desktop_file(
	pkgdir:		std::path::PathBuf,
	app_id:		&str,
	desktop_file:	std::path::PathBuf,
) -> Result<(), ArchError> {
	let desktop_path = {
		let mut path = pkgdir;
		path.push("usr");
		path.push("share");
		path.push("applications");
		path
	};

	if desktop_path.exists() {
		tokio::fs::remove_dir_all(&desktop_path)
			.await
			.map_err(ArchError::DesktopFileRmIOError)
			?
	};

	tokio::fs::create_dir_all(&desktop_path)
		.await
		.map_err(ArchError::DesktopFileInstallIOError)
		?;

	tokio::fs::copy(
		desktop_file,
		{
			let mut path = desktop_path.to_path_buf();
			let mut name = String::from(app_id);
			name.push_str(".desktop");
			path.push(&name);
			path
		},
	)
		.await
		.map_err(ArchError::DesktopFileInstallIOError)
		?;

	Ok(())
}
