class Stylemd < Formula
  desc "Markdown styling toolkit — lint, format, fix, and polish your prose"
  homepage "https://github.com/leaf-kit/style.md"
  version "0.2.0"
  license "MIT"

  on_macos do
    url "https://github.com/leaf-kit/style.md/releases/download/v0.2.0/stylemd-x86_64-apple-darwin.tar.gz"
    sha256 "6b734d9521b189155b5866b48183970b3ec5a4e1cee4cc923835436477fb1af1"
  end

  def install
    bin.install "stylemd"
  end

  test do
    assert_match "stylemd #{version}", shell_output("#{bin}/stylemd --version")

    (testpath/"test.md").write("# Title\ntext   \n")
    output = shell_output("#{bin}/stylemd check #{testpath}/test.md 2>&1")
    assert_match "trailing", output.downcase

    system bin/"stylemd", "init"
    assert_predicate testpath/".stylemd.toml", :exist?
  end
end
