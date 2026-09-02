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

	/**
		Removes any D-Bus service installed in package root.

		Generates a new one and installs them if enabled.
	*/
	async fn dbus_service(
		&self,
		app_id:		&str,
		generate:	bool,
	) -> Result<(), Self::PostError>;

	/**
		Removes any GNOME Shell Extensions, Modes installed in the system.

		Search Provider is preserved, with sandbox_id.ini being the name.
	*/
	async fn gnome_shell(
		&self,
		app_id:		&str,
	) -> Result<(), Self::PostError>;

	type PostError;
}

