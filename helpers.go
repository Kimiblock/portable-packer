package main

import (
	"errors"
	"io"
	"os"
	"path/filepath"
	"sync"
)

type path string

func copyDir(dst path, src path) error {
	var wg sync.WaitGroup
	var errChan = make(chan error, 512)
	defer func () {
		close(errChan)
	} ()
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
		})
	}
	wg.Wait()
	for sig := range errChan {
		if sig != nil {
			return sig
		}
	}
	return nil
}