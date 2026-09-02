/**
	This struct implements post install function for Arch Linux
*/
pub struct ArchPost {
	pkgdir:	std::sync::Arc<String>,
}

impl Default for ArchPost {
	fn default() -> Self {
		let pkgdir = std::env::var("pkgdir").expect("Could not get pkgdir from env");

		Self {
			pkgdir:	std::sync::Arc::new(pkgdir),
		}
	}
}
