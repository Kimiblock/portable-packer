/**
	Gets a list of PathBuf from pacman database, assumes dbroot as default
*/
pub async fn get(pkgname: &str) -> Result<Vec<std::path::PathBuf>, super::ArchError> {
	let package_root = match_pkg_database(&pkgname).await?;

	let files_path = {
		let mut path = package_root.to_path_buf();
		path.push("desc");
		path
	};

	let mut file = tokio::fs::OpenOptions::new()
		.read(true)
		.write(false)
		.open(files_path)
		.await
		.map_err(super::ArchError::DbFilesIOError)
		?;



	let content = {
		let mut buf = String::new();
		use tokio::io::AsyncReadExt;
		file
			.read_to_string(&mut buf)
			.await
			.map_err(super::ArchError::DbFilesIOError)
			?;
		buf
	};

	let mut paths = vec![];

	{
		for line in content.split("\n") {
			let line = line.trim();

			if line.is_empty() {
				continue;
			};

			paths.push(
				std::path::PathBuf::from(line)
			);
		}
	};

	Ok(paths)
}


/// Find the path for pacman database
async fn match_pkg_database(pkgname: &str) -> Result<std::path::PathBuf, super::ArchError> {
	let db_prefix = {
		let path = std::path::PathBuf::from(
			"/var/lib/pacman/local/"
		);
		path
	};

	let mut entries = tokio::fs::read_dir(&db_prefix)
		.await
		.map_err(super::ArchError::DbIOError)
		?;

	loop {
		let entry = match entries
			.next_entry()
			.await
			.map_err(super::ArchError::DbIOError)
			?
		{
			Some(v)	=> v,
			None	=> return Err(
				super::ArchError::NoSuchPackageInDatabase
			)
		};

		let name = entry.file_name();
		let name = match name.to_str() {
			Some(v)	=> {
				if ! v.starts_with(&pkgname) {
					continue;
				};
				v
			}
			None	=> {
				return Err(
					super::ArchError::OsStringError(name)
				);
			}
		};

		use tokio::io::AsyncReadExt;

		let mut desc = {
			let mut desc = std::path::PathBuf::from(&db_prefix);
			desc.push(&name);

			tokio::fs::OpenOptions::new()
				.read(true)
				.write(false)
				.open(&desc)
				.await
				.map_err(super::ArchError::DbDescIOError)
				?
		};

		let mut content = String::new();

		desc
			.read_to_string(&mut content)
			.await
			.map_err(super::ArchError::DbDescIOError)
			?;

		let lines = content.split("\n");

		let mut is_name = false;
		for line in lines {
			if line.trim().is_empty() {
				continue;
			};

			if line.trim() == "%NAME%" {
				is_name = true;
			};

			if is_name {
				if pkgname == line.trim() {
					let mut path = db_prefix.to_path_buf();
					path.push(&name);
					return Ok(path);
				} else {
					continue;
				};
			}
		};
	}
}
