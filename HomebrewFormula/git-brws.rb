class GitBrws < Formula
  version '0.11.0'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-apple-darwin.zip"
    sha256 'f5ab569946d46bc8e897504d8b4735e1c94f2fbd329937a63b30ca5031e029bb' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '248771d9f64cee4b320736b1a4372caeef4ad94dd599a1635c5c974d0e690731' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
