use clap::{ App, SubCommand, Arg };
use std::fs::{ self, File };
use std::io::Write;
use std::path::Path;
use chrono::offset::Local;
use chrono::Datelike;
use slug::slugify;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let app = App::new("Zipity")
        .version("0.0.1-alpha.2")
        .author("Eric David Smith")
        .about("A command-line tool for Zipity")
        .subcommand(
            SubCommand::with_name("init")
                .about("Creates a new Zipity project")
                .arg(Arg::with_name("project_name").required(true))
        )
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("serve"))
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a new component to the project")
                .subcommand(
                    SubCommand::with_name("route")
                        .about("Adds a new route with a markdown file")
                        .arg(Arg::with_name("route_name").required(true))
                )
                .subcommand(
                    SubCommand::with_name("api")
                        .about("Adds a new API endpoint")
                        .arg(Arg::with_name("endpoint_name").required(true))
                )
                .subcommand(
                    SubCommand::with_name("test")
                        .about("Adds a new test")
                        .arg(Arg::with_name("test_name").required(true))
                )
                .subcommand(
                    SubCommand::with_name("md")
                        .about("Adds a new markdown file")
                        .arg(Arg::with_name("file_name").required(true))
                )
                .subcommand(
                    SubCommand::with_name("static")
                        .about("Adds a new static file")
                        .arg(Arg::with_name("file_name").required(true))
                )
        )
        .get_matches();

    match app.subcommand() {
        ("init", Some(init_matches)) => {
            let project_name = init_matches.value_of("project_name").unwrap();
            println!("Creating new Zipity project: {}", project_name);

            // Create the project directory
            fs::create_dir(project_name).expect("Failed to create project directory");

            // Create the README.md file inside the project directory
            let readme_path = Path::new(project_name).join("README.md");
            let mut readme_file = File::create(&readme_path).expect(
                "Failed to create README.md file"
            );
            readme_file
                .write_all(
                    b"# My Zipity Project\n\nWelcome to my Zipity project!\n\n## Usage\n\n```bash\n./cli/zipity <SUBCOMMAND> [OPTIONS]\n```\n\n## Help\n\n```text\nZipity 0.0.1-alpha.2\nEric David Smith\nA command-line tool for Zipity\n\nUSAGE:\n    cli [SUBCOMMAND]\n\nFLAGS:\n    -h, --help       Prints help information\n    -V, --version    Prints version information\n\nSUBCOMMANDS:\n    add      Adds a new component to the project\n    build\n    help     Prints this message or the help of the given subcommand(s)\n    init\n    serve\n```"
                )
                .expect("Failed to write to README.md");

            // Copy static assets to the project directory
            let static_dir = Path::new("static");
            let destination_dir = Path::new(project_name).join("static");
            copy_directory(static_dir, destination_dir).expect("Failed to copy static assets");

            // Create the .gitignore file
            let gitignore_path = Path::new(project_name).join(".gitignore");
            let mut gitignore_file = File::create(&gitignore_path).expect(
                "Failed to create .gitignore file"
            );
            gitignore_file.write_all(b"/target\n/out\n").expect("Failed to write to .gitignore");

            // Copy the template.html file to the project directory
            let template_path = Path::new("template.html");
            let destination_path = Path::new(project_name).join("template.html");
            fs::copy(template_path, destination_path).expect("Failed to copy template.html");

            // Run 'git init' in the project directory
            Command::new("git")
                .arg("init")
                .arg(project_name)
                .output()
                .expect("Failed to run 'git init'");

            println!("New Zipity project created: {}", project_name);

            // Create the routes directory
            let routes_dir_path = Path::new(project_name).join("routes");
            fs::create_dir(&routes_dir_path).expect("Failed to create routes directory");

            // Create the index.md file in the routes directory
            let current_date = Local::now();
            let formatted_date = format!(
                "{}-{:02}-{:02}",
                current_date.year(),
                current_date.month(),
                current_date.day()
            );
            let route_slug = slugify("Index");
            let index_md_path = routes_dir_path.join(format!("{}.md", route_slug));
            let mut index_md_file = File::create(&index_md_path).expect(
                "Failed to create index.md file"
            );
            let index_md_content = format!(
                r#"---
title: "Index"
slug: "{}"
date: "{}"
description: "This is a description of my post"
keywords: "keyword1, keyword2, keyword3"
author: "Author Name"
---

# Index

This is a placeholder for the Index route.
"#,
                route_slug,
                formatted_date
            );
            index_md_file
                .write_all(index_md_content.as_bytes())
                .expect("Failed to write to index.md file");

            println!("Created index.md file at {}", index_md_path.to_string_lossy());
        }
        ("build", Some(_)) => {
            println!("You ran 'build' command");
        }
        ("serve", Some(_)) => {
            println!("You ran 'serve' command");
            let host = "127.0.0.1";
            let port = 8080;

            println!("Server is running on http://{}:{}", host, port);

            let output = Command::new("cargo")
                .args(&["run", "--bin", "zipity"])
                .output()
                .expect("Failed to start server");

            if !output.status.success() {
                eprintln!("Error: {:?}", output);
            }
        }
        ("add", Some(add_matches)) => {
            match add_matches.subcommand() {
                ("route", Some(matches)) => {
                    let route_name = matches.value_of("route_name").unwrap();
                    println!("You ran 'add route' command with route name '{}'", route_name);

                    let current_date = Local::now();
                    let formatted_date = format!(
                        "{}-{:02}-{:02}",
                        current_date.year(),
                        current_date.month(),
                        current_date.day()
                    );
                    let route_slug = slugify(route_name);

                    let route_file_path = Path::new("routes").join(format!("{}.md", route_slug));
                    let mut route_file = File::create(&route_file_path).expect(
                        "Failed to create route file"
                    );

                    let template = format!(
                        r#"---
title: "{}"
slug: "{}"
date: "{}"
description: "This is a description of my post"
keywords: "keyword1, keyword2, keyword3"
author: "Author Name"
---

# {}

This is a placeholder for the {} route.
"#,
                        route_name,
                        route_slug,
                        formatted_date,
                        route_name,
                        route_name
                    );
                    route_file
                        .write_all(template.as_bytes())
                        .expect("Failed to write to route file");

                    println!("Created route file at {}", route_file_path.to_string_lossy());
                }
                ("api", Some(matches)) => {
                    let api_name = matches.value_of("endpoint_name").unwrap();
                    println!("You ran 'add api' command with API name '{}'", api_name);

                    let api_slug = slugify(api_name);

                    // Create the 'api' directory if it doesn't exist
                    fs::create_dir_all("api").expect("Failed to create 'api' directory");

                    let api_file_path = Path::new("api").join(format!("{}.rs", api_slug));
                    let mut api_file = File::create(&api_file_path).expect(
                        "Failed to create API file"
                    );

                    let template = format!(
                        r#"use actix_web::{{web, HttpResponse, Responder}};

pub async fn {}() -> impl Responder {{
    HttpResponse::Ok().body("This is the {} API endpoint")
}}
"#,
                        api_slug,
                        api_name
                    );
                    api_file.write_all(template.as_bytes()).expect("Failed to write to API file");

                    println!("Created API file at {}", api_file_path.to_string_lossy());
                }
                ("test", Some(matches)) => {
                    let test_name = matches.value_of("test_name").unwrap();
                    println!("You ran 'add test' command with test name '{}'", test_name);
                    // Here is where you would add the logic to actually create the new test
                }
                ("md", Some(matches)) => {
                    let file_name = matches.value_of("file_name").unwrap();
                    println!("You ran 'add md' command with file name '{}'", file_name);
                    // Here is where you would add the logic to actually create the new markdown file
                }
                ("static", Some(matches)) => {
                    let file_name = matches.value_of("file_name").unwrap();
                    println!("You ran 'add static' command with file name '{}'", file_name);
                    // Here is where you would add the logic to actually create the new static file
                }
                _ => {
                    println!("Unknown 'add' subcommand");
                }
            }
        }
        _ => {
            println!("Unknown command");
        }
    }
}

// Helper function to copy a directory recursively
fn copy_directory(source: &Path, destination: PathBuf) -> std::io::Result<()> {
    if source.is_dir() {
        fs::create_dir_all(&destination)?;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = destination.join(entry.file_name());

            if entry_path.is_dir() {
                copy_directory(&entry_path, dest_path)?;
            } else {
                fs::copy(&entry_path, &dest_path)?;
            }
        }
    }
    Ok(())
}
