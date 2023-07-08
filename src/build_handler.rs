use std::fs;
use std::path::{ Path };
use pulldown_cmark::{ html, Options, Parser };
use serde::Deserialize;
use serde_yaml;

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    slug: String,
    description: Option<String>,
    keywords: Option<String>,
    author: Option<String>,
}

pub fn build_static_files() -> Result<(), Box<dyn std::error::Error>> {
    // Create the output directory for the static HTML files
    fs::create_dir_all("out")?;

    // Copy static assets to the output directory
    copy_static_assets()?;

    // Read the Markdown files in the routes directory
    let entries = fs::read_dir("routes")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some("md".as_ref()) {
            let output_dir = Path::new("out");
            let output_file = output_dir.join(path.file_stem().unwrap()).with_extension("html");

            // Parse Markdown and convert to HTML
            let markdown_file = fs::read_to_string(&path)?;
            let split: Vec<&str> = markdown_file.splitn(3, "---").collect();
            if split.len() == 3 {
                let front_matter: FrontMatter = serde_yaml::from_str(split[1])?;
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(split[2], options);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                // Load template
                let template = fs::read_to_string("out/template.html")?; // Change the path to the template file

                // Replace placeholders in template
                let html_output = template
                    .replace("{{title}}", &front_matter.title)
                    .replace("{{slug}}", &front_matter.slug)
                    .replace("{{description}}", front_matter.description.as_deref().unwrap_or(""))
                    .replace("{{keywords}}", front_matter.keywords.as_deref().unwrap_or(""))
                    .replace("{{author}}", front_matter.author.as_deref().unwrap_or(""))
                    .replace("{{content}}", &html_output);

                // Write the static HTML file
                fs::write(&output_file, html_output)?;
            }
        }
    }

    Ok(())
}

fn copy_static_assets() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new("out");
    let assets_dir = Path::new("static");

    if !output_dir.exists() || !output_dir.is_dir() {
        fs::create_dir_all(&output_dir)?;
    }

    if assets_dir.exists() && assets_dir.is_dir() {
        let entries = fs::read_dir(&assets_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let output_path = output_dir.join(file_name);

                fs::copy(path, output_path)?;
            }
        }
    }

    Ok(())
}
