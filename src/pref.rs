pub mod cmdline;
pub mod config;
pub mod config_toml;
pub mod config_legacy;

pub enum OperationMode {
	Help,
	CopyArch{
		options:	RuntimeOptions,
	},
	PostOnlyArch{
		options:	RuntimeOptions,
	},
}

/**
	The public struct RuntimeOptions describes both decoded configuration and options parsed from
		command line arguments
*/
pub struct RuntimeOptions {
	/**
		The sandbox_id is the equivalent of config's sandbox_id

		It represents a unique, fixed identity of a sandbox.
	*/
	pub config:		crate::pref::config::Config,

	/**
		The path for supplied .desktop file (currently single)
	*/
	pub desktop_file:	std::path::PathBuf,
}
