class GitBrws < Formula
  version '0.11.2'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-apple-darwin.zip"
    sha256 '70cd910fec10e8a5eb1f2da3005024c7ad6d5468333b1a4034c91f3730548245' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '9795736ee1069f92e2d8875f7c0cd1cd864c8b837bf35cb83549ed20105f30a5' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
