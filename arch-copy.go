package main

import (
	"bufio"
	"io"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
)

func archCopy(act action, logger *log.Logger) {
	srcdir := os.Getenv("srcdir")
	pkgdir := os.Getenv("pkgdir")
	pkgname := os.Getenv("pkgname")
	if len(srcdir) == 0 || len(pkgdir) == 0 || len(pkgname) == 0 {
		logger.Fatalln("Please declare srcdir, pkgdir, pkgname as global")
	}
	sourcePkg := act.copyPkg
	if len(sourcePkg) == 0 {
		logger.Fatalln("Could not copy from package: not exist")
	}
	cmd := exec.Command("pacman", "-Ql", sourcePkg)
	cmd.Stderr = os.Stderr
	out, err := cmd.Output()
	if err != nil {
		logger.Fatalln("Could not obtain file list:", err)
	}
	scanner := bufio.NewScanner(strings.NewReader(string(out)))

	var pathList []string

	for scanner.Scan() {
		line := scanner.Text()
		sp := strings.Split(line, " ")
		if len(sp) < 2 {
			logger.Println("Invalid output: column mismatch")
			continue
		}
		pathList = append(pathList, strings.Join(sp[1:], ""))
	}
	logger.Println("Successfully obtained path list")

	var wg sync.WaitGroup
	for _, path := range pathList {
		wg.Go(func() {
			stat, err := os.Lstat(path)
			if err != nil {
				log.Fatalln("Could not stat package file:", err)
			}
			if stat.IsDir() {
				return
			}
			if stat.Mode()&os.ModeSymlink != 0 {
				log.Println("Processing symlink", stat.Name())
			}
			err = os.MkdirAll(filepath.Dir(filepath.Join(pkgdir, path)), 0755)
			if err != nil {
				log.Fatalln("Could not create directory:", err)
			}
			destFile, err := os.OpenFile(
				filepath.Join(pkgdir, path),
				os.O_TRUNC|os.O_WRONLY|os.O_CREATE,
				stat.Mode().Perm(),
			)
			if err != nil {
				log.Fatalln("Could not create file:", err)
			}
			defer destFile.Close()
			oriFile, err := os.OpenFile(
				path,
				os.O_RDONLY,
				stat.Mode().Perm(),
			)
			if err != nil {
				log.Fatalln("Could not open file:", err)
			}
			defer oriFile.Close()
			_, err = io.Copy(destFile, oriFile)
			if err != nil {
				log.Fatalln("Could not write file:", err)
			}
		})
	}
	wg.Wait()
}