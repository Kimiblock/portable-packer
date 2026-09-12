enum OpMode {
	Help,
	Copy,
	Post,
}

enum Distro {
	Arch,
}

/**
	Get the user preference and possibly configuration data from cmdline
*/
pub async fn get_pref() -> super::OperationMode {
	let mut args = std::env::args();
	args.next();

	if args.len() <= 1 {
		return super::OperationMode::Help;
	};

	let mut mode: Option<OpMode> = None;
	let mut config: Option<super::config::Config> = None;
	let mut distro: Option<Distro> = None;
	let mut desktop_path: Option<std::path::PathBuf> = None;

	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--distro"		=> {
				match args.next().expect("Expected a distribution codename").as_str() {
					"arch" | "archlinux"	=> {
						distro = Some(Distro::Arch);
					}
					v			=> {
						unimplemented!("Distribution unimplemented: {v}");
					}
				}
			}
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
			"--desktop-file"	=> {
				desktop_path = Some(
					args.next().expect("Expected path after").into()
				)
			}
			v			=> {
				eprintln!("Unimplemented argument: {v:?}")
			}
		}
	};

	let runtime_options = super::RuntimeOptions {
		config:		{
			config.expect("Expected a configuration")
		},
		desktop_file:	desktop_path.expect("Expected a desktop file"),
	};

	match mode.unwrap_or(OpMode::Help) {
		OpMode::Help	=> {
			super::OperationMode::Help
		}
		OpMode::Copy	=> {
			match distro.expect("Expected a distribution") {
				Distro::Arch	=> {
					super::OperationMode::CopyArch {
						options: runtime_options,
					}
				}
			}
		}
		OpMode::Post	=> {
			match distro.expect("Expected a distribution") {
				Distro::Arch	=> {
					super::OperationMode::PostOnlyArch {
						options: runtime_options,
					}
				}
			}
		}
	}

}
