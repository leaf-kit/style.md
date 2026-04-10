use super::{LintRule, LintWarning, Severity};
use crate::i18n::{Lang, Messages};

pub struct TrailingWhitespaceRule {
    pub lang: Lang,
}

impl Default for TrailingWhitespaceRule {
    fn default() -> Self {
        Self { lang: Lang::En }
    }
}

impl LintRule for TrailingWhitespaceRule {
    fn name(&self) -> &str {
        "trailing-whitespace"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
        let mut in_code_block = false;

        for (i, line) in content.lines().enumerate() {
            if line.trim().starts_with("```") {
                in_code_block = !in_code_block;
            }
            if in_code_block {
                continue;
            }

            if line != line.trim_end() {
                let trailing = line.len() - line.trim_end().len();
                let has_tabs =
                    line.ends_with('\t') || line.chars().rev().take(trailing).any(|c| c == '\t');

                let msg = if has_tabs {
                    Messages::trailing_whitespace_with_tabs(self.lang, trailing)
                } else {
                    Messages::trailing_whitespace(self.lang, trailing)
                };

                warnings.push(LintWarning {
                    line: i + 1,
                    column: line.trim_end().len() + 1,
                    message: msg,
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    fixable: true,
                });
            }
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        let mut in_code_block = false;
        let lines: Vec<String> = content
            .lines()
            .map(|line| {
                if line.trim().starts_with("```") {
                    in_code_block = !in_code_block;
                }
                if in_code_block {
                    line.to_string()
                } else {
                    line.trim_end().to_string()
                }
            })
            .collect();

        let mut output = lines.join("\n");
        if content.ends_with('\n') {
            output.push('\n');
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_trailing_spaces() {
        let content = "Hello   \nWorld\n";
        let rule = TrailingWhitespaceRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].line, 1);
        assert!(warnings[0].fixable);
    }

    #[test]
    fn test_detect_trailing_spaces_ko() {
        let content = "Hello   \nWorld\n";
        let rule = TrailingWhitespaceRule { lang: Lang::Ko };
        let warnings = rule.check(content);
        assert!(warnings[0].message.contains("후행 공백"));
    }

    #[test]
    fn test_detect_trailing_tabs() {
        let content = "Hello\t\nWorld\n";
        let rule = TrailingWhitespaceRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("tabs") || warnings[0].message.contains("탭"));
    }

    #[test]
    fn test_no_trailing_whitespace() {
        let content = "Hello\nWorld\n";
        let rule = TrailingWhitespaceRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_fix_trailing_whitespace() {
        let content = "Hello   \nWorld  \t\nGood\n";
        let rule = TrailingWhitespaceRule::default();
        let fixed = rule.fix(content);
        assert_eq!(fixed, "Hello\nWorld\nGood\n");
    }

    #[test]
    fn test_code_block_ignored() {
        let content = "```\nHello   \n```\n";
        let rule = TrailingWhitespaceRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_fix_preserves_code_block() {
        let content = "```\nHello   \n```\nTrailing   \n";
        let rule = TrailingWhitespaceRule::default();
        let fixed = rule.fix(content);
        assert!(fixed.contains("Hello   "));
        assert!(fixed.contains("\nTrailing\n"));
    }

    #[test]
    fn test_multiple_trailing_lines() {
        let content = "A  \nB\nC \nD\t\n";
        let rule = TrailingWhitespaceRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 3);
    }
}
