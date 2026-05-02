package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
)

func archPost (logger *log.Logger, act action) {
	var wg sync.WaitGroup
	srcdir := os.Getenv("srcdir")
	pkgdir := os.Getenv("pkgdir")
	pkgname := os.Getenv("pkgname")
	if len(srcdir) == 0 || len(pkgdir) == 0 || len(pkgname) == 0 {
		logger.Fatalln("Please declare srcdir, pkgdir, pkgname as global")
	}
	wg.Go(func() {
		if act.binOverlay {
			err := copyDir(
				path(filepath.Join(
					pkgdir,
					"/usr/lib/portable/info",
					act.appID,
					"bin",
				)),
				path(filepath.Join(
					pkgdir,
					"/usr/bin",
				)),
			)
			if err != nil {
				logger.Fatalln("Could not copy binaries to overlay:", err)
			}
		}

		err := os.RemoveAll(filepath.Join(pkgdir, "/usr/bin"))
		if err != nil {
			if os.IsNotExist(err) {
				return
			}
			logger.Fatalln("Could not remove /usr/bin in package root:", err)
		}
	})
	pathList := []string{
		"/usr/share/applications",
		"/etc/xdg/autostart",
		"/usr/share/dbus-1",
		"/usr/share/menu",
		"/usr/share/gnome-shell",
	}
	for _, path := range pathList {
		wg.Go(func() {
			err := os.RemoveAll(
				filepath.Join(
					pkgdir,
					path,
				),
			)
			if err != nil {
				if os.IsNotExist(err) {
					return
				}
				logger.Fatalln("Could not remove directory:", err)
			}
		})
	}
	wg.Wait()
	logger.Println("Cleaned holes")
	builder := strings.Builder{}
	builder.WriteString("#!/usr/bin/bash\n")
	if act.modernConfig {
		builder.WriteString("export PORTABLE_CONF=")
	} else {
		builder.WriteString("export _portableConfig=")
	}

	builder.WriteString(act.appID)
	builder.WriteString("\nexec portable --file-forwarding -- $@\n")
	reader := strings.NewReader(builder.String())
	err := os.MkdirAll(filepath.Join(pkgdir, "/usr/bin"), 0755)
	if err != nil {
		logger.Fatalln("Could not create stub file:", err)
	}
	file, err := os.OpenFile(
		filepath.Join(pkgdir, "/usr/bin", pkgname),
		os.O_CREATE|os.O_WRONLY|os.O_TRUNC,
		0755,
	)
	_, err = io.Copy(file, reader)
	if err != nil {
		logger.Fatalln("Could not create stub file:", err)
	}
	file.Close()

	if len(act.desktopFile) > 0 {
		ori, err := os.OpenFile(
			act.desktopFile,
			os.O_RDONLY,
			0700,
		)
		if err != nil {
			logger.Fatalln("Could not install .desktop file:", err)
		}
		err = os.MkdirAll(
			filepath.Join(pkgdir, "usr/share/applications/"),
			0755,
		)
		if err != nil {
			logger.Fatalln("Could not install .desktop file:", err)
		}
		dest, err := os.OpenFile(
			filepath.Join(pkgdir, "usr/share/applications/", act.appID + ".desktop"),
			os.O_CREATE|os.O_TRUNC|os.O_WRONLY,
			0644,
		)
		if err != nil {
			logger.Fatalln("Could not install .desktop file:", err)
		}
		_, err = io.Copy(dest, ori)
		if err != nil {
			logger.Fatalln("Could not install .desktop file:", err)
		}
		ori.Close()
		dest.Close()
		cmd := exec.Command(
			"desktop-file-validate",
			filepath.Join(pkgdir, "usr/share/applications/", act.appID + ".desktop"),
		)
		cmd.Stderr = os.Stderr
		cmd.Stdout = os.Stdout
		err = cmd.Run()
		if err != nil {
			file, err := os.Open(filepath.Join(pkgdir, "usr/share/applications/", act.appID + ".desktop"))
			if err != nil {
				logger.Fatalln("Could not open .desktop file:", err)
			}
			defer file.Close()
			scanner := bufio.NewScanner(file)
			for scanner.Scan() {
				fmt.Fprintln(os.Stdout, scanner.Text())
			}
			logger.Fatalln("Validation of desktop file failed!", err)
		}
		if ! act.busActivate {
			cmdline := []string{
				"--remove-key=DBusActivatable",
				filepath.Join(pkgdir, "usr/share/applications/", act.appID + ".desktop"),
			}
			cmd := exec.Command("desktop-file-edit", cmdline...)
			cmd.Stderr = os.Stderr
			err := cmd.Run()
			if err != nil {
				logger.Fatalln("Could not set DBusActivatable key:", err)
			}
		} else {
			cmdline := []string{
				"--set-key=DBusActivatable",
				"--set-value=true",
				filepath.Join(pkgdir, "usr/share/applications/", act.appID + ".desktop"),
			}
			cmd := exec.Command("desktop-file-edit", cmdline...)
			cmd.Stderr = os.Stderr
			err := cmd.Run()
			if err != nil {
				logger.Fatalln("Could not set DBusActivatable key:", err)
			}
		}
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
	if act.busActivate {
		instDBusService(logger, act, pkgdir)
	}
}