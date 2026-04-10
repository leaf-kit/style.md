use crate::config::Config;
use crate::i18n::Lang;
use crate::rules::blank_line::BlankLineRule;
use crate::rules::code_block::CodeBlockRule;
use crate::rules::heading::HeadingStructureRule;
use crate::rules::image::ImageRule;
use crate::rules::link::LinkRule;
use crate::rules::list::ListRule;
use crate::rules::trailing_whitespace::TrailingWhitespaceRule;
use crate::rules::{LintRule, LintWarning};
use std::path::Path;

pub struct Linter {
    rules: Vec<Box<dyn LintRule>>,
}

impl Linter {
    /// Create a light-mode linter (default): fast, essential checks only.
    pub fn light(config: &Config, file_path: Option<&Path>, lang: Lang) -> Self {
        let mut rules: Vec<Box<dyn LintRule>> = Vec::new();

        if config.light.heading_structure {
            rules.push(Box::new(HeadingStructureRule { lang }));
        }
        if config.light.trailing_whitespace {
            rules.push(Box::new(TrailingWhitespaceRule { lang }));
        }
        if config.light.blank_lines_around_headings {
            rules.push(Box::new(BlankLineRule {
                max_consecutive: 0, // light: only check around headings, no consecutive limit
                lang,
                headings_only: true,
            }));
        }
        if config.light.unclosed_code_block {
            rules.push(Box::new(CodeBlockRule {
                lang,
                check_language: false,
            }));
        }
        if config.light.list_marker_consistency {
            rules.push(Box::new(ListRule {
                preferred_marker: config.light.list_marker.clone(),
                lang,
            }));
        }
        if config.light.image_file_exists {
            rules.push(Box::new(ImageRule {
                base_path: file_path.and_then(|p| p.to_str()).map(String::from),
                lang,
            }));
        }

        Self { rules }
    }

    /// Create a deep-mode linter: all light checks + thorough analysis.
    pub fn deep(config: &Config, file_path: Option<&Path>, lang: Lang) -> Self {
        let mut rules: Vec<Box<dyn LintRule>> = Vec::new();

        // All light rules
        if config.light.heading_structure {
            rules.push(Box::new(HeadingStructureRule { lang }));
        }
        if config.light.trailing_whitespace {
            rules.push(Box::new(TrailingWhitespaceRule { lang }));
        }
        if config.light.list_marker_consistency {
            rules.push(Box::new(ListRule {
                preferred_marker: config.light.list_marker.clone(),
                lang,
            }));
        }
        if config.light.image_file_exists {
            rules.push(Box::new(ImageRule {
                base_path: file_path.and_then(|p| p.to_str()).map(String::from),
                lang,
            }));
        }

        // Deep-only: blank lines (full: headings + consecutive)
        if config.light.blank_lines_around_headings || config.deep.consecutive_blank_lines {
            rules.push(Box::new(BlankLineRule {
                max_consecutive: config.deep.max_consecutive_blank_lines,
                lang,
                headings_only: false,
            }));
        }

        // Deep-only: code block with language check
        if config.light.unclosed_code_block || config.deep.code_block_language {
            rules.push(Box::new(CodeBlockRule {
                lang,
                check_language: config.deep.code_block_language,
            }));
        }

        // Deep-only: link validation
        let check_external = config.deep.external_links;
        if config.deep.anchor_links || config.deep.relative_links || check_external {
            rules.push(Box::new(LinkRule {
                base_path: file_path.and_then(|p| p.to_str()).map(String::from),
                check_external,
                lang,
            }));
        }

        Self { rules }
    }

    pub fn check(&self, content: &str) -> Vec<LintWarning> {
        let mut all_warnings: Vec<LintWarning> = Vec::new();

        for rule in &self.rules {
            all_warnings.extend(rule.check(content));
        }

        all_warnings.sort_by_key(|w| (w.line, w.column));
        all_warnings
    }

    pub fn fix(&self, content: &str) -> String {
        let mut result = content.to_string();
        for rule in &self.rules {
            result = rule.fix(&result);
        }
        result
    }

