/**
	This struct implements post install function for Arch Linux
*/
pub struct ArchPost {
	pkgdir:		std::sync::Arc<std::path::PathBuf>,
	pkgname:	std::sync::Arc<String>,
}

impl Default for ArchPost {
	fn default() -> Self {
		let pkgdir = std::env::var("pkgdir").expect("Could not get pkgdir from env");
		let pkgname = std::env::var("pkgname")
			.expect("Could not get pkgname from env");

		Self {
			pkgdir:	std::sync::Arc::new(
				std::path::PathBuf::from(pkgdir)
			),
			pkgname: std::sync::Arc::new(
				pkgname.into()
			)
		}
	}
}

impl super::traits::PostInstall for ArchPost {
	async fn binary(&self, app_id: std::sync::Arc<String>, overlay: bool) -> Result<String, Self::PostError> {
		binary(self.pkgdir.to_path_buf(), self.pkgname.clone(), app_id.clone(), overlay).await
	}
	async fn gnome_shell(
			&self,
		) -> Result<(), Self::PostError>
	{
		gnome_shell(
			self.pkgdir.to_path_buf(),
		).await
	}
	async fn desktop_file(
			&self,
			app_id:		std::sync::Arc<String>,
			desktop_path:	std::path::PathBuf,
		) -> Result<(), Self::PostError>
	{
		desktop_file(
			self.pkgdir.to_path_buf(),
			app_id,
			desktop_path,
		).await
	}
	async fn dbus_service(
			&self,
			app_id:		std::sync::Arc<String>,
			generate:	bool,
		) -> Result<(), Self::PostError>
	{
		dbus_service(
			self.pkgdir.to_path_buf(),
			app_id,
			generate,
		).await
	}

	type PostError = ArchError;
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

	#[error("I/O error removing D-Bus services")]
	BusRmIOError(std::io::Error),

	#[error("I/O error installing D-Bus services")]
	BusInstallIOError(std::io::Error),

	#[error("I/O error cleaning GNOME Shell paths")]
	GNOMEShellIOError(std::io::Error),
}

async fn binary(
	pkgdir:		std::path::PathBuf,
	pkgname:	std::sync::Arc<String>,
	app_id:		std::sync::Arc<String>,
	overlay:	bool,
) -> Result<String, ArchError> {
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
		path.push(app_id.as_str());
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
		content.push_str(app_id.as_str());
		content.push_str("\n");

		content.push_str("exec portable --file-forwarding -- $@");
		content
	};

	let binary_path = {
		let mut path = binary_path;
		path.push(pkgname.as_str());
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

	Ok(pkgname.to_string())
}

async fn desktop_file(
	pkgdir:		std::path::PathBuf,
	app_id:		std::sync::Arc<String>,
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
			let mut name = String::from(app_id.as_str());
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

async fn dbus_service(
	pkgdir:		std::path::PathBuf,
	app_id:		std::sync::Arc<String>,
	generate:	bool,
) -> Result<(), ArchError> {
	let dbus_service_path = {
		let dbus_path: std::path::PathBuf = [
			"usr",
			"share",
			"dbus-1",
			"services",
		]
			.iter()
			.collect();
		pkgdir.join(dbus_path)
	};

	if tokio::fs::try_exists(&dbus_service_path).await.map_err(ArchError::BusRmIOError)? {
		tokio::fs::remove_dir_all(&dbus_service_path)
			.await
			.map_err(ArchError::BusRmIOError)
			?;
	};

	if ! generate {
		return Ok(());
	};

	tokio::fs::create_dir_all(&dbus_service_path)
		.await
		.map_err(ArchError::BusInstallIOError)
		?;

	let service_content = {
		let mut content = String::new();
		content.push_str("[D-BUS Service]");
		content.push_str("\n");
		content.push_str("Name=");
		content.push_str(&app_id);
		content.push_str("\n");

		content.push_str("Exec=/usr/bin/env PORTABLE_CONF=");
		content.push_str(&app_id);
		content.push_str(" portable --dbus-activation");

		content.push_str("\n");
		content
	};

	let file_path = {
		let mut path = dbus_service_path.to_path_buf();

		let basename = {
			let mut name = String::from(app_id.as_str());
			name.push_str(".service");
			name
		};

		path.push(&basename);
		path
	};

	let mut file = tokio::fs::OpenOptions::new()
		.read(false)
		.write(true)
		.create_new(true)
		.open(file_path)
		.await
		.map_err(ArchError::BusInstallIOError)
		?;

	use tokio::io::AsyncWriteExt;

	file
		.write(service_content.as_bytes())
		.await
		.map_err(ArchError::BusInstallIOError)
		?;
	Ok(())
}

async fn gnome_shell(
	pkgdir:		std::path::PathBuf,
) -> Result<(), ArchError> {
	let shell_path = {
		let shared_path: std::path::PathBuf = [
			"usr",
			"share",
			"gnome-shell",
		].iter().collect();

		pkgdir.join(shared_path)
	};

	if tokio::fs::try_exists(&shell_path)
		.await
		.map_err(ArchError::GNOMEShellIOError)
		?
	{
		tokio::fs::remove_dir_all(&shell_path)
			.await
			.map_err(ArchError::GNOMEShellIOError)
			?;
	};

	Ok(())
}

