package gitbrws

import (
	"fmt"
	"gopkg.in/src-d/go-git.v4"
)

func CommitURL(repo *git.Repository, args []string) (string, error) {
	fmt.Println(repo.IsEmpty())
	return "", nil
}
