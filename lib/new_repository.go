package gitbrws

import (
	"github.com/pkg/errors"
	"gopkg.in/src-d/go-git.v4"
	"os"
	"strings"
)

const InvalidRepoFormatMessage = `Invalid repository format. Valid format are one of below:
  user/repo      (e.g. rhysd/git-brws)
  host/user/repo (e.g. github.com/rhysd/git-brws)
  Git URL        (e.g. https://github.com/rhysd/git-brws.git)`

func normalizeRepoUrl(repo string) (string, error) {
	if len(repo) == 0 {
		return "", nil
	}

	if !strings.HasSuffix(repo, ".git") {
		repo = repo + ".git"
	}

	if strings.HasPrefix(repo, "git://") || strings.HasPrefix(repo, "https://") || strings.HasPrefix(repo, "http://") {
		return repo, nil
	}

	ns := strings.Count(repo, "/")

	if ns == 1 {
		// Note: user/repo (e.g. rhysd/git-brws)
		repo = "https://github.com/" + repo
		return repo, nil
	}

	if ns == 2 {
		// Note: host/user/repo (e.g. github.com/rhysd/git-brws)
		repo = "https://" + repo
		return repo, nil
	}

	return "", errors.New(InvalidRepoFormatMessage)
}

func NewRepository(url, dir string) (*git.Repository, error) {
	// When repository URL is specified, we might be able to get repository in memory
	// using git.Clone().
	var err error

	if url, err = normalizeRepoUrl(url); err != nil {
		return nil, err
	}

	if len(url) != 0 {
		r := git.NewMemoryRepository()
		if err := r.Clone(&git.CloneOptions{URL: url}); err != nil {
			return nil, errors.Wrap(err, "Failed to clone repository into memory for "+url)
		}
		return r, nil
	}

	if len(dir) == 0 {
		if dir, err = os.Getwd(); err != nil {
			return nil, err
		}
	}

	return git.NewFilesystemRepository(dir)
}
