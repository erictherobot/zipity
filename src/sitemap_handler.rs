// src/sitemap_handler.rs

use actix_web::{ HttpResponse };
use sitemap::structs::{ ChangeFreq, UrlEntry };
use sitemap::writer::SiteMapWriter;
use url::Url;
use std::fs;
use std::io::Cursor;

pub async fn sitemap() -> Result<HttpResponse, actix_web::Error> {
    let mut buffer = Cursor::new(Vec::<u8>::new());

    let sitemap_writer = SiteMapWriter::new(&mut buffer);

    let entries = fs::read_dir("./routes").map_err(|e| {
        eprintln!("Failed to read directory: {:?}", e);
        actix_web::Error::from(e)
    })?;

    let mut urlwriter = sitemap_writer.start_urlset().map_err(|e| {
        eprintln!("Failed to start urlset: {:?}", e);
        actix_web::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "md" {
                let path_str = path.to_str().unwrap().replace("./routes", "").replace(".md", "");
                let url = Url::parse(&format!("http://127.0.0.1:8080{}", path_str)).unwrap();
                let entry = UrlEntry::builder()
                    .loc(url.as_str())
                    .changefreq(ChangeFreq::Weekly)
                    .build()
                    .map_err(|e| {
                        eprintln!("Failed to create UrlEntry: {:?}", e);
                        actix_web::Error::from(
                            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                        )
                    })?;
                urlwriter.url(entry).map_err(|e| {
                    eprintln!("Failed to write URL entry: {:?}", e);
                    actix_web::Error::from(
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    )
                })?;
            }
        }
    }

    urlwriter.end().map_err(|e| {
        eprintln!("Failed to end urlset: {:?}", e);
        actix_web::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;

    Ok(
        HttpResponse::Ok()
            .content_type("application/xml")
            .body(String::from_utf8(buffer.into_inner()).unwrap())
    )
}
