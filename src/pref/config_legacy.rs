pub async fn get(path: &std::path::PathBuf) -> super::config::Config {
	let mut file = tokio::fs::OpenOptions::new()
		.read(true)
		.write(false)
		.create(false)
		.open(path)
		.await
		.expect("Could not open legacy config");

	let raw_content = {
		let mut content = String::new();

		use tokio::io::AsyncReadExt;

		file
			.read_to_string(&mut content)
			.await
			.expect("Could not read legacy config");

		content
	};

	let deserialised_config: portable_legacy_conf::Config = portable_legacy_conf::from_str(&raw_content)
		.expect("Could not deserialise legacy config");

	let mut dev_allow = vec![];

	if deserialised_config.game {
		dev_allow.push(super::config::DeviceAllow::DiscreteGPU);
	};
	if deserialised_config.camera {
		dev_allow.push(super::config::DeviceAllow::Camera);
	};
	if deserialised_config.input_dev {
		dev_allow.push(super::config::DeviceAllow::Input);
	};

	use super::config::*;

	Config {
		metadata: Metadata {
			sandbox_id:		deserialised_config.app_id,
			display_name:		deserialised_config.friendly_name,
			state_directory:	deserialised_config.state_dir,
		},
		exec: Exec {
			target:			deserialised_config.target.0,
			arguments:		deserialised_config.target.1.unwrap_or(vec![]),
			overlay:		false,
		},
		dbus_activation: BusExec::default(),
		system: SysMgmt {
			allow_inhibit:		false,
			conduct_inhibit:	false,
			uclamp_max:		100,
			device_allow:		dev_allow,
		},
		network: Network {
			allow_network:		deserialised_config.bind_network,
			enable_filter:		false,
			block_dest:		vec![],
		},
		privacy: Privacy {
			lockdown_options:	LockdownConfig::default(),
			x11_compat:		! deserialised_config.wayland,
			classic_notif:		false,
			push_notification:	false,
		},
		advanced: Advanced {
			use_zink:		deserialised_config.zink,
			qt5_compat:		deserialised_config.qt5,
			mpris_names: 		vec![],
			tray_wake: 		deserialised_config.tray_wake,
			allow_kde_status:	false,
			flatpak_env:		deserialised_config.flatpak_info,
			allow_debug:		false,
		},
	}
}
