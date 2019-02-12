class GitBrws < Formula
  version '0.10.1'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-apple-darwin.zip"
    sha256 'cb0a18d772aaeb10b9d61ec7acf553403263ab0ce1cbe18dd66a0ba2d4b7ba61' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/#{version}/git-brws-#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '1439f0a6d8fbb1d8669b8c44a65a9cdbc01c97c97b94953898d815a6cb9d338e' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