    #[allow(dead_code)]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

pub struct LintResult {
    pub file: String,
    pub warnings: Vec<LintWarning>,
    pub fixable_count: usize,
}

impl LintResult {
    pub fn new(file: String, warnings: Vec<LintWarning>) -> Self {
        let fixable_count = warnings.iter().filter(|w| w.fixable).count();
        Self {
            file,
            warnings,
            fixable_count,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.warnings
            .iter()
            .any(|w| w.severity == crate::rules::Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.warnings
            .iter()
            .filter(|w| w.severity == crate::rules::Severity::Error)
            .count()
    }

    #[allow(dead_code)]
    pub fn warning_count(&self) -> usize {
        self.warnings
            .iter()
            .filter(|w| w.severity == crate::rules::Severity::Warning)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_light() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::En);
        assert!(linter.rule_count() >= 5);
    }

    #[test]
    fn test_linter_deep() {
        let config = Config::default();
        let linter = Linter::deep(&config, None, Lang::En);
        assert!(linter.rule_count() > Linter::light(&config, None, Lang::En).rule_count());
    }

    #[test]
    fn test_light_detects_trailing_whitespace() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::En);
        let content = "# Title\n\nSome text   \n";
        let warnings = linter.check(content);
        assert!(warnings.iter().any(|w| w.rule == "trailing-whitespace"));
    }

    #[test]
    fn test_light_no_code_block_language_check() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::En);
        let content = "# Title\n\n```\ncode\n```\n";
        let warnings = linter.check(content);
        assert!(!warnings.iter().any(|w| w.message.contains("language")));
    }

    #[test]
    fn test_deep_checks_code_block_language() {
        let config = Config::default();
        let linter = Linter::deep(&config, None, Lang::En);
        let content = "# Title\n\n```\ncode\n```\n";
        let warnings = linter.check(content);
        assert!(warnings
            .iter()
            .any(|w| w.message.contains("language") || w.message.contains("언어")));
    }

    #[test]
    fn test_light_no_link_check() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::En);
        let content = "# Title\n\n[broken](#nope)\n";
        let warnings = linter.check(content);
        assert!(!warnings.iter().any(|w| w.rule == "broken-link"));
    }

    #[test]
    fn test_deep_checks_links() {
        let config = Config::default();
        let linter = Linter::deep(&config, None, Lang::En);
        let content = "# Title\n\n[broken](#nope)\n";
        let warnings = linter.check(content);
        assert!(warnings.iter().any(|w| w.rule == "broken-link"));
    }

    #[test]
    fn test_linter_ko() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::Ko);
        let content = "# Title\nSome text   \n";
        let warnings = linter.check(content);
        assert!(warnings
            .iter()
            .any(|w| w.message.contains("후행 공백") || w.message.contains("빈 줄")));
    }

    #[test]
    fn test_linter_fix() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::En);
        let content = "# Title\nSome text   \n\n\n\nMore text\n";
        let fixed = linter.fix(content);
        assert!(!fixed.contains("   \n"));
    }

    #[test]
    fn test_lint_result() {
        let warnings = vec![
            LintWarning {
                line: 1,
                column: 1,
                message: "test".to_string(),
                rule: "test".to_string(),
                severity: crate::rules::Severity::Error,
                fixable: true,
            },
            LintWarning {
                line: 2,
                column: 1,
                message: "test".to_string(),
                rule: "test".to_string(),
                severity: crate::rules::Severity::Warning,
                fixable: false,
            },
        ];
        let result = LintResult::new("test.md".to_string(), warnings);
        assert!(result.has_errors());
        assert_eq!(result.error_count(), 1);
        assert_eq!(result.warning_count(), 1);
        assert_eq!(result.fixable_count, 1);
    }

    #[test]
    fn test_warnings_sorted_by_line() {
        let config = Config::default();
        let linter = Linter::light(&config, None, Lang::En);
        let content = "text   \n# H1\nmore   \n# H1 again\n";
        let warnings = linter.check(content);
        for i in 1..warnings.len() {
            assert!(warnings[i].line >= warnings[i - 1].line);
        }
    }
}
