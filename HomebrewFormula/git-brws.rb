class GitBrws < Formula
  version '0.11.11'
  desc 'Command line tool to open repository, file, commit, diff, pull request or issue in browser'
  homepage 'https://github.com/rhysd/git-brws'

  if OS.mac?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-apple-darwin.zip"
    sha256 '7eff99b0c390517b7061dadf4dc64ce5ab928645f93cbe80faa1a81c306af5b2' # mac
  elsif OS.linux?
    url "https://github.com/rhysd/git-brws/releases/download/v#{version}/git-brws-v#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 '3d3056bb3f0614e397e83be6510ebd308b556871b0d7cc68aa97e99dda8d24bd' # linux
  end

  def install
    bin.install 'git-brws'
    man1.install 'git-brws.1'
  end

  test do
    system "#{bin}/git-brws", '--version'
  end
end
