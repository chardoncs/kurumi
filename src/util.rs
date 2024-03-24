use std::path::Path;

use url::{ParseError, Url};

use crate::error::{Error, ErrorKind};

/// Convert the possible path/URL/URI string to Servo's URL
pub fn convert_to_url(path_str: &str) -> Result<Url, Error> {
    // Try parsing the raw string as URL at first
    Ok(match Url::parse(path_str) {
        // It works!
        Ok(url) => url,
        // Nah...
        Err(err) => {
            if let ParseError::RelativeUrlWithoutBase = err {
                // Add local file base (file://) to the absolute path
                let patched_path = format!(
                    "file://{}",
                    Path::new(path_str)
                        .canonicalize()
                        .or_else(|err| Err(Error::new(ErrorKind::File, err.to_string().as_str())))?
                        .to_str()
                        .ok_or_else(|| Error::new(ErrorKind::File, "Path string invalid"))?
                );

                // Try again
                Url::parse(patched_path.as_str())
                    .or_else(|err| Err(Error::new(ErrorKind::UrlParsing, err.to_string().as_str())))?
            } else {
                // No idea how to solve it...
                Err(Error::new(ErrorKind::UrlParsing, err.to_string().as_str()))?
            }
        }
    })
}

/// Add suffix to the input title string
pub fn patch_title(title: Option<&str>) -> String {
    let app_name = clap::crate_name!();

    match title {
        Some(title) => format!("{} \u{2014} {}", title, app_name),
        None => app_name.to_string(),
    }
}

pub fn pos_percentage(cur: f64, total: f64) -> String {
    format!("{}%", (cur / total * 100.0).round())
}

pub fn _page_info(cur: i32, total: i32) -> String {
    format!("{}/{}", cur, total)
}
