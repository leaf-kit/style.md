mod cli;
mod config;
mod formatter;
mod i18n;
mod linter;
mod rules;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::Config;
use formatter::Formatter;
use i18n::{Lang, Messages};
use linter::{LintResult, Linter};
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let cli = Cli::parse();
    let use_color = atty::is(atty::Stream::Stdout);
    let lang = Lang::from_str(&cli.lang);

    match cli.command {
        Some(Commands::Check {
            path,
            config,
            format,
            deep,
            watch,
            summary,
            ignore_pattern,
            git_staged,
        }) => {
            let cfg = Config::load(config.as_deref());

            let files = if git_staged {
                get_git_staged_files()
            } else if path == "-" {
                return check_stdin(&cfg, &format, use_color, lang, deep);
            } else {
                collect_md_files(&path, ignore_pattern.as_deref())
            };

            if files.is_empty() {
                if use_color {
                    eprintln!(
                        "{} {}",
                        Messages::warning(lang).yellow().bold(),
                        Messages::no_md_files(lang)
                    );
                } else {
                    eprintln!(
                        "{} {}",
                        Messages::warning(lang),
                        Messages::no_md_files(lang)
                    );
                }
                return;
            }

            let results = lint_files(&files, &cfg, deep, lang);
            let fmt = Formatter::new(use_color, lang);

            match format.as_str() {
                "json" => println!("{}", fmt.format_json(&results)),
                "yaml" => println!("{}", fmt.format_yaml(&results)),
                _ => {
                    for result in &results {
                        let output = fmt.format_result(result);
                        if !output.is_empty() {
                            println!("{}", output);
                        }
                    }
                    if summary || !cli.quiet {
                        println!("{}", fmt.format_summary(&results));
                    }
                }
            }

            if deep && cfg.deep.toc_check {
                for file in &files {
                    if let Ok(content) = std::fs::read_to_string(file) {
                        check_toc(file, &content, use_color, lang);
                    }
                }
            }

            if watch {
                watch_files(&path, &cfg, deep, use_color, lang);
            }

            let has_errors = results.iter().any(|r| r.has_errors());
            std::process::exit(if has_errors { 1 } else { 0 });
        }

        Some(Commands::Format {
            path,
            config,
            diff,
            dry_run,
        }) => {
            let cfg = Config::load(config.as_deref());
            let files = collect_md_files(&path, None);
            let fmt = Formatter::new(use_color, lang);
            let linter = Linter::light(&cfg, None, lang);
            let mut fixed_count = 0;

            for file in &files {
                if let Ok(content) = std::fs::read_to_string(file) {
                    let fixed = linter.fix(&content);
                    if fixed != content {
                        fixed_count += 1;
                        if diff {
                            println!("{}", fmt.format_diff(&content, &fixed, file));
                        }
                        if !dry_run {
                            if let Err(e) = std::fs::write(file, &fixed) {
                                eprintln!("Error writing {}: {}", file, e);
                            } else if use_color {
                                println!("  {} {}", Messages::formatted(lang).green(), file);
                            } else {
                                println!("  {} {}", Messages::formatted(lang), file);
                            }
                        } else if use_color {
                            println!("  {} {}", Messages::would_format(lang).cyan(), file);
                        } else {
                            println!("  {} {}", Messages::would_format(lang), file);
                        }
                    }
                }
            }

            if use_color {
                println!(
                    "\n{} {}",
                    Messages::done(lang).bold(),
                    Messages::files_formatted(lang, fixed_count, dry_run)
                );
            } else {
                println!(
                    "\n{} {}",
                    Messages::done(lang),
                    Messages::files_formatted(lang, fixed_count, dry_run)
                );
            }
        }

        Some(Commands::Fix {
            path,
            config,
            deep,
            diff,
            dry_run,
        }) => {
            let cfg = Config::load(config.as_deref());
            let files = collect_md_files(&path, None);
            let fmt = Formatter::new(use_color, lang);
            let mut total_fixed = 0;

            for file in &files {
                if let Ok(content) = std::fs::read_to_string(file) {
                    let linter = if deep {
                        Linter::deep(&cfg, Some(Path::new(file)), lang)
                    } else {
                        Linter::light(&cfg, Some(Path::new(file)), lang)
                    };
                    let warnings = linter.check(&content);
                    let fixable: Vec<_> = warnings.iter().filter(|w| w.fixable).collect();

                    if fixable.is_empty() {
                        continue;
                    }

                    let fixed = linter.fix(&content);
                    if fixed != content {
                        let fix_count = fixable.len();
                        total_fixed += fix_count;

                        if diff {
                            println!("{}", fmt.format_diff(&content, &fixed, file));
                        }

                        if !dry_run {
                            if let Err(e) = std::fs::write(file, &fixed) {
                                eprintln!("Error writing {}: {}", file, e);
                            } else if use_color {
                                println!(
                                    "  {} {} ({})",
                                    Messages::fixed(lang).green().bold(),
                                    file,
                                    Messages::issues_fixed(lang, fix_count)
                                );
                            } else {
                                println!(
                                    "  {} {} ({})",
                                    Messages::fixed(lang),
                                    file,
                                    Messages::issues_fixed(lang, fix_count)
                                );
                            }
                        } else if use_color {
                            println!(
                                "  {} {} ({})",
                                Messages::dry_run(lang).cyan(),
                                file,
                                Messages::issues_would_fix(lang, fix_count)
                            );
                        } else {
                            println!(
                                "  {} {} ({})",
                                Messages::dry_run(lang),
                                file,
                                Messages::issues_would_fix(lang, fix_count)
                            );
                        }
                    }
                }
            }

            if use_color {
                println!(
                    "\n{} {}",
                    Messages::done(lang).bold(),
                    Messages::total_auto_fixed(lang, total_fixed, dry_run)
                );
            } else {
                println!(
                    "\n{} {}",
                    Messages::done(lang),
                    Messages::total_auto_fixed(lang, total_fixed, dry_run)
                );
            }
        }

        Some(Commands::Stats { path, deep, report }) => {
            let cfg = Config::load(None);
            let files = collect_md_files(&path, None);
            let results = lint_files(&files, &cfg, deep, lang);
            let fmt = Formatter::new(use_color, lang);

            match report.as_str() {
                "json" => println!("{}", fmt.format_json(&results)),
                "yaml" => println!("{}", fmt.format_yaml(&results)),
                "md" | "markdown" => println!("{}", format_report_md(&results)),
                "html" => println!("{}", format_report_html(&results)),
                _ => println!("{}", fmt.format_stats(&results)),
            }
        }

        Some(Commands::Links { path }) => {
            let cfg = Config::load(None);
            let files = collect_md_files(&path, None);
            let fmt = Formatter::new(use_color, lang);
            let mut all_results = Vec::new();

            for file in &files {
                if let Ok(content) = std::fs::read_to_string(file) {
                    let warnings = rules::link::check_links_in_file(
                        Path::new(file),
                        &content,
                        cfg.deep.external_links,
                        lang,
                    );
                    all_results.push(LintResult::new(file.clone(), warnings));
                }
            }

            for result in &all_results {
                let output = fmt.format_result(result);
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            println!("{}", fmt.format_summary(&all_results));
        }

        Some(Commands::Lsp) => {
            if use_color {
                eprintln!(
                    "{} {}",
                    Messages::info(lang).cyan().bold(),
                    Messages::lsp_not_implemented(lang)
                );
            } else {
                eprintln!(
                    "{} {}",
                    Messages::info(lang),
                    Messages::lsp_not_implemented(lang)
                );
            }
            std::process::exit(1);
        }

        Some(Commands::Init) => {
            let config_path = ".stylemd.toml";
            if Path::new(config_path).exists() {
                if use_color {
                    eprintln!(
                        "{} {} {}",
                        Messages::warning(lang).yellow().bold(),
                        config_path,
                        Messages::config_exists(lang)
                    );
                } else {
                    eprintln!(
                        "{} {} {}",
                        Messages::warning(lang),
                        config_path,
                        Messages::config_exists(lang)
                    );
                }
                std::process::exit(1);
            }
            match std::fs::write(config_path, Config::default_toml()) {
                Ok(_) => {
                    if use_color {
                        println!(
                            "{} {} {}",
                            "success:".green().bold(),
                            Messages::config_created(lang),
                            config_path
                        );
                    } else {
                        println!(
                            "success: {} {}",
                            Messages::config_created(lang),
                            config_path
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error creating config: {}", e);
                    std::process::exit(1);
                }
            }
        }

        None => {
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
        }
    }
}

fn collect_md_files(path: &str, ignore_pattern: Option<&str>) -> Vec<String> {
    let path = Path::new(path);

    if path.is_file() {
        return vec![path.to_string_lossy().to_string()];
    }

    let ignore_glob = ignore_pattern.and_then(|p| glob::Pattern::new(p).ok());

    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = entry.path();
        if p.is_file() {
            if let Some(ext) = p.extension() {
                if ext == "md" || ext == "markdown" {
                    let file_str = p.to_string_lossy().to_string();
                    if let Some(ref glob) = ignore_glob {
                        if glob.matches(&file_str) {
                            continue;
                        }
                    }
                    files.push(file_str);
                }
            }
        }
    }

    files.sort();
    files
}

fn lint_files(files: &[String], config: &Config, deep: bool, lang: Lang) -> Vec<LintResult> {
    files
        .iter()
        .filter_map(|file| {
            let content = std::fs::read_to_string(file).ok()?;
            let linter = if deep {
                Linter::deep(config, Some(Path::new(file)), lang)
            } else {
                Linter::light(config, Some(Path::new(file)), lang)
            };
            let warnings = linter.check(&content);
            Some(LintResult::new(file.clone(), warnings))
        })
        .collect()
}

fn check_stdin(config: &Config, format: &str, use_color: bool, lang: Lang, deep: bool) {
    let stdin = io::stdin();
    let content: String = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>()
        .join("\n");

    let linter = if deep {
        Linter::deep(config, None, lang)
    } else {
        Linter::light(config, None, lang)
    };
    let warnings = linter.check(&content);
    let result = LintResult::new("<stdin>".to_string(), warnings);
    let fmt = Formatter::new(use_color, lang);

    match format {
        "json" => println!("{}", fmt.format_json(&[result])),
        "yaml" => println!("{}", fmt.format_yaml(&[result])),
        _ => {
            let output = fmt.format_result(&result);
            if !output.is_empty() {
                println!("{}", output);
            }
            println!("{}", fmt.format_summary(&[result]));
        }
    }
}

fn get_git_staged_files() -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .filter(|l| l.ends_with(".md") || l.ends_with(".markdown"))
                .map(String::from)
                .collect()
        }
        Err(_) => Vec::new(),
    }
}

