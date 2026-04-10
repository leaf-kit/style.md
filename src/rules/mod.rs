pub mod blank_line;
pub mod code_block;
pub mod heading;
pub mod image;
pub mod link;
pub mod list;
pub mod trailing_whitespace;

use crate::i18n::Lang;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    #[allow(dead_code)]
    Info,
}

impl Severity {
    pub fn display(&self, lang: Lang) -> &'static str {
        use crate::i18n::Messages;
        match self {
            Severity::Error => Messages::error(lang),
            Severity::Warning => Messages::warning(lang),
            Severity::Info => Messages::info(lang),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LintWarning {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub rule: String,
    pub severity: Severity,
    pub fixable: bool,
}

impl fmt::Display for LintWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: [{}] {} ({})",
            self.line, self.column, self.severity, self.message, self.rule
        )
    }
}

pub trait LintRule {
    fn name(&self) -> &str;
    fn check(&self, content: &str) -> Vec<LintWarning>;
    fn fix(&self, content: &str) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_display() {
        assert_eq!(format!("{}", Severity::Error), "error");
        assert_eq!(format!("{}", Severity::Warning), "warning");
        assert_eq!(format!("{}", Severity::Info), "info");
    }

    #[test]
    fn test_severity_display_ko() {
        assert_eq!(Severity::Error.display(Lang::Ko), "오류");
        assert_eq!(Severity::Warning.display(Lang::Ko), "경고");
    }

    #[test]
    fn test_lint_warning_display() {
        let w = LintWarning {
            line: 5,
            column: 1,
            message: "test message".to_string(),
            rule: "test-rule".to_string(),
            severity: Severity::Warning,
            fixable: true,
        };
        assert_eq!(format!("{}", w), "5:1: [warning] test message (test-rule)");
    }
}
