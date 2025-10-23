use regex::Regex;
use std::process::Command;

pub fn check_openssl() {
    Command::new("openssl")
        .arg("-v")
        .output()
        .expect("No openssl found. Please install openssl.");
}

pub fn check_xmake() {
    Command::new("xmake")
        .arg("--help")
        .output()
        .expect("No xmake found. Please install xmake.");
}

pub fn check_xrepo() {
    let stdout = String::from_utf8(
        execute::shell("xrepo --version")
            .output()
            .expect("Error invoking shell, check your environment.")
            .stdout,
    )
    .expect("Failed to convert xrepo output to string.");

    if !stdout.contains("xRepo") {
        panic!("No xrepo found. Please install xrepo. Output: {}.", stdout);
    }
}

fn extract_paths(regex: &Regex, data: &str) -> Vec<String> {
    let mut paths = Vec::new();
    if let Some(captures) = regex.captures(data)
        && let Some(content) = captures.get(1)
    {
        let quoted_paths: Vec<&str> = content
            .as_str()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for quoted_path in quoted_paths {
            let path = quoted_path.trim_matches('"');
            paths.push(path.to_string());
        }
    }
    paths
}

pub fn xrepo_fetch(dep: &str) -> (Vec<String>, Vec<String>) {
    let stdout = String::from_utf8(strip_ansi_escapes::strip(
        execute::shell(format!("xrepo fetch {}", dep))
            .output()
            .unwrap_or_else(|_| panic!("Failed to fetch dependency: {}.", dep))
            .stdout,
    ))
    .expect("Failed to convert xrepo output to string.");
    let links = extract_paths(
        &Regex::new(r#"[^sys]links\s*=\s*\{\s*((?:"[^"]*",?\s*)*)\s*\}"#).unwrap(),
        &stdout,
    );
    let link_dirs = extract_paths(
        &Regex::new(r#"linkdirs\s*=\s*\{\s*((?:"[^"]*",?\s*)*)\s*\}"#).unwrap(),
        &stdout,
    );

    (links, link_dirs)
}
