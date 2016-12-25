package gitbrws

import (
	"github.com/pkg/errors"
	"github.com/skratchdot/open-golang/open"
)

type CmdOptions struct {
	Repo string
	Dir  string
}

type Command struct {
	Options *CmdOptions
}

func NewCommand(o *CmdOptions) *Command {
	return &Command{o}
}

func (cmd *Command) Open(args []string) error {
	u, err := cmd.URL(args)
	if err != nil {
		return err
	}
	return open.Run(u)
}

func (cmd *Command) URL(args []string) (string, error) {
	var u string

	r, err := NewRepository(cmd.Options.Repo, cmd.Options.Dir)
	if err != nil {
		return "", err
	}

	if u, err = CommitURL(r, args); err != nil {
		return "", err
	}
	if len(u) != 0 {
		return u, nil
	}

	return "", errors.New("Invalid arguments. Please see git-brws -help for usage")
}
