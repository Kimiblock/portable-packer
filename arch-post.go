package main

import (
	"io"
	"log"
	"os"
	"path/filepath"
)

func archPost (logger *log.Logger, act action) {
	srcdir := os.Getenv("srcdir")
	pkgdir := os.Getenv("pkgdir")
	pkgname := os.Getenv("pkgname")
	if len(srcdir) == 0 || len(pkgdir) == 0 || len(pkgname) == 0 {
		logger.Fatalln("Please declare srcdir, pkgdir, pkgname as global")
	}
	if len(act.configPath) > 0 {
		logger.Println("Installing configuration")
		config, err := os.OpenFile(act.configPath, os.O_RDONLY, 0700)
		if err != nil {
			logger.Fatalln("Could not open configuration:", err)
		}
		defer config.Close()
		var confDest string
		confDest = filepath.Join(pkgdir, "usr/lib/portable/info", act.appID)
		switch act.modernConfig {
			case false:
				confDest = filepath.Join(confDest, "config")
			case true:
				confDest = filepath.Join(confDest, "config.toml")
		}
		err = os.MkdirAll(filepath.Dir(confDest), 0755)
		if err != nil {
			logger.Fatalln("Could not write configuration:", err)
		}
		dest, err := os.OpenFile(confDest, os.O_WRONLY|os.O_TRUNC|os.O_CREATE, 0755)
		if err != nil {
			logger.Fatalln("Could not write configuration:", err)
		}
		defer dest.Close()
		_, err = io.Copy(dest, config)
		if err != nil {
			logger.Fatalln("Could not write configuration:", err)
		}
	}

}