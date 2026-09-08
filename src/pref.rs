pub enum OperationMode {
	Help,
	Copy(RuntimeOptions),
	PostOnly(RuntimeOptions),
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
	pub sandbox_id:	String,
}
