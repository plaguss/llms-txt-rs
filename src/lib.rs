use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref LINK_REGEX: Regex =
        Regex::new(r"-\s*\[(?P<title>[^\]]+)\]\((?P<url>[^\)]+)\)(?::\s*(?P<desc>.*))?").unwrap();
    static ref SECTION_REGEX: Regex = Regex::new(r"(?m)^##\s*(.*?)\n").unwrap();
    static ref START_REGEX: Regex =
        Regex::new(r"(?ms)^#\s*(?P<title>.+?$)\n+(?:^>\s*(?P<summary>.+?$)\n+)?(?P<info>.*)")
            .unwrap();
}

fn parse_links(links: &str) -> Vec<HashMap<String, String>> {
    links
        .trim()
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            LINK_REGEX.captures(line).map(|caps| {
                let mut link = HashMap::new();
                link.insert(
                    "title".to_string(),
                    caps.name("title").unwrap().as_str().to_string(),
                );
                link.insert(
                    "url".to_string(),
                    caps.name("url").unwrap().as_str().to_string(),
                );
                if let Some(desc) = caps.name("desc") {
                    link.insert("desc".to_string(), desc.as_str().trim().to_string());
                } else {
                    link.insert("desc".to_string(), "".to_string());
                }
                link
            })
        })
        .collect()
}

fn split_text(txt: &str) -> (&str, Vec<&str>) {
    // Find the first match to determine the start of the sections
    let mut iter = SECTION_REGEX.splitn(txt, 2);
    let start = iter.next().unwrap_or("");

    // Collect the headers and their contents in order
    let mut rest = Vec::new();
    let mut last_end = 0;

    for mat in SECTION_REGEX.find_iter(txt) {
        // Extract the content between the previous match and the current match
        if last_end > 0 {
            let content = txt[last_end..mat.start()].trim();
            if !content.is_empty() {
                rest.push(content);
            }
        }

        // Extract the header without "## " and newline and add it
        let header = &txt[mat.start() + 3..mat.end() - 1];
        rest.push(header);

        last_end = mat.end();
    }

    // Add the content after the last section header
    if last_end > 0 {
        let content = txt[last_end..].trim();
        if !content.is_empty() {
            rest.push(content);
        }
    }

    (start, rest)
}

#[pyfunction]
pub fn parse_llms_txt(py: Python<'_>, txt: &str) -> PyResult<PyObject> {
    let (start, rest) = split_text(txt);

    let sections = PyDict::new(py);
    for chunk in rest.chunks(2) {
        if chunk.len() == 2 {
            let section_name = chunk[0].trim();
            let links = parse_links(chunk[1]);
            // Convert the links to a Python-compatible format.
            // This is done so we can return None in python if the description is empty.
            // Not sure it's really worth, but just to make it equivalent to the Python version.
            // Returning py.None from parse_links makes it hard to read and test afterwards.
            let py_links = PyList::new(
                py,
                links.into_iter().map(|link| {
                    let py_dict = PyDict::new(py);
                    py_dict
                        .set_item("title", link.get("title").unwrap())
                        .unwrap();
                    py_dict.set_item("url", link.get("url").unwrap()).unwrap();
                    let desc = link.get("desc").unwrap();
                    let _ = py_dict.set_item(
                        "desc",
                        if desc.is_empty() {
                            py.None()
                        } else {
                            desc.into_py(py)
                        },
                    );
                    py_dict
                }),
            );
            sections.set_item(section_name, py_links.unwrap())?;
        }
    }

    let start_caps = START_REGEX.captures(start.trim()).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to parse header section")
    })?;

    let result = PyDict::new(py);
    result.set_item(
        "title",
        start_caps
            .name("title")
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing title"))?
            .as_str()
            .trim(),
    )?;

    result.set_item(
        "summary",
        match start_caps.name("summary") {
            Some(summary) => summary.as_str().trim().into_py(py),
            None => py.None(),
        },
    )?;

    result.set_item(
        "info",
        start_caps
            .name("info")
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing info"))?
            .as_str()
            .trim(),
    )?;

    result.set_item("sections", sections)?;
    Ok(result.into())
}

