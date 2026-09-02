pub trait PostInstall {
	/**
		Overlay the contents in bin into app-private copy of OverlayFS.

		Deletes original binaries, and generates a new stub one for launching the sandbox.
	*/
	async fn binary(&self, overlay: bool) -> Result<(), Self::PostError>;

	type PostError;
}
