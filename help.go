package main

import ("log")

func help (logger *log.Logger) {
	logger.Println("This is Portable packer, a tool to build sandboxed package")
	logger.Println("Visit https://github.com/Kraftland/portable for documentation and information.")
	logger.Println("Supported arguments:")
	logger.Println("	--distro [distro name]	-> Specify the distribution. (arch)")
	logger.Println("	--mode [copy/post]	-> Modes of operation")
	logger.Println("		copy [pkg]	-> Copy from existing package")
	logger.Println("		post		-> Post-package mode. Do clean ups, leak tests and install config + .desktop file.")
	//logger.Println("	--hash [true / false]	-> Enables hashing of configuration file. Currently has no effect. (optional)")
	logger.Println("	--config [path]	-	-> Specify the legacy configuration source for sandbox")
	logger.Println("	--config-ng [path]	-> Specify the TOML configuration source for sandbox")
	logger.Println("	--desktop-file [path]	-> Specify the desktop file path for sandbox")
	logger.Println("	--dbus-activation	-> Enables the activation from D-Bus (optional)")
}