use crate::i18n::{Lang, Messages};
use crate::linter::LintResult;
use crate::rules::{LintWarning, Severity};
use colored::Colorize;
use similar::{ChangeTag, TextDiff};

pub struct Formatter {
    pub use_color: bool,
    pub lang: Lang,
}

impl Formatter {
    pub fn new(use_color: bool, lang: Lang) -> Self {
        Self { use_color, lang }
    }

    pub fn format_warning(&self, file: &str, warning: &LintWarning) -> String {
        if self.use_color {
            let severity_str = match warning.severity {
                Severity::Error => warning.severity.display(self.lang).red().bold().to_string(),
                Severity::Warning => warning
                    .severity
                    .display(self.lang)
                    .yellow()
                    .bold()
                    .to_string(),
                Severity::Info => warning
                    .severity
                    .display(self.lang)
                    .cyan()
                    .bold()
                    .to_string(),
            };
            let fix_badge = if warning.fixable {
                format!(" [{}]", Messages::fixable_tag(self.lang))
                    .green()
                    .to_string()
            } else {
                String::new()
            };
            format!(
                "  {}:{}:{}: {} {}{}",
                file.bold(),
                warning.line.to_string().cyan(),
                warning.column.to_string().cyan(),
                severity_str,
                warning.message,
                fix_badge
            )
        } else {
            let fix_badge = if warning.fixable {
                format!(" [{}]", Messages::fixable_tag(self.lang))
            } else {
                String::new()
            };
            format!(
                "  {}:{}:{}: {} {}{}",
                file,
                warning.line,
                warning.column,
                warning.severity.display(self.lang),
                warning.message,
                fix_badge
            )
        }
    }

    pub fn format_result(&self, result: &LintResult) -> String {
        let mut output = Vec::new();

        if result.warnings.is_empty() {
            return String::new();
        }

        if self.use_color {
            output.push(format!("\n{}", result.file.bold().underline()));
        } else {
            output.push(format!("\n{}", result.file));
        }

        for warning in &result.warnings {
            output.push(self.format_warning(&result.file, warning));
        }

        output.join("\n")
    }

    pub fn format_summary(&self, results: &[LintResult]) -> String {
        let total_warnings: usize = results.iter().map(|r| r.warnings.len()).sum();
        let total_errors: usize = results.iter().map(|r| r.error_count()).sum();
        let total_fixable: usize = results.iter().map(|r| r.fixable_count).sum();
        let files_with_issues = results.iter().filter(|r| !r.warnings.is_empty()).count();
        let total_files = results.len();

        let mut parts = Vec::new();

        if self.use_color {
            parts.push(format!(
                "\n{} {} {}, {} {}",
                Messages::summary(self.lang).bold(),
                total_files,
                Messages::files_checked(self.lang),
                if files_with_issues > 0 {
                    files_with_issues.to_string().yellow().to_string()
                } else {
                    "0".green().to_string()
                },
                Messages::with_issues(self.lang)
            ));
            if total_errors > 0 {
                parts.push(format!(
                    "  {} {}, {} {}",
                    total_errors.to_string().red().bold(),
                    Messages::errors(self.lang),
                    (total_warnings - total_errors).to_string().yellow(),
                    Messages::warnings(self.lang)
                ));
            } else if total_warnings > 0 {
                parts.push(format!(
                    "  {} {}",
                    total_warnings.to_string().yellow(),
                    Messages::warnings(self.lang)
                ));
            }
            if total_fixable > 0 {
                parts.push(format!(
                    "  {} {} ({})",
                    total_fixable.to_string().green(),
                    Messages::auto_fixable(self.lang),
                    Messages::use_fix(self.lang)
                ));
            }
            if total_warnings == 0 {
                parts.push(format!(
                    "  {}",
                    Messages::all_clean(self.lang).green().bold()
                ));
            }
        } else {
            parts.push(format!(
                "\n{} {} {}, {} {}",
                Messages::summary(self.lang),
                total_files,
                Messages::files_checked(self.lang),
                files_with_issues,
                Messages::with_issues(self.lang)
            ));
            if total_warnings > 0 {
                parts.push(format!(
                    "  {} {}, {} {}",
                    total_errors,
                    Messages::errors(self.lang),
                    total_warnings - total_errors,
                    Messages::warnings(self.lang)
                ));
            }
            if total_fixable > 0 {
                parts.push(format!(
                    "  {} {} ({})",
                    total_fixable,
                    Messages::auto_fixable(self.lang),
                    Messages::use_fix(self.lang)
                ));
            }
            if total_warnings == 0 {
                parts.push(format!("  {}", Messages::all_clean(self.lang)));
            }
        }

        parts.join("\n")
    }