#[pymodule]
fn llms_txt_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_llms_txt, m)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_links_basic() {
        let input = "- [Example Title](https://example.com)";
        let result = parse_links(input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["title"], "Example Title");
        assert_eq!(result[0]["url"], "https://example.com");
        assert_eq!(result[0]["desc"], "");
    }

    #[test]
    fn test_parse_links_with_description() {
        let input = "- [Example Title](https://example.com): A sample description";
        let result = parse_links(input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["title"], "Example Title");
        assert_eq!(result[0]["url"], "https://example.com");
        assert_eq!(result[0]["desc"], "A sample description");
    }

    #[test]
    fn test_parse_links_multiple() {
        let input = r#"- [llms.txt proposal](https://llmstxt.org/index.md): The proposal for llms.txt
- [Python library docs](https://llmstxt.org/intro.html.md): Docs for `llms-txt` python lib
- [ed demo](https://llmstxt.org/ed-commonmark.md): Tongue-in-cheek example of how llms.txt could be used in the classic `ed` editor, used to show how editors could incorporate llms.txt in general."#;
        let result = parse_links(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0]["title"], "llms.txt proposal");
        assert_eq!(result[0]["url"], "https://llmstxt.org/index.md");
        assert_eq!(result[0]["desc"], "The proposal for llms.txt");

        assert_eq!(result[1]["title"], "Python library docs");
        assert_eq!(result[1]["url"], "https://llmstxt.org/intro.html.md");
        assert_eq!(result[1]["desc"], "Docs for `llms-txt` python lib");

        assert_eq!(result[2]["title"], "ed demo");
        assert_eq!(result[2]["url"], "https://llmstxt.org/ed-commonmark.md");
        assert_eq!(result[2]["desc"], "Tongue-in-cheek example of how llms.txt could be used in the classic `ed` editor, used to show how editors could incorporate llms.txt in general.");
    }

    #[test]
    fn test_parse_links_empty_input() {
        let input = "";
        let result = parse_links(input);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_links_whitespace_input() {
        let input = "   \n  ";
        let result = parse_links(input);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_split_text() {
        let txt = r#"# Title

> Optional description goes here

Optional details go here

## Section name

- [Link title](https://link_url): Optional link details

## Optional

- [Link title](https://link_url)
"#;

        let (start, rest) = split_text(txt);
        assert_eq!(
            start,
            "# Title\n\n> Optional description goes here\n\nOptional details go here\n\n"
        );
        assert_eq!(
            rest,
            vec![
                "Section name",
                "- [Link title](https://link_url): Optional link details",
                "Optional",
                "- [Link title](https://link_url)"
            ]
        );
    }

    #[test]
    fn test_split_text_no_sections() {
        let txt = r#"# Title

> Optional description goes here

Optional details go here
"#;

        let (start, rest) = split_text(txt);
        assert_eq!(
            start,
            "# Title\n\n> Optional description goes here\n\nOptional details go here\n"
        );
        assert_eq!(rest, Vec::<&str>::new());
    }

    #[test]
    fn test_split_text_single_section() {
        let txt = r#"# Title

## Section name

- [Link title](https://link_url): Optional link details
"#;

        let (start, rest) = split_text(txt);
        assert_eq!(start, "# Title\n\n");
        assert_eq!(
            rest,
            vec![
                "Section name",
                "- [Link title](https://link_url): Optional link details"
            ]
        );
    }

    #[test]
    fn test_split_text_multiple_sections() {
        let txt = r#"# Title

## Section 1

- [Link title](https://link_url): Optional link details

## Section 2

- [Link title](https://link_url)

## Section 3

- [Link title](https://link_url)
"#;

        let (start, rest) = split_text(txt);
        assert_eq!(start, "# Title\n\n");
        assert_eq!(
            rest,
            vec![
                "Section 1",
                "- [Link title](https://link_url): Optional link details",
                "Section 2",
                "- [Link title](https://link_url)",
                "Section 3",
                "- [Link title](https://link_url)"
            ]
        );
    }

    #[test]
    fn test_split_text_empty_input() {
        let txt = "";

        let (start, rest) = split_text(txt);
        assert_eq!(start, "");
        assert_eq!(rest, Vec::<&str>::new());
    }
}
