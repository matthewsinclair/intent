# Intent.app's Homebrew cask, as a template. devbin's built-in `formula` renders
# it beside the formula (bin/.devbin/config.yaml, `formula.cask`), its sha256
# filled from the release's SHA256SUMS, and writes it to the tap as
# Casks/intent-app.rb, the token this file's name gives. The floor is the app's
# deployment target in native/macos/Intent/project.yml, as the symbol brew names
# that release by; `brew style` refuses the quoted ">= :tahoe" form.
cask "intent-app" do
  version "@@VERSION@@"
  sha256 "@@SHA256:Intent.app.zip@@"

  url "https://github.com/matthewsinclair/intent/releases/download/v#{version}/Intent.app.zip"
  name "Intent"
  desc "Menubar app for Intent's steel thread process"
  homepage "https://github.com/matthewsinclair/intent"

  depends_on formula: "matthewsinclair/intent/intent"
  depends_on macos: :tahoe

  app "Intent.app"
end
