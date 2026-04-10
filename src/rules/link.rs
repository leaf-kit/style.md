use super::{LintRule, LintWarning, Severity};
use crate::i18n::{Lang, Messages};
use regex::Regex;
use std::path::Path;

pub struct LinkRule {
    pub base_path: Option<String>,
    pub check_external: bool,
    pub lang: Lang,
}

impl Default for LinkRule {
    fn default() -> Self {
        Self {
            base_path: None,
            check_external: false,
            lang: Lang::En,
        }
    }
}

#[derive(Debug)]
pub struct LinkInfo {
    pub line: usize,
    pub column: usize,
    #[allow(dead_code)]
    pub text: String,
    pub url: String,
    pub link_type: LinkType,
}

#[derive(Debug, PartialEq)]
pub enum LinkType {
    External,
    Anchor,
    Relative,
}

impl LintRule for LinkRule {
    fn name(&self) -> &str {
        "broken-link"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
        let links = extract_links(content);
        let headings = extract_headings(content);

        for link in &links {
            match link.link_type {
                LinkType::Anchor => {
                    let anchor = link.url.trim_start_matches('#');
                    let slug = heading_to_slug(anchor);
                    if !headings.iter().any(|h| heading_to_slug(h) == slug) {
                        warnings.push(LintWarning {
                            line: link.line,
                            column: link.column,
                            message: Messages::broken_anchor(self.lang, &link.url),
                            rule: self.name().to_string(),
                            severity: Severity::Error,
                            fixable: false,
                        });
                    }
                }
                LinkType::Relative => {
                    if let Some(base) = &self.base_path {
                        let target = Path::new(base)
                            .parent()
                            .unwrap_or(Path::new("."))
                            .join(&link.url);
                        let path_without_anchor = target
                            .to_str()
                            .unwrap_or("")
                            .split('#')
                            .next()
                            .unwrap_or("");
                        if !Path::new(path_without_anchor).exists() {
                            warnings.push(LintWarning {
                                line: link.line,
                                column: link.column,
                                message: Messages::broken_relative(self.lang, &link.url),
                                rule: self.name().to_string(),
                                severity: Severity::Error,
                                fixable: false,
                            });
                        }
                    }
                }
                LinkType::External => {
                    if self.check_external {
                        match check_url(&link.url) {
                            Ok(status) if status >= 400 => {
                                warnings.push(LintWarning {
                                    line: link.line,
                                    column: link.column,
                                    message: Messages::broken_external(
                                        self.lang, &link.url, status,
                                    ),
                                    rule: self.name().to_string(),
                                    severity: Severity::Error,
                                    fixable: false,
                                });
                            }
                            Err(e) => {
                                warnings.push(LintWarning {
                                    line: link.line,
                                    column: link.column,
                                    message: Messages::unreachable_link(self.lang, &link.url, &e),
                                    rule: self.name().to_string(),
                                    severity: Severity::Warning,
                                    fixable: false,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        content.to_string()
    }
}

pub fn extract_links(content: &str) -> Vec<LinkInfo> {
    let mut links = Vec::new();
    let re = Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap();
    let mut in_code_block = false;

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        for cap in re.captures_iter(line) {
            let text = cap[1].to_string();
            let url = cap[2].to_string();
            let column = cap.get(0).map(|m| m.start() + 1).unwrap_or(1);

            let link_type = if url.starts_with("http://") || url.starts_with("https://") {
                LinkType::External
            } else if url.starts_with('#') {
                LinkType::Anchor
            } else {
                LinkType::Relative
            };

            links.push(LinkInfo {
                line: i + 1,
                column,
                text,
                url,
                link_type,
            });
        }
    }

    links
}

pub fn extract_headings(content: &str) -> Vec<String> {
    let mut headings = Vec::new();
    let mut in_code_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        if trimmed.starts_with('#') {
            let heading_text = trimmed.trim_start_matches('#').trim();
            if !heading_text.is_empty() {
                headings.push(heading_text.to_string());
            }
        }
    }

    headings
}

pub fn heading_to_slug(heading: &str) -> String {
    heading
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c == ' ' {
                '-'
            } else {
                ' '
            }
        })
        .filter(|c| *c != ' ')
        .collect()
}

fn check_url(url: &str) -> Result<u16, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.head(url).send().map_err(|e| e.to_string())?;
    Ok(resp.status().as_u16())
}

pub fn check_links_in_file(
    path: &Path,
    content: &str,
    check_external: bool,
    lang: Lang,
) -> Vec<LintWarning> {
    let rule = LinkRule {
        base_path: path.to_str().map(String::from),
        check_external,
        lang,
    };
    rule.check(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let content = "Check [Google](https://google.com) and [anchor](#section)\n";
        let links = extract_links(content);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].link_type, LinkType::External);
        assert_eq!(links[1].link_type, LinkType::Anchor);
    }

    #[test]
    fn test_extract_relative_links() {
        let content = "[readme](./README.md)\n";
        let links = extract_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link_type, LinkType::Relative);
    }

    #[test]
    fn test_heading_to_slug() {
        assert_eq!(heading_to_slug("Hello World"), "hello-world");
        assert_eq!(heading_to_slug("API Design!"), "api-design");
        assert_eq!(heading_to_slug("H1: Title"), "h1-title");
    }

    #[test]
    fn test_broken_anchor() {
        let content = "# Title\n\n[link](#nonexistent)\n";
        let rule = LinkRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("Broken anchor"));
    }

    #[test]
    fn test_valid_anchor() {
        let content = "# My Section\n\n[link](#my-section)\n";
        let rule = LinkRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_extract_headings() {
        let content = "# H1\n## H2\n### H3\n";
        let headings = extract_headings(content);
        assert_eq!(headings, vec!["H1", "H2", "H3"]);
    }

    #[test]
    fn test_links_in_code_block_ignored() {
        let content = "```\n[link](http://example.com)\n```\n";
        let links = extract_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_multiple_links_same_line() {
        let content = "[a](http://a.com) and [b](http://b.com)\n";
        let links = extract_links(content);
        assert_eq!(links.len(), 2);
    }
}
