mod read_files;

/**
	Implements the GetFileList trait
*/
pub enum Arch {
	LocalPackage {
		pkgname:	String
	},
}

#[derive(Debug, thiserror::Error)]
pub enum ArchError {
	#[error("I/O error reading file: {0:#?}")]
	IOError(std::io::Error),

	#[error("I/O error listing pacman database: {0:#?}")]
	DbIOError(std::io::Error),

	#[error("I/O error reading pacman database desc: {0:#?}")]
	DbDescIOError(std::io::Error),

	#[error("I/O error reading pacman database files: {0:#?}")]
	DbFilesIOError(std::io::Error),

	#[error("No relevant entry in pacman database")]
	NoSuchPackageInDatabase,

	#[error("Error converting OsString to &str: {0:#?}")]
	OsStringError(std::ffi::OsString),
}

impl crate::distro::file_list::GetFileList for Arch {
	async fn list(&self) -> Result<Vec<super::PackageFile>, Self::ListError> {

	}

	type ListError = ArchError;
}
