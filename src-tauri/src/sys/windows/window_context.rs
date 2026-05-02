// window_context.rs

// classifies window context by the name or title

pub fn classify_window_context<'a>(app_name: &str, title: &str) -> &'a str {
    let title = title.to_lowercase();
    let app_name = app_name.to_lowercase();

    // Match app names exactly or by known prefixes to avoid false positives
    // (e.g. "code" substring would match "Unicode", "encode", etc.)
    let is_code_app = matches!(
        app_name.as_str(),
        "code" | "vscodium" | "codium" | "zed" | "helix" | "nvim" | "neovim" | "vim" | "emacs"
        | "terminal" | "alacritty" | "kitty" | "wezterm" | "gnome-terminal" | "konsole"
        | "powershell" | "xterm"
    ) || app_name.starts_with("jetbrains") || app_name.starts_with("intellij");

    if is_code_app {
        "coding"
    } else if title.contains("github") || title.contains("gitlab") {
        "coding"
    } else if app_name.contains("libreoffice") || title.contains(" - word") {
        "writing"
    } else if app_name.contains("docs") || title.contains("document") || title.contains(" - docs") {
        "writing"
    } else if title.contains("discord") || title.contains("slack") || title.contains("teams") || title.contains("zoom") {
        "communication"
    } else if title.contains("spotify") || title.contains("music") || app_name.contains("spotify") {
        "music"
    } else if title.contains("youtube") || title.contains("netflix") || title.contains("reddit") || title.contains("steam") {
        "distraction"
    } else if app_name.contains("chrome") || app_name.contains("firefox") || app_name.contains("safari") || app_name.contains("brave") {
        "browser"
    } else if title.contains("explorer") || title.contains("finder") || title.contains("files") {
        "file_management"
    } else {
        "other"
    }
}