fn check_toc(file: &str, content: &str, use_color: bool, lang: Lang) {
    let headings = rules::link::extract_headings(content);
    if headings.is_empty() {
        return;
    }

    let toc_re = regex::Regex::new(r"(?i)##?\s*(table of contents|toc|contents)").unwrap();
    let has_toc = content.lines().any(|l| toc_re.is_match(l));

    if !has_toc && headings.len() > 3 {
        if use_color {
            println!(
                "  {} {}: {}",
                Messages::info(lang).cyan().bold(),
                file,
                Messages::toc_missing(lang, file, headings.len())
            );
        } else {
            println!(
                "  {} {}: {}",
                Messages::info(lang),
                file,
                Messages::toc_missing(lang, file, headings.len())
            );
        }
    }
}

fn watch_files(path: &str, config: &Config, deep: bool, use_color: bool, lang: Lang) {
    use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    let mut watcher = match RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        NotifyConfig::default(),
    ) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error starting watcher: {}", e);
            return;
        }
    };

    if let Err(e) = watcher.watch(Path::new(path), RecursiveMode::Recursive) {
        eprintln!("Error watching path: {}", e);
        return;
    }

    if use_color {
        println!(
            "\n{} {}",
            "watch:".cyan().bold(),
            Messages::watching(lang, path)
        );
    } else {
        println!("\nwatch: {}", Messages::watching(lang, path));
    }

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                let md_files: Vec<_> = event
                    .paths
                    .iter()
                    .filter(|p| {
                        p.extension()
                            .map(|e| e == "md" || e == "markdown")
                            .unwrap_or(false)
                    })
                    .collect();

                if !md_files.is_empty() {
                    if use_color {
                        println!(
                            "\n{} {}",
                            "watch:".cyan().bold(),
                            Messages::change_detected(lang)
                        );
                    } else {
                        println!("\nwatch: {}", Messages::change_detected(lang));
                    }

                    let files: Vec<String> = md_files
                        .iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect();
                    let results = lint_files(&files, config, deep, lang);
                    let fmt = Formatter::new(use_color, lang);
                    for result in &results {
                        let output = fmt.format_result(result);
                        if !output.is_empty() {
                            println!("{}", output);
                        }
                    }
                    println!("{}", fmt.format_summary(&results));
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(_) => break,
        }
    }
}

