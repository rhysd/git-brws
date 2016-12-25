package main

import (
	"flag"
	"fmt"
	"github.com/rhysd/git-brws/lib"
	"os"
)

const Usage = `Usage: git-brws {args} [{flags}]

  TBW: Introduction

ARGS:
  TBW: Describe each arguments

FLAGS:`

const Version = "0.0.0"

func main() {
	help := flag.Bool("help", false, "Show this help")
	version := flag.Bool("version", false, "Show version")
	repo := flag.String("repo", "", "'user/repo' or .git URL repository you want to see")
	url := flag.Bool("url", false, "Output URL to stdout instead of opening it in browser")
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

	c := gitbrws.NewCommand(&gitbrws.CmdOptions{
		*repo, *dir,
	})

	fmt.Println("TODO:", *c, flag.Args())

	if *url {
		o, err := c.URL(flag.Args())
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(3)
		}
		fmt.Println(o)
		return
	}

	if err := c.Open(flag.Args()); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(3)
	}
}
