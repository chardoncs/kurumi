use std::path::Path;

use url::{ParseError, Url};

use crate::error::{Error, ErrorKind};

pub fn convert_to_url(path_str: &str) -> Result<Url, Error> {
    // Try parsing the raw string as URL
    Ok(match Url::parse(path_str) {
        // It works!
        Ok(url) => url,
        // Nah...
        Err(err) => {
            if let ParseError::RelativeUrlWithoutBase = err {
                // Add base to the absolute path
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
                Err(Error::new(ErrorKind::UrlParsing, err.to_string().as_str()))?
            }
        }
    })
}

pub fn patch_title(title: Option<&str>) -> String {
    let app_name = clap::crate_name!();

    match title {
        Some(title) => format!("{} \u{2014} {}", title, app_name),
        None => app_name.to_string(),
    }
}