fn format_report_md(results: &[LintResult]) -> String {
    let mut output = Vec::new();
    output.push("# Markdown Lint Report\n".to_string());

    let total: usize = results.iter().map(|r| r.warnings.len()).sum();
    let errors: usize = results.iter().map(|r| r.error_count()).sum();
    let files = results.len();

    output.push(format!("**Files scanned:** {}", files));
    output.push(format!("**Total issues:** {}", total));
    output.push(format!("**Errors:** {}", errors));
    output.push(format!("**Warnings:** {}\n", total - errors));

    for result in results {
        if result.warnings.is_empty() {
            continue;
        }
        output.push(format!("## {}\n", result.file));
        output.push("| Line | Severity | Rule | Message |".to_string());
        output.push("|------|----------|------|---------|".to_string());
        for w in &result.warnings {
            output.push(format!(
                "| {} | {} | {} | {} |",
                w.line, w.severity, w.rule, w.message
            ));
        }
        output.push(String::new());
    }

    output.join("\n")
}

fn format_report_html(results: &[LintResult]) -> String {
    let total: usize = results.iter().map(|r| r.warnings.len()).sum();
    let errors: usize = results.iter().map(|r| r.error_count()).sum();

    let mut rows = String::new();
    for result in results {
        for w in &result.warnings {
            let severity_class = match w.severity {
                rules::Severity::Error => "error",
                rules::Severity::Warning => "warning",
                rules::Severity::Info => "info",
            };
            rows.push_str(&format!(
                "<tr class=\"{}\"><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                severity_class, result.file, w.line, w.severity, w.rule, w.message
            ));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>stylemd Report</title>
<style>
body {{ font-family: -apple-system, sans-serif; margin: 2rem; }}
h1 {{ color: #333; }}
.summary {{ background: #f5f5f5; padding: 1rem; border-radius: 8px; margin: 1rem 0; }}
table {{ border-collapse: collapse; width: 100%; }}
th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
th {{ background: #4a90d9; color: white; }}
.error {{ background: #ffe0e0; }}
.warning {{ background: #fff3e0; }}
.info {{ background: #e0f0ff; }}
</style>
</head>
<body>
<h1>stylemd Lint Report</h1>
<div class="summary">
<p><strong>Files:</strong> {} | <strong>Issues:</strong> {} | <strong>Errors:</strong> {} | <strong>Warnings:</strong> {}</p>
</div>
<table>
<tr><th>File</th><th>Line</th><th>Severity</th><th>Rule</th><th>Message</th></tr>
{}
</table>
</body>
</html>"#,
        results.len(),
        total,
        errors,
        total - errors,
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_md_files_single() {
        let dir = std::env::temp_dir().join("stylemd_test_collect");
        let _ = std::fs::create_dir_all(&dir);
        let test_file = dir.join("test.md");
        std::fs::write(&test_file, "# Test\n").unwrap();

        let files = collect_md_files(test_file.to_str().unwrap(), None);
        assert_eq!(files.len(), 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_collect_md_files_directory() {
        let dir = std::env::temp_dir().join("stylemd_test_dir");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("a.md"), "# A\n").unwrap();
        std::fs::write(dir.join("b.md"), "# B\n").unwrap();
        std::fs::write(dir.join("c.txt"), "not md\n").unwrap();

        let files = collect_md_files(dir.to_str().unwrap(), None);
        assert_eq!(files.len(), 2);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_lint_files() {
        let dir = std::env::temp_dir().join("stylemd_test_lint");
        let _ = std::fs::create_dir_all(&dir);
        let test_file = dir.join("test.md");
        std::fs::write(&test_file, "# Title\nSome text   \n").unwrap();

        let files = vec![test_file.to_string_lossy().to_string()];
        let config = Config::default();
        let results = lint_files(&files, &config, false, Lang::En); // light mode
        assert_eq!(results.len(), 1);
        assert!(!results[0].warnings.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_format_report_md() {
        let results = vec![LintResult::new("test.md".to_string(), vec![])];
        let report = format_report_md(&results);
        assert!(report.contains("# Markdown Lint Report"));
    }

    #[test]
    fn test_format_report_html() {
        let results = vec![LintResult::new("test.md".to_string(), vec![])];
        let report = format_report_html(&results);
        assert!(report.contains("<!DOCTYPE html>"));
        assert!(report.contains("stylemd"));
    }

    #[test]
    fn test_ignore_pattern() {
        let dir = std::env::temp_dir().join("stylemd_test_ignore");
        let _ = std::fs::create_dir_all(dir.join("node_modules"));
        std::fs::write(dir.join("a.md"), "# A\n").unwrap();
        std::fs::write(dir.join("node_modules/b.md"), "# B\n").unwrap();

        let files = collect_md_files(dir.to_str().unwrap(), Some("**/node_modules/**"));
        assert!(files.iter().all(|f| !f.contains("node_modules")));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
