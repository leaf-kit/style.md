use super::{LintRule, LintWarning, Severity};
use crate::i18n::{Lang, Messages};

pub struct CodeBlockRule {
    pub lang: Lang,
    /// If false (light mode), only check unclosed blocks. If true (deep), also check language.
    pub check_language: bool,
}

impl Default for CodeBlockRule {
    fn default() -> Self {
        Self {
            lang: Lang::En,
            check_language: true,
        }
    }
}

impl LintRule for CodeBlockRule {
    fn name(&self) -> &str {
        "code-block"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
        let mut in_code_block = false;
        let mut code_block_start: usize = 0;

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                if !in_code_block {
                    in_code_block = true;
                    code_block_start = i + 1;

                    let lang_spec = trimmed.trim_start_matches('`').trim();
                    if self.check_language && lang_spec.is_empty() {
                        warnings.push(LintWarning {
                            line: i + 1,
                            column: 1,
                            message: Messages::code_block_missing_lang(self.lang).to_string(),
                            rule: self.name().to_string(),
                            severity: Severity::Warning,
                            fixable: false,
                        });
                    }
                } else {
                    in_code_block = false;
                }
            }
        }

        if in_code_block {
            warnings.push(LintWarning {
                line: code_block_start,
                column: 1,
                message: Messages::unclosed_code_block(self.lang).to_string(),
                rule: self.name().to_string(),
                severity: Severity::Error,
                fixable: true,
            });
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        let mut in_code_block = false;
        let lines: Vec<&str> = content.lines().collect();
        let mut result: Vec<String> = Vec::new();

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
            }
            result.push(line.to_string());
        }

        if in_code_block {
            result.push("```".to_string());
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
    fn test_missing_language() {
        let content = "```\ncode here\n```\n";
        let rule = CodeBlockRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_missing_language_ko() {
        let content = "```\ncode here\n```\n";
        let rule = CodeBlockRule {
            lang: Lang::Ko,
            check_language: true,
        };
        let warnings = rule.check(content);
        assert!(warnings[0].message.contains("언어 지정 누락"));
    }

    #[test]
    fn test_with_language() {
        let content = "```rust\nlet x = 1;\n```\n";
        let rule = CodeBlockRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_unclosed_code_block() {
        let content = "```rust\nlet x = 1;\nno closing\n";
        let rule = CodeBlockRule::default();
        let warnings = rule.check(content);
        assert!(warnings.iter().any(|w| w.severity == Severity::Error));
    }

    #[test]
    fn test_fix_unclosed_code_block() {
        let content = "```rust\nlet x = 1;\n";
        let rule = CodeBlockRule::default();
        let fixed = rule.fix(content);
        assert!(fixed.ends_with("```\n"));
    }

    #[test]
    fn test_multiple_code_blocks() {
        let content = "```rust\ncode\n```\n\n```python\ncode\n```\n";
        let rule = CodeBlockRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_nested_backticks_in_code() {
        let content = "```markdown\n## heading\n```\n";
        let rule = CodeBlockRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }
}
