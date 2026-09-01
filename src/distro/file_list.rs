#[cfg(feature = "archlinux")]
pub mod archlinux;

pub mod symlink_parse;

/**
	The public trait GetFileList is implemented by multiple backends
*/
pub trait GetFileList {
	async fn list(&self) -> Result<Vec<PackageFile>, Self::ListError>;

	type ListError;
}

/**
	The public enum PackageFile represents a file in a package.

	It describes several key information to implement the "copy" action.
*/
pub enum PackageFile {
	Regular {
		/**
			source_path describes the current file path on disk.
		*/
		source_path:	std::path::PathBuf,

		/**
			dest_path describes the destination to which the file would be installed.

			The parent path will be automatically created.
		*/
		dest_path:	std::path::PathBuf,
	},

	Symlink {
		/**
			dest_path describes the destination to which the link would be installed.

			The parent path will be automatically created.
		*/
		dest_path:	std::path::PathBuf,

		/**
			link_target should be a path describing the link target
		*/
		link_target:	std::path::PathBuf,
	}
}

impl PackageFile {
	/**
		Install the file or symbolic link into the package directory
	*/
	pub async fn copy(self) -> Result<(), std::io::Error> {
		match self {
			Self::Regular { source_path, dest_path }	=> {
				tokio::fs::copy(
					source_path,
					dest_path,
				)
					.await
					?;
				Ok(())
			}
			Self::Symlink { dest_path, link_target }	=> {
				tokio::fs::symlink(
					link_target,
					dest_path,
				)
					.await
					?;
				Ok(())
			}
		}
	}
}
