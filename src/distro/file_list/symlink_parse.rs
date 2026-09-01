/**
	Read the destination of a symbolic link
*/
pub async fn read_dest(path: &std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
	path.as_path().read_link()
}
