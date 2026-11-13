cask "lumela" do
  version "0.2.0"
  sha256 "cc39ef4cf9f03da3b46f5a39517ad12eb5088d57e07aa8f23697bfa9f218821e"

  url "https://github.com/tw4/lumela/releases/download/v#{version}/Lumela_#{version}_universal.dmg"
  name "Lumela"
  desc "Screenshot tool: capture, beautify, annotate, redact and copy text"
  homepage "https://github.com/tw4/lumela"

  livecheck do
    url :url
    strategy :github_latest
  end

  auto_updates true

  app "Lumela.app"

  # The app is not notarized yet; without this macOS refuses to open it.
  postflight do
    system_command "/usr/bin/xattr", args: ["-dr", "com.apple.quarantine", "#{appdir}/Lumela.app"]
  end

  zap trash: [
    "~/Library/Application Support/app.lumela.desktop",
    "~/Library/Caches/app.lumela.desktop",
    "~/Library/Preferences/app.lumela.desktop.plist",
    "~/Library/WebKit/app.lumela.desktop",
  ]
end
