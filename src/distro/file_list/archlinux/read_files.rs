


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
