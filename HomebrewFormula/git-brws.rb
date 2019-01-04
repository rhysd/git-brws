class GitBrws < Formula
  version '0.9.1'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-apple-darwin.zip"
    sha256 'c7d899a1879baa234c0c957179c613b2d1ed6fbf3c40543afbd2be83f0a6580f'
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 'fe72552912d1f753c6a54803f28e1209848900a7b4d5ea49d2b6bd31b8e529f3'
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
