package main

import (
	"os"
	"log"
)

func main() {
	logger := log.New(os.Stdout, "[Packer] ", 0)
	if len(os.Args) < 2 {
		help(logger)
		return
	}
	args := os.Args[1:]
	logger.Println("Got command line arguments:", args)


}