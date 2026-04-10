use super::{LintRule, LintWarning, Severity};
use crate::i18n::{Lang, Messages};
use crate::rules::heading::heading_level;

pub struct BlankLineRule {
    pub max_consecutive: usize,
    pub lang: Lang,
    /// If true, only check blank lines around headings (light mode).
    /// If false, also check consecutive blank lines (deep mode).
    pub headings_only: bool,
}

impl Default for BlankLineRule {
    fn default() -> Self {
        Self {
            max_consecutive: 1,
            lang: Lang::En,
            headings_only: false,
        }
    }
}

impl LintRule for BlankLineRule {
    fn name(&self) -> &str {
        "blank-line"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut consecutive_blanks = 0;
        let mut in_code_block = false;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
            }
            if in_code_block {
                consecutive_blanks = 0;
                continue;
            }

            if trimmed.is_empty() {
                consecutive_blanks += 1;
                if !self.headings_only
                    && self.max_consecutive > 0
                    && consecutive_blanks > self.max_consecutive
                {
                    warnings.push(LintWarning {
                        line: i + 1,
                        column: 1,
                        message: Messages::too_many_blank_lines(
                            self.lang,
                            consecutive_blanks,
                            self.max_consecutive,
                        ),
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        fixable: true,
                    });
                }
            } else {
                consecutive_blanks = 0;
            }

            if heading_level(trimmed).is_some() {
                if i > 0 && !lines[i - 1].trim().is_empty() {
                    warnings.push(LintWarning {
                        line: i + 1,
                        column: 1,
                        message: Messages::missing_blank_before_heading(self.lang).to_string(),
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        fixable: true,
                    });
                }
                if i + 1 < lines.len()
                    && !lines[i + 1].trim().is_empty()
                    && heading_level(lines[i + 1].trim()).is_none()
                {
                    warnings.push(LintWarning {
                        line: i + 1,
                        column: 1,
                        message: Messages::missing_blank_after_heading(self.lang).to_string(),
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        fixable: true,
                    });
                }
            }
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut consecutive_blanks = 0;
        let mut in_code_block = false;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
            }

            if in_code_block {
                result.push(line.to_string());
                consecutive_blanks = 0;
                continue;
            }

            if trimmed.is_empty() {
                consecutive_blanks += 1;
                if consecutive_blanks <= self.max_consecutive {
                    result.push(line.to_string());
                }
            } else {
                if heading_level(trimmed).is_some() && i > 0 {
                    if let Some(prev) = result.last() {
                        if !prev.trim().is_empty() {
                            result.push(String::new());
                        }
                    }
                }

                result.push(line.to_string());

                if heading_level(trimmed).is_some() {
                    if let Some(next) = lines.get(i + 1) {
                        if !next.trim().is_empty() && heading_level(next.trim()).is_none() {
                            result.push(String::new());
                        }
                    }
                }

                consecutive_blanks = 0;
            }
        }

        let mut output = result.join("\n");
        if content.ends_with('\n') && !output.ends_with('\n') {
            output.push('\n');
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_blank_lines() {
        let content = "Line 1\n\n\n\nLine 2\n";
        let rule = BlankLineRule::default();
        let warnings = rule.check(content);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_consecutive_blank_lines_ko() {
        let content = "Line 1\n\n\n\nLine 2\n";
        let rule = BlankLineRule {
            max_consecutive: 1,
            lang: Lang::Ko,
            headings_only: false,
        };
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.message.contains("연속 빈 줄")));
    }

    #[test]
    fn test_missing_blank_before_heading() {
        let content = "Some text\n## Heading\n";
        let rule = BlankLineRule::default();
        let warnings = rule.check(content);
        assert!(warnings
            .iter()
            .any(|w| w.message.contains("before heading")));
    }

    #[test]
    fn test_missing_blank_after_heading() {
        let content = "## Heading\nSome text\n";
        let rule = BlankLineRule::default();
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.message.contains("after heading")));
    }

    #[test]
    fn test_fix_consecutive_blanks() {
        let content = "Line 1\n\n\n\nLine 2\n";
        let rule = BlankLineRule::default();
        let fixed = rule.fix(content);
        assert_eq!(fixed, "Line 1\n\nLine 2\n");
    }

    #[test]
    fn test_fix_blank_before_heading() {
        let content = "Some text\n## Heading\n";
        let rule = BlankLineRule::default();
        let fixed = rule.fix(content);
        assert!(fixed.contains("Some text\n\n## Heading"));
    }

    #[test]
    fn test_valid_blank_lines() {
        let content = "# Title\n\nParagraph\n\n## Next\n\nMore text\n";
        let rule = BlankLineRule::default();
        let warnings = rule.check(content);
        let heading_warnings: Vec<_> = warnings
            .iter()
            .filter(|w| w.message.contains("heading") || w.message.contains("blank line"))
            .collect();
        assert!(heading_warnings.is_empty());
    }

    #[test]
    fn test_code_block_blanks_ignored() {
        let content = "# Title\n\n```\n\n\n\n```\n";
        let rule = BlankLineRule::default();
        let warnings = rule.check(content);
        let blank_warnings: Vec<_> = warnings
            .iter()
            .filter(|w| w.message.contains("consecutive") || w.message.contains("연속"))
            .collect();
        assert!(blank_warnings.is_empty());
    }
}
