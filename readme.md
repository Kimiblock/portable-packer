# Dependencies
- desktop-file-utils


# Usage

```
[Packer] This is Portable packer, a tool to build sandboxed package
[Packer] Visit https://github.com/Kraftland/portable for documentation and information.
[Packer] Supported arguments:
[Packer] 	--distro [distro name]	-> Specify the distribution. (arch)
[Packer] 	--mode [copy/post]	-> Modes of operation
[Packer] 		copy [pkg]	-> Copy from existing package
[Packer] 		post		-> Post-package mode. Do clean ups, leak tests and install config + .desktop file.
[Packer] 	--config [path]	-	-> Specify the legacy configuration source for sandbox
[Packer] 	--config-ng [path]	-> Specify the TOML configuration source for sandbox
[Packer] 	--desktop-file [path]	-> Specify the desktop file path for sandbox
[Packer] 	--dbus-activation	-> Enables the activation from D-Bus (optional)
```

# Architecture

- Main package: higher level task-dispatches, control flow integration
	- file_list: Defines