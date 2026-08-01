package main

import (
	"errors"
	"io"
	"log"
	"os"
	"path/filepath"
	"sync"
)

type path string

func copyDir(dst path, src path, logger *log.Logger) error {
	var wg sync.WaitGroup
	var errChan = make(chan error, 512)
	wg.Go(func() {
		err := os.MkdirAll(string(dst), 0755)
		if err != nil {
			errChan <- err
		}
	})
	entries, err := os.ReadDir(string(src))
	if err != nil {
		return err
	}
	wg.Wait()
	for _, ent := range entries {
		entry := ent
		wg.Go(func() {
			info, err := entry.Info()
			if err != nil {
				errChan <- err
				return
			}
			mode := info.Mode().Perm()

			// Handle symlinks correctly
			if info.Mode()&os.ModeSymlink != 0 {
				dest, err := os.Readlink(
					filepath.Join(
						string(dst),
						entry.Name(),
					),
				)
				if err != nil {
					log.Fatalln("Could not read link destination:", err)
				}
				log.Println("Processing symlink", entry.Name(), dest)
				err = os.Symlink(
					dest,
					filepath.Join(string(src), entry.Name()),
				)
				return
			}

			if info.IsDir() {
				errChan <- errors.New("Packer does not support recursive directory")
				return
			}
			srcFile, err := os.Open(
				filepath.Join(string(src), entry.Name()),
			)
			if err != nil {
				errChan <- err
				return
			}
			defer srcFile.Close()
			dstFile, err := os.OpenFile(
				filepath.Join(
					string(dst),
					entry.Name(),
				),
				os.O_TRUNC|os.O_CREATE|os.O_WRONLY,
				mode,
			)
			if err != nil {
				errChan <- err
				return
			}
			defer dstFile.Close()
			_, err = io.Copy(dstFile, srcFile)
			if err != nil {
				errChan <- err
			}
			logger.Println("Copied",
				filepath.Join(string(src), entry.Name()),
				"to",
				filepath.Join(
					string(dst),
					entry.Name(),
				))
		})
	}
	wg.Wait()
	close(errChan)
	for sig := range errChan {
		if sig != nil {
			return sig
		}
	}
	return nil
}