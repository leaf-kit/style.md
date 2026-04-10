use super::{LintRule, LintWarning, Severity};
use crate::i18n::Lang;
use regex::Regex;
use std::path::Path;

pub struct ImageRule {
    pub base_path: Option<String>,
    pub lang: Lang,
}

impl Default for ImageRule {
    fn default() -> Self {
        Self {
            base_path: None,
            lang: Lang::En,
        }
    }
}

#[derive(Debug)]
pub struct ImageRef {
    pub line: usize,
    pub column: usize,
    pub alt: String,
    pub path: String,
}

impl LintRule for ImageRule {
    fn name(&self) -> &str {
        "image-ref"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
        let images = extract_images(content);

        for img in &images {
            if img.path.starts_with("http://") || img.path.starts_with("https://") {
                continue;
            }

            if let Some(base) = &self.base_path {
                let base_dir = Path::new(base).parent().unwrap_or(Path::new("."));
                let target = base_dir.join(&img.path);
                if !target.exists() {
                    let msg = match self.lang {
                        Lang::Ko => format!("이미지 파일을 찾을 수 없음: {}", img.path),
                        Lang::En => format!("Image file not found: {}", img.path),
                    };
                    warnings.push(LintWarning {
                        line: img.line,
                        column: img.column,
                        message: msg,
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        fixable: false,
                    });
                }
            }

            if img.alt.is_empty() {
                let msg = match self.lang {
                    Lang::Ko => format!("이미지에 alt 텍스트 누락: {}", img.path),
                    Lang::En => format!("Image missing alt text: {}", img.path),
                };
                warnings.push(LintWarning {
                    line: img.line,
                    column: img.column,
                    message: msg,
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    fixable: false,
                });
            }
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        content.to_string()
    }
}

pub fn extract_images(content: &str) -> Vec<ImageRef> {
    let mut images = Vec::new();
    let re = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();
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
            let alt = cap[1].to_string();
            let path = cap[2].trim().to_string();
            let column = cap.get(0).map(|m| m.start() + 1).unwrap_or(1);

            images.push(ImageRef {
                line: i + 1,
                column,
                alt,
                path,
            });
        }
    }

    // Also check HTML <img> tags
    let img_re = Regex::new(r#"<img\s[^>]*src\s*=\s*"([^"]+)"[^>]*/?\s*>"#).unwrap();
    let alt_re = Regex::new(r#"alt\s*=\s*"([^"]*)""#).unwrap();
    let mut in_code_block2 = false;

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block2 = !in_code_block2;
            continue;
        }
        if in_code_block2 {
            continue;
        }

        for cap in img_re.captures_iter(line) {
            let path = cap[1].to_string();
            let alt = alt_re
                .captures(cap.get(0).unwrap().as_str())
                .map(|a| a[1].to_string())
                .unwrap_or_default();
            let column = cap.get(0).map(|m| m.start() + 1).unwrap_or(1);

            // Skip if already captured by markdown syntax
            if !images
                .iter()
                .any(|img| img.line == i + 1 && img.path == path)
            {
                images.push(ImageRef {
                    line: i + 1,
                    column,
                    alt,
                    path,
                });
            }
        }
    }

    images
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_markdown_images() {
        let content = "# Doc\n\n![logo](images/logo.png)\n\nSome text\n";
        let images = extract_images(content);
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].alt, "logo");
        assert_eq!(images[0].path, "images/logo.png");
    }

    #[test]
    fn test_extract_html_images() {
        let content = "# Doc\n\n<img src=\"images/logo.png\" alt=\"logo\" />\n";
        let images = extract_images(content);
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].path, "images/logo.png");
        assert_eq!(images[0].alt, "logo");
    }

    #[test]
    fn test_missing_alt_text() {
        let content = "![](images/logo.png)\n";
        let rule = ImageRule::default();
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.message.contains("alt text")));
    }

    #[test]
    fn test_missing_alt_text_ko() {
        let content = "![](images/logo.png)\n";
        let rule = ImageRule {
            base_path: None,
            lang: Lang::Ko,
        };
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.message.contains("alt 텍스트")));
    }

    #[test]
    fn test_missing_image_file() {
        let content = "![logo](nonexistent/image.png)\n";
        let rule = ImageRule {
            base_path: Some("/tmp/test.md".to_string()),
            lang: Lang::En,
        };
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.message.contains("not found")));
    }

    #[test]
    fn test_external_image_skipped() {
        let content = "![logo](https://example.com/logo.png)\n";
        let rule = ImageRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_images_in_code_block_ignored() {
        let content = "```\n![logo](images/logo.png)\n```\n";
        let images = extract_images(content);
        assert!(images.is_empty());
    }

    #[test]
    fn test_multiple_images() {
        let content = "![a](a.png)\n![b](b.png)\n![c](c.png)\n";
        let images = extract_images(content);
        assert_eq!(images.len(), 3);
    }
}
