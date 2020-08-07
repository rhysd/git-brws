class GitBrws < Formula
  version '0.11.10'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-apple-darwin.zip"
    sha256 '581274c1439472a7fef2370072b495fde1a19ded2a2f4b2f50eac86ddde4bc11' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '7be0cb4ae6a7ec8e87fe18cd243f2162e3463494319acfc17631c7d8bb550a45' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
