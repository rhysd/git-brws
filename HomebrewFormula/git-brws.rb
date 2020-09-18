class GitBrws < Formula
  version '0.11.12'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-apple-darwin.zip"
    sha256 '9ca73a57c8327e24fc648790fb6065dd4795db251b4e2cf27184144ee8d3b827' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 'c616b63355b0c659e8b4cba301129b3ba66bbf52e5b014cde070384fe4ec7ca3' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
