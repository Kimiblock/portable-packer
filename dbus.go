package main

import (
	"io"
	"log"
	"os"
	"path/filepath"
	"strings"
)

func instDBusService(logger *log.Logger, act action, prefixPath string) {
	if ! act.busActivate {
		return
	}
	builder := strings.Builder{}
	builder.WriteString("[D-BUS Service]\n")
	builder.WriteString("Name=" + act.appID + "\n")
	builder.WriteString("Exec=/usr/bin/env _portableConfig=" + act.appID + " portable --dbus-activation\n")
	path := filepath.Join(prefixPath, "usr/share/dbus-1/services", act.appID + ".service")
	err := os.MkdirAll(filepath.Dir(path), 0755)
	if err != nil {
		logger.Fatalln("Could not write D-Bus service:", err)
	}
	file, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		logger.Fatalln("Could not write D-Bus service:", err)
	}
	defer file.Close()
	reader := strings.NewReader(builder.String())
	_, err = io.Copy(file, reader)
	if err != nil {
		logger.Fatalln("Could not write D-Bus service:", err)
	}
}