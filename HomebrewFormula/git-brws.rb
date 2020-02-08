class GitBrws < Formula
  version '0.11.7'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-apple-darwin.zip"
    sha256 '81836e709ac78a82f46081e2b3b2476d44f43dfaf16adc1c70646b41e6844778' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 'e1421dbe0d472b185a67b8693e8e889731b84b525600f5bdeb137eb24009863d' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
