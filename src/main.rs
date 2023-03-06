extern crate mdbook;

use std::fs;
use std::path::Path;
use std::io::prelude::*;


fn scan_files(dir: &Path, html_files: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "html" {
                    let file_path = path.to_str().unwrap().to_string();
                    html_files.push(file_path.replacen("../html/", "", 1));                    
                }
            }
        } else if path.is_dir() {
            scan_files(&path, html_files);
        }
    }
}

fn generate_sitemap(domain: &str, html_files: &[String]) {
    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#,
        html_files
            .iter()
            .map(|file| format!("<url><loc>{}/{}</loc></url>",domain, file))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let mut file = fs::File::create("../html/sitemap.xml").unwrap();
    file.write_all(sitemap.as_bytes()).unwrap();
}

fn main() {
    let mut html_files = vec![];
    scan_files(Path::new("../"), &mut html_files);
    generate_sitemap("http://0.0.0.0:3000",&html_files);
}