    pub fn format_diff(&self, original: &str, fixed: &str, file: &str) -> String {
        let diff = TextDiff::from_lines(original, fixed);
        let mut output = Vec::new();

        if self.use_color {
            output.push(format!("\n--- {}", file.red()));
            output.push(format!("+++ {}", file.green()));
        } else {
            output.push(format!("\n--- {}", file));
            output.push(format!("+++ {}", file));
        }

        for change in diff.iter_all_changes() {
            let line = change.to_string_lossy();
            let line = line.trim_end_matches('\n');
            match change.tag() {
                ChangeTag::Delete => {
                    if self.use_color {
                        output.push(format!("{}", format!("-{}", line).red()));
                    } else {
                        output.push(format!("-{}", line));
                    }
                }
                ChangeTag::Insert => {
                    if self.use_color {
                        output.push(format!("{}", format!("+{}", line).green()));
                    } else {
                        output.push(format!("+{}", line));
                    }
                }
                ChangeTag::Equal => {
                    output.push(format!(" {}", line));
                }
            }
        }

        output.join("\n")
    }

    pub fn format_json(&self, results: &[LintResult]) -> String {
        let mut entries = Vec::new();
        for result in results {
            for warning in &result.warnings {
                entries.push(serde_json::json!({
                    "file": result.file,
                    "line": warning.line,
                    "column": warning.column,
                    "severity": format!("{}", warning.severity),
                    "message": warning.message,
                    "rule": warning.rule,
                    "fixable": warning.fixable,
                }));
            }
        }
        serde_json::to_string_pretty(&entries).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_yaml(&self, results: &[LintResult]) -> String {
        let mut entries = Vec::new();
        for result in results {
            for warning in &result.warnings {
                entries.push(serde_json::json!({
                    "file": result.file,
                    "line": warning.line,
                    "column": warning.column,
                    "severity": format!("{}", warning.severity),
                    "message": warning.message,
                    "rule": warning.rule,
                    "fixable": warning.fixable,
                }));
            }
        }
        serde_yaml::to_string(&entries).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_stats(&self, results: &[LintResult]) -> String {
        let total_files = results.len();
        let total_warnings: usize = results.iter().map(|r| r.warnings.len()).sum();
        let total_errors: usize = results.iter().map(|r| r.error_count()).sum();
        let total_fixable: usize = results.iter().map(|r| r.fixable_count).sum();
        let clean_files = results.iter().filter(|r| r.warnings.is_empty()).count();

        let score = if total_files == 0 {
            100.0
        } else {
            let clean_ratio = clean_files as f64 / total_files as f64;
            (clean_ratio * 100.0).round()
        };

        let mut rule_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for result in results {
            for warning in &result.warnings {
                *rule_counts.entry(warning.rule.clone()).or_insert(0) += 1;
            }
        }

        let mut output = Vec::new();
        if self.use_color {
            output.push(format!(
                "\n{}",
                Messages::quality_report(self.lang).bold().underline()
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::files_scanned(self.lang),
                total_files
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::clean_files(self.lang),
                clean_files.to_string().green()
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::total_issues(self.lang),
                total_warnings
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::errors_label(self.lang),
                if total_errors > 0 {
                    total_errors.to_string().red().to_string()
                } else {
                    "0".green().to_string()
                }
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::warnings_label(self.lang),
                total_warnings - total_errors
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::auto_fixable_label(self.lang),
                total_fixable.to_string().green()
            ));

            let score_str = if score >= 90.0 {
                format!("{:.0}%", score).green().bold().to_string()
            } else if score >= 70.0 {
                format!("{:.0}%", score).yellow().bold().to_string()
            } else {
                format!("{:.0}%", score).red().bold().to_string()
            };
            output.push(format!(
                "  {:20}{}",
                Messages::quality_score(self.lang),
                score_str
            ));

            if !rule_counts.is_empty() {
                output.push(format!(
                    "\n  {}",
                    Messages::issues_by_rule(self.lang).bold()
                ));
                let mut sorted: Vec<_> = rule_counts.iter().collect();
                sorted.sort_by(|a, b| b.1.cmp(a.1));
                for (rule, count) in sorted {
                    output.push(format!("    {:30} {}", rule.yellow(), count));
                }
            }
        } else {
            output.push(format!("\n{}", Messages::quality_report(self.lang)));
            output.push(format!(
                "  {:20}{}",
                Messages::files_scanned(self.lang),
                total_files
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::clean_files(self.lang),
                clean_files
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::total_issues(self.lang),
                total_warnings
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::errors_label(self.lang),
                total_errors
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::warnings_label(self.lang),
                total_warnings - total_errors
            ));
            output.push(format!(
                "  {:20}{}",
                Messages::auto_fixable_label(self.lang),
                total_fixable
            ));
            output.push(format!(
                "  {:20}{:.0}%",
                Messages::quality_score(self.lang),
                score
            ));

            if !rule_counts.is_empty() {
                output.push(format!("\n  {}", Messages::issues_by_rule(self.lang)));
                let mut sorted: Vec<_> = rule_counts.iter().collect();
                sorted.sort_by(|a, b| b.1.cmp(a.1));
                for (rule, count) in sorted {
                    output.push(format!("    {:30} {}", rule, count));
                }
            }
        }

        output.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_warning() -> LintWarning {
        LintWarning {
            line: 5,
            column: 10,
            message: "test warning".to_string(),
            rule: "test-rule".to_string(),
            severity: Severity::Warning,
            fixable: true,
        }
    }

    #[test]
    fn test_format_warning_no_color() {
        let fmt = Formatter::new(false, Lang::En);
        let output = fmt.format_warning("test.md", &sample_warning());
        assert!(output.contains("test.md"));
        assert!(output.contains("5"));
        assert!(output.contains("test warning"));
        assert!(output.contains("[fixable]"));
    }

    #[test]
    fn test_format_warning_ko() {
        let fmt = Formatter::new(false, Lang::Ko);
        let output = fmt.format_warning("test.md", &sample_warning());
        assert!(output.contains("경고"));
        assert!(output.contains("[수정가능]"));
    }

    #[test]
    fn test_format_result_empty() {
        let fmt = Formatter::new(false, Lang::En);
        let result = LintResult::new("test.md".to_string(), vec![]);
        let output = fmt.format_result(&result);
        assert!(output.is_empty());
    }

    #[test]
    fn test_format_summary_clean() {
        let fmt = Formatter::new(false, Lang::En);
        let results = vec![LintResult::new("test.md".to_string(), vec![])];
        let output = fmt.format_summary(&results);
        assert!(output.contains("All files are clean!"));
    }

    #[test]
    fn test_format_summary_clean_ko() {
        let fmt = Formatter::new(false, Lang::Ko);
        let results = vec![LintResult::new("test.md".to_string(), vec![])];
        let output = fmt.format_summary(&results);
        assert!(output.contains("깨끗합니다"));
    }

    #[test]
    fn test_format_diff() {
        let fmt = Formatter::new(false, Lang::En);
        let original = "Hello   \nWorld\n";
        let fixed = "Hello\nWorld\n";
        let output = fmt.format_diff(original, fixed, "test.md");
        assert!(output.contains("-Hello"));
        assert!(output.contains("+Hello"));
    }

    #[test]
    fn test_format_json() {
        let fmt = Formatter::new(false, Lang::En);
        let results = vec![LintResult::new(
            "test.md".to_string(),
            vec![sample_warning()],
        )];
        let json = fmt.format_json(&results);
        assert!(json.contains("test-rule"));
        assert!(json.contains("test.md"));
    }

    #[test]
    fn test_format_yaml() {
        let fmt = Formatter::new(false, Lang::En);
        let results = vec![LintResult::new(
            "test.md".to_string(),
            vec![sample_warning()],
        )];
        let yaml = fmt.format_yaml(&results);
        assert!(yaml.contains("test-rule"));
        assert!(yaml.contains("test.md"));
    }

    #[test]
    fn test_format_yaml_empty() {
        let fmt = Formatter::new(false, Lang::En);
        let results = vec![LintResult::new("clean.md".to_string(), vec![])];
        let yaml = fmt.format_yaml(&results);
        assert!(yaml.contains("[]"));
    }

    #[test]
    fn test_format_stats() {
        let fmt = Formatter::new(false, Lang::En);
        let results = vec![
            LintResult::new("a.md".to_string(), vec![sample_warning()]),
            LintResult::new("b.md".to_string(), vec![]),
        ];
        let output = fmt.format_stats(&results);
        assert!(output.contains("Quality Report"));
        assert!(output.contains("50%"));
    }

    #[test]
    fn test_format_stats_ko() {
        let fmt = Formatter::new(false, Lang::Ko);
        let results = vec![
            LintResult::new("a.md".to_string(), vec![sample_warning()]),
            LintResult::new("b.md".to_string(), vec![]),
        ];
        let output = fmt.format_stats(&results);
        assert!(output.contains("품질 리포트"));
        assert!(output.contains("규칙별 이슈"));
    }

    #[test]
    fn test_format_stats_perfect_score() {
        let fmt = Formatter::new(false, Lang::En);
        let results = vec![
            LintResult::new("a.md".to_string(), vec![]),
            LintResult::new("b.md".to_string(), vec![]),
        ];
        let output = fmt.format_stats(&results);
        assert!(output.contains("100%"));
    }
}
