use std::path::Path;

use url::{ParseError, Url};

use crate::{constants::{SCALE_MAX, SCALE_MIN}, error::{Error, ErrorKind}};

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

pub fn percentage(cur: f64, total: f64) -> String {
    format!("{}%", (cur / total * 100.0).round())
}

pub fn _page_info(cur: i32, total: i32) -> String {
    format!("{}/{}", cur, total)
}

#[derive(Debug)]
pub enum PageFitKind {
    Page(f64),
    Width(f64),
    None,
}

pub fn check_page_fit(page_size_original: (f64, f64), outer_size: (f64, f64), factor: f64) -> PageFitKind {
    let (pw0, ph0) = page_size_original;
    let (w, h) = outer_size;

    let pw = pw0 * factor;
    let ph = ph0 * factor;

    if ph0 < h && ph >= h {
        PageFitKind::Page(h / ph0)
    } else if pw0 < w && pw >= w {
        PageFitKind::Width(w / pw0)
    } else {
        PageFitKind::None
    }
}

pub fn format_scale_status(mut factor: f64, page_fit: PageFitKind) -> String {
    static SCALE_MAX_PER: i32 = (SCALE_MAX * 100.0) as i32;
    static SCALE_MIN_PER: i32 = (SCALE_MIN * 100.0) as i32;

    let mut status_list = Vec::<&'static str>::new();

    let mut rounded_factor = (factor * 100.0).round() as i32;

    if rounded_factor >= SCALE_MAX_PER {
        status_list.push("max");
    } else if rounded_factor <= SCALE_MIN_PER {
        status_list.push("min");
    }

    match page_fit {
        PageFitKind::Width(f) => {
            status_list.push("fit width");
            factor = f;
        }
        PageFitKind::Page(f) => {
            status_list.push("fit page");
            factor = f;
        }
        _ => {}
    }
    
    rounded_factor = (factor * 100.0).round() as i32;

    let suffix = if status_list.is_empty() {
        "".to_string()
    } else {
        format!(" ({})", status_list.join(","))
    };

    if rounded_factor == 100 {
        "".to_string()
    } else {
        format!(
            "Zoom: {}%{}",
            rounded_factor,
            suffix,
        )
    }
}
