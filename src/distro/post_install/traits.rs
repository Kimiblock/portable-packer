pub trait PostInstall {
	/**
		Overlay the contents in bin into app-private copy of OverlayFS.

		Deletes original binaries, and generates a new stub one for launching the sandbox.

		The resulting String would be that binary name.
	*/
	async fn binary(&self, overlay: bool) -> Result<String, Self::PostError>;

	/**
		Removes any .desktop file that is installed in the package root.

		Installs the new one into package root.
	*/
	async fn desktop_file(
		&self,
		app_id:		&str,
		desktop_file:	std::path::PathBuf,
	) -> Result<(), Self::PostError>;

	type PostError;
}

