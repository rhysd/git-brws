class GitBrws < Formula
  version '0.11.6'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-apple-darwin.zip"
    sha256 'c700385a3a470fcc137e2bc71d8c1a2b3ff5723a7da0cb081859b18036076d35' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '2c900c315a059dbd3079f5be781ef674bf0c124d1254cc986ba1c127ae4fa406' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
