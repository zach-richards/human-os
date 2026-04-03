// window_context.rs

fn classify_window(title: &str, domain: Option<String>) -> String {
    let title = title.to_lowercase();

    if title.contains("code") {
        "coding"
    } else if title.contains("github") {
        "coding"
    } else if title.contains("libreoffice") || title.contains("word") {
        "writing"
    } else if title.contains("docs") {
        "writing"
    } else if title.contains("terminal") || title.contains("powershell") {
        "coding"
    } else if title.contains("discord") || title.contains("slack") {
        "communication"
    } else if title.contains("spotify") || title.contains("music") {
        "music"
    } else if title.contains("youtube") || title.contains("netflix") || title.contains("reddit") {
        "distraction"
    } else if title.contains("chrome") || title.contains("firefox") {
        "browser"
    } else if title.contains("explorer") || title.contains("finder") || title.contains("files") {
        "file_management"
    } else {
        "other"
    }
}