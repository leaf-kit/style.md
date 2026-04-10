use super::{LintRule, LintWarning, Severity};
use crate::i18n::{Lang, Messages};

pub struct HeadingStructureRule {
    pub lang: Lang,
}

impl Default for HeadingStructureRule {
    fn default() -> Self {
        Self { lang: Lang::En }
    }
}

impl LintRule for HeadingStructureRule {
    fn name(&self) -> &str {
        "heading-structure"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
        let mut h1_count = 0;
        let mut last_level: Option<usize> = None;
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

            if let Some(level) = heading_level(trimmed) {
                if level == 1 {
                    h1_count += 1;
                    if h1_count > 1 {
                        warnings.push(LintWarning {
                            line: i + 1,
                            column: 1,
                            message: Messages::multiple_h1(self.lang).to_string(),
                            rule: self.name().to_string(),
                            severity: Severity::Warning,
                            fixable: false,
                        });
                    }
                }

                if let Some(prev) = last_level {
                    if level > prev + 1 {
                        warnings.push(LintWarning {
                            line: i + 1,
                            column: 1,
                            message: Messages::heading_level_skip(self.lang, level, prev),
                            rule: self.name().to_string(),
                            severity: Severity::Error,
                            fixable: false,
                        });
                    }
                }
                last_level = Some(level);
            }
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        content.to_string()
    }
}

pub fn heading_level(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }
    let hashes = trimmed.chars().take_while(|&c| c == '#').count();
    if hashes > 6 {
        return None;
    }
    if trimmed.len() == hashes || trimmed.chars().nth(hashes) == Some(' ') {
        Some(hashes)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_level() {
        assert_eq!(heading_level("# Title"), Some(1));
        assert_eq!(heading_level("## Subtitle"), Some(2));
        assert_eq!(heading_level("### Third"), Some(3));
        assert_eq!(heading_level("###### Sixth"), Some(6));
        assert_eq!(heading_level("####### Seven"), None);
        assert_eq!(heading_level("Not a heading"), None);
        assert_eq!(heading_level("#NoSpace"), None);
    }

    #[test]
    fn test_duplicate_h1() {
        let content = "# First\n\nSome text\n\n# Second\n";
        let rule = HeadingStructureRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].line, 5);
    }

    #[test]
    fn test_duplicate_h1_ko() {
        let content = "# First\n\nSome text\n\n# Second\n";
        let rule = HeadingStructureRule { lang: Lang::Ko };
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("H1"));
        assert!(warnings[0].message.contains("여러 개"));
    }

    #[test]
    fn test_heading_level_skip() {
        let content = "# Title\n\n### Skipped H2\n";
        let rule = HeadingStructureRule::default();
        let warnings = rule.check(content);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_heading_level_skip_ko() {
        let content = "# Title\n\n### Skipped H2\n";
        let rule = HeadingStructureRule { lang: Lang::Ko };
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.message.contains("건너뜀")));
    }

    #[test]
    fn test_valid_headings() {
        let content = "# Title\n\n## Section\n\n### Subsection\n";
        let rule = HeadingStructureRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_headings_in_code_block_ignored() {
        let content = "# Title\n\n```\n# Not a heading\n### Also not\n```\n\n## Real\n";
        let rule = HeadingStructureRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }
}
