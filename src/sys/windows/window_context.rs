// window_context.rs

pub fn classify_window_context<'a>(app_name: &str, title: &str) -> &'a str {
    let title = title.to_lowercase();
    let app_name = app_name.to_lowercase();

    if title.contains("code") || app_name.contains("code") {
        "coding"
    } else if title.contains("github") {
        "coding"
    } else if app_name.contains("libreoffice") || title.contains("word") {
        "writing"
    } else if app_name.contains("docs") || title.contains("document") || title.contains("docs") {
        "writing"
    } else if app_name.contains("terminal") || app_name.contains("powershell") {
        "coding"
    } else if title.contains("discord") || title.contains("slack") || title.contains("teams") || title.contains("zoom") {
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