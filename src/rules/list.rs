use super::{LintRule, LintWarning, Severity};
use crate::i18n::{Lang, Messages};

pub struct ListRule {
    pub preferred_marker: String,
    pub lang: Lang,
}

impl Default for ListRule {
    fn default() -> Self {
        Self {
            preferred_marker: "-".to_string(),
            lang: Lang::En,
        }
    }
}

impl LintRule for ListRule {
    fn name(&self) -> &str {
        "list-style"
    }

    fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut warnings = Vec::new();
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

            if let Some(marker) = get_list_marker(trimmed) {
                if marker != self.preferred_marker {
                    warnings.push(LintWarning {
                        line: i + 1,
                        column: 1,
                        message: Messages::list_marker_inconsistent(
                            self.lang,
                            marker,
                            &self.preferred_marker,
                        ),
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        fixable: true,
                    });
                }

                let indent = line.len() - line.trim_start().len();
                if indent > 0 && indent % 2 != 0 && indent % 4 != 0 {
                    warnings.push(LintWarning {
                        line: i + 1,
                        column: 1,
                        message: Messages::list_bad_indentation(self.lang, indent),
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        fixable: false,
                    });
                }
            }
        }

        warnings
    }

    fn fix(&self, content: &str) -> String {
        let mut in_code_block = false;
        let lines: Vec<String> = content
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                    return line.to_string();
                }
                if in_code_block {
                    return line.to_string();
                }

                if let Some(marker) = get_list_marker(trimmed) {
                    if marker != self.preferred_marker {
                        let indent = line.len() - line.trim_start().len();
                        let rest = trimmed[marker.len()..].to_string();
                        return format!("{}{}{}", " ".repeat(indent), self.preferred_marker, rest);
                    }
                }

                line.to_string()
            })
            .collect();

        let mut output = lines.join("\n");
        if content.ends_with('\n') && !output.ends_with('\n') {
            output.push('\n');
        }
        output
    }
}

fn get_list_marker(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("- ") {
        Some("-")
    } else if trimmed.starts_with("* ") {
        Some("*")
    } else if trimmed.starts_with("+ ") {
        Some("+")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_markers() {
        let content = "- Item 1\n- Item 2\n- Item 3\n";
        let rule = ListRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_mixed_markers() {
        let content = "- Item 1\n* Item 2\n+ Item 3\n";
        let rule = ListRule::default();
        let warnings = rule.check(content);
        assert_eq!(warnings.len(), 2);
    }

    #[test]
    fn test_mixed_markers_ko() {
        let content = "- Item 1\n* Item 2\n";
        let rule = ListRule {
            preferred_marker: "-".to_string(),
            lang: Lang::Ko,
        };
        let warnings = rule.check(content);
        assert!(warnings[0].message.contains("목록 기호"));
    }

    #[test]
    fn test_fix_markers() {
        let content = "* Item 1\n+ Item 2\n- Item 3\n";
        let rule = ListRule::default();
        let fixed = rule.fix(content);
        assert_eq!(fixed, "- Item 1\n- Item 2\n- Item 3\n");
    }

    #[test]
    fn test_indented_list() {
        let content = "- Item\n  - Sub item\n    - Sub sub item\n";
        let rule = ListRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_odd_indentation() {
        let content = "- Item\n   - Bad indent\n";
        let rule = ListRule::default();
        let warnings = rule.check(content);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_code_block_lists_ignored() {
        let content = "```\n* not a list\n+ also not\n```\n";
        let rule = ListRule::default();
        let warnings = rule.check(content);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_fix_preserves_indentation() {
        let content = "* Item\n  * Sub item\n";
        let rule = ListRule::default();
        let fixed = rule.fix(content);
        assert_eq!(fixed, "- Item\n  - Sub item\n");
    }

    #[test]
    fn test_get_list_marker() {
        assert_eq!(get_list_marker("- item"), Some("-"));
        assert_eq!(get_list_marker("* item"), Some("*"));
        assert_eq!(get_list_marker("+ item"), Some("+"));
        assert_eq!(get_list_marker("1. item"), None);
        assert_eq!(get_list_marker("text"), None);
    }
}
