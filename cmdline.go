package main

import (
	"bufio"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/BurntSushi/toml"
)

func cmdlineDispatcher (cmdline []string, logger *log.Logger) {
	logger.Println("Got command line arguments:", cmdline)
	var skip int
	type action struct {
		// post or copy
		act		string

		// Only arch
		distro		string
		configPath	string
		modernConfig	bool
		appID		string
		desktopFile	string

		busActivate	bool
		busArgs		[]string
	}
	var actionData action

	for idx := range cmdline {
		if skip > 0 {
			skip--
			continue
		}
		switch cmdline[idx] {
			case "--distro":
				if len(cmdline) <= idx + 1 {
					logger.Fatalln("Could not decode command line arguments: not enough arguments")
				}
				skip++
				switch cmdline[idx + 1] {
					case "arch":
						actionData.distro = "arch"
					default:
						logger.Fatalln("Unsupported distro:", cmdline[idx + 1])
				}
			case "--mode":
				if len(cmdline) <= idx + 1 {
					logger.Fatalln("Could not decode command line arguments: not enough arguments")
				}
				skip++
				switch cmdline[idx + 1] {
					case "copy", "post":
						actionData.act = cmdline[idx + 1]
					default:
						logger.Fatalln("Unsupported mode:", cmdline[idx + 1])
				}
			case "--config":
				logger.Println("Warning: The legacy KEY=VAL style configuration is deprecated")
				if len(cmdline) <= idx + 1 {
					logger.Fatalln("Could not decode command line arguments: not enough arguments")
				}
				skip++
				actionData.modernConfig = false
				path, err := filepath.Abs(cmdline[idx + 1])
				if err != nil {
					logger.Fatalln("Could not get absolute path of configuration:", err)
				}
				actionData.configPath = path
				file, err := os.OpenFile(path, os.O_RDONLY, 0700)
				if err != nil {
					logger.Fatalln("Could not read legacy configuration:", err)
				}
				scanner := bufio.NewScanner(file)
				for scanner.Scan() {
					line := scanner.Text()
					id, hasPrefix := strings.CutPrefix(line, "appID=")
					if hasPrefix {
						actionData.appID = id
						break
					}
				}
				file.Close()
			case "--config-ng":
				if len(cmdline) <= idx + 1 {
					logger.Fatalln("Could not decode command line arguments: not enough arguments")
				}
				skip++
				actionData.modernConfig = true
				path, err := filepath.Abs(cmdline[idx + 1])
				if err != nil {
					logger.Fatalln("Could not get absolute path of configuration:", err)
				}
				actionData.configPath = path
				file, err := os.OpenFile(path, os.O_RDONLY, 0700)
				if err != nil {
					logger.Fatalln("Could not read configuration:", err)
				}
				defer file.Close()
				reader := bufio.NewReader(file)
				decoder := toml.NewDecoder(reader)
				var conf Config
				md, err := decoder.Decode(&conf)
				file.Close()
				if err != nil {
					log.Fatalln("Could not read configuration:", err)
				}
				if len(md.Undecoded()) > 0 {
					log.Println("Undecoded keys:", md.Undecoded())
				}
				actionData.appID = conf.Metadata.AppID
			case "--desktop-file":
				if len(cmdline) <= idx + 1 {
					logger.Fatalln("Could not decode command line arguments: not enough arguments")
				}
				skip++
				path, err := filepath.Abs(cmdline[idx + 1])
				if err != nil {
					logger.Fatalln("Could not get absolute path of .desktop file:", err)
				}
				file, err := os.OpenFile(path, os.O_RDONLY, 0700)
				if err != nil {
					logger.Fatalln("Could not open .desktop file:", err)
				}
				cmd := exec.Command("desktop-file-validate", path)
				cmd.Stderr = os.Stderr
				cmd.Stdout = os.Stdout
				err = cmd.Run()
				if err != nil {
					logger.Println("Validating .desktop file failed!", err)
					time.Sleep(5 * time.Second)
				}
				file.Close()
			case "--dbus-arguments":
				if len(cmdline) <= idx + 1 {
					logger.Fatalln("Could not decode command line arguments: not enough arguments")
				}
				skip++
				actionData.busArgs = append(actionData.busArgs, cmdline[idx + 1])
			case "--dbus-activation":
				actionData.busActivate = true
		}
	}
}