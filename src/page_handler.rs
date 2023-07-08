// src/page_handler.rs

use actix_web::{ web, HttpResponse, Responder };
use pulldown_cmark::{ html, Options, Parser };
use std::fs;
use std::path::PathBuf;
use crate::root_handler::FrontMatter;

pub async fn page(path: web::Path<PathBuf>) -> impl Responder {
    let page_path = format!("routes/{}.md", path.as_path().to_str().unwrap());
    if let Ok(markdown_file) = fs::read_to_string(&page_path) {
        let split: Vec<&str> = markdown_file.splitn(3, "---").collect();
        if split.len() == 3 {
            let front_matter: FrontMatter = serde_yaml::from_str(split[1]).unwrap();
            // println!("{:?}", front_matter);

            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(split[2], options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            // Load template
            let template = fs::read_to_string("template.html").unwrap();

            // Replace placeholders in template
            let html_output = template
                .replace("{{title}}", &front_matter.title)
                .replace("{{slug}}", &front_matter.slug)
                .replace("{{description}}", front_matter.description.as_deref().unwrap_or(""))
                .replace("{{keywords}}", front_matter.keywords.as_deref().unwrap_or(""))
                .replace("{{author}}", front_matter.author.as_deref().unwrap_or(""))
                .replace("{{content}}", &html_output);

            HttpResponse::Ok().body(html_output)
        } else {
            HttpResponse::NotFound().body(format!("Invalid Markdown file format: {}", &page_path))
        }
    } else {
        HttpResponse::NotFound().body(format!("Page not found: {}", &page_path))
    }
}
