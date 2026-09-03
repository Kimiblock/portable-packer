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
}

async fn desktop_file(
	post:		&ArchPost,
	app_id:		&str,
	desktop_file:	std::path::PathBuf,
) -> Result<(), ArchError> {
	let desktop_path = {
		let mut path = post.pkgdir.to_path_buf();
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
