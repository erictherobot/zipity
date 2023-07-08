// src/robots_handler.rs

use actix_web::{ HttpResponse };
use std::io::Cursor;
use std::io::Write;

pub async fn robots() -> Result<HttpResponse, actix_web::Error> {
    let mut buffer = Cursor::new(Vec::<u8>::new());

    writeln!(buffer, "User-Agent: *")?;
    writeln!(buffer, "Allow: /")?;
    writeln!(buffer, "Sitemap: http://127.0.0.1:8080/sitemap.xml")?;

    // Add additional rules to allow or disallow specific paths
    writeln!(buffer, "Disallow: /path/to/disallowed-page")?;

    Ok(
        HttpResponse::Ok()
            .content_type("text/plain")
            .body(String::from_utf8(buffer.into_inner()).unwrap())
    )
}
