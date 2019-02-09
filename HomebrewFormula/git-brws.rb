class GitBrws < Formula
  version '0.9.2'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-apple-darwin.zip"
    sha256 '31e552c7fdd254c6c04487c87c958c6530354de4eb78e45dc078baea5630b7bc' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '1c368373d56b3bb74b92a1b01572e0662aec5e6f97f68c7e2fc971f5db3188cb' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
