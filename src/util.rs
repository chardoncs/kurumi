use poppler::Document;

use std::path::Path;

use gtk::ScrolledWindow;
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

/// Refresh record
pub struct RefreshRecord {
    pub cur_page: usize,
    pub scroll_percentage: f32,
}

/// Implementation of dynamic loading of PDF pages within the display
pub fn refresh_dynamic_pages<'a>(sw: &'a ScrolledWindow, box_: &'a gtk::Box, doc: &'a Document, active_pages: &'a mut Vec<usize>) -> RefreshRecord {
    todo!();
}
