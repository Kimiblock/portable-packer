enum OpMode {
	Help,
	Copy,
	Post,
}

/**
	Get the user preference and possibly configuration data from cmdline
*/
pub async fn get_pref() -> super::OperationMode {
	let args = std::env::args();

	if args.len() <= 1 {
		return super::OperationMode::Help;
	};

	let mut mode: Option<OpMode>;
	let mut config: Option<super::config::Config>;

	while let Some(arg) = args.skip(1).next() {
		match arg.as_str() {
			"--mode"		=> {
				match args.next().expect("Expected argument after --mode").as_str() {
					"copy"	=> {
						mode = Some(OpMode::Copy);
					}
					"post"	=> {
						mode = Some(OpMode::Post);
					}
					v	=> {
						panic!("Could not parse cmdline: unexpected argument {v:?} after --mode")
					}
				};
			}
			"--config"		=> {
				eprintln!("Legacy configuration is deprecated in Portable 14");
				let path: std::path::PathBuf = args.next().expect("Expected path after --config").into();

				config = Some(
					super::config_legacy::get(&path)
						.await
				)

			}
			"--config-ng"		=> {
				let path: std::path::PathBuf = args.next().expect("Expected path after --config-ng").into();

				config = Some(super::config_toml::get(&path).await)
			}
			"--desktop-file"	=> {}
		}
	};

}
