package main

import (
	"flag"
	"fmt"
	"github.com/rhysd/git-brws/lib"
	"os"
)

const Usage = `Usage: git-brws {args} [{options}]

	TBW: Introduction

ARGS:
	TBW: Describe each arguments

FLAGS:`

const Version = "0.0.0"

func main() {
	help := flag.Bool("help", false, "Show this help")
	version := flag.Bool("version", false, "Show version")
	url := flag.String("url", "", "'user/repo' or URL repository you want to see")
	dir := flag.String("dir", "", "Path to directory of your repository")

	flag.Usage = func() {
		fmt.Println()
		fmt.Fprintln(os.Stderr, Usage)
		flag.PrintDefaults()
	}

	flag.Parse()

	if *help {
		fmt.Fprintln(os.Stderr, Usage)
		return
	}

	if *version {
		fmt.Println(Version)
		return
	}

	options := &gitbrws.Options{
		*url,
		*dir,
	}

	fmt.Println("TODO:", *options, flag.Args())
}
