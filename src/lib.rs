//! # gemini2html
//! Simple library for parsing Gemini (gmi) to HTML format
//! Example usage:
//! ```
//! use gemini2html::parse_to_html;
//!
//! let header = "# Title\n ## Description";
//! let html: Vec<String> = parse_to_html(header).unwrap();
//!
//! assert_eq!(html, vec!["<h1>Title</h1>", "<h2>Description</h2>"])
//! ```

use std::error::Error;
use std::fmt;
use std::fmt::Write as _;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
struct ParseGmiErr;

impl fmt::Display for ParseGmiErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed parsing Gemini to HTML format!")
    }
}

impl Error for ParseGmiErr {}

/// Parse gemini (gmi) to html format
pub fn parse_to_html(gmi: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();

    let (mut cb_bool, mut cb_text) = (false, String::new());
    let (mut list_bool, mut list_text) = (false, String::new());

    'main: for line in gmi.lines() {
        let line = line.trim_start();

        if let Some(strip) = line.strip_prefix("* ") {
            write!(list_text, "<li>{strip}</li>")?;
            list_bool = true;
            continue
        }

        if list_bool {
            result.push(format!("<ul>{list_text}</ul>"));
            list_bool = false;
            list_text.clear();
        }

        if line == "```" {
            result.push(cb_text.clone() + "</code>");
            cb_text.clear();
            cb_bool = false;
            continue;
        }

        if cb_bool {
            writeln!(cb_text, "{line}")?;
            continue;
        }

        if let Some(strip) = line.strip_prefix("# ") {
            result.push(format!("<h1>{strip}</h1>"));
            continue;
        }

        if let Some(strip) = line.strip_prefix("## ") {
            result.push(format!("<h2>{strip}</h2>"));
            continue;
        }

        if let Some(strip) = line.strip_prefix("### ") {
            result.push(format!("<h3>{strip}</h3>"));
            continue;
        }

        if let Some(strip) = line.strip_prefix("=> ") {
            let url = strip.split_whitespace().next().ok_or(ParseGmiErr)?;
            let text = strip.strip_prefix(url).ok_or(ParseGmiErr)?.trim_start();
            let image_formats = [".jpeg", ".png", ".jpg", ".webp", ".gif", ".svg"];
            for format in image_formats {
                if url.ends_with(format) {
                    result.push(format!("<img src=\"{url}\">{text}</img>"));
                    continue 'main
                }
            }
            result.push(format!("<a href=\"{url}\">{text}</a>"));
            continue;
        }

        if let Some(strip) = line.strip_prefix("> ") {
            result.push(format!("<blockquote><p>{strip}</p></blockquote>"));
            continue;
        }

        if let Some(strip) = line.strip_prefix("```") {
            writeln!(cb_text, "<code> {strip}")?;
            cb_bool = true;
            continue;
        }

        result.push(format!("<p>{line}</p>"));
    }
    Ok(result)
}
