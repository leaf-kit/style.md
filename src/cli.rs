use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "stylemd",
    version,
    about = "stylemd — Markdown styling toolkit",
    long_about = "Markdown styling toolkit — lint, format, fix, and polish your prose.\n\n\
        stylemd keeps your Markdown files clean, consistent, and well-structured\n\
        by enforcing style rules, fixing common issues, and generating quality reports.\n\n\
        v0.1.0 \"First Stroke\" — the foundational release that lays down\n\
        the essential styling rules every Markdown project needs.\n\n\
        By default, stylemd runs in light mode — fast syntax and structure checks.\n\
        Use --deep for thorough analysis including link validation and TOC checks.",
    after_help = "Discussion:\n    \
        stylemd treats every Markdown file as a canvas.\n    \
        Style isn't just formatting — it's clarity, consistency, and craft.\n\n    \
        Run `stylemd check` to detect style issues, `stylemd fix` to auto-repair them,\n    \
        or `stylemd fmt` to apply consistent formatting.\n\n    \
        Light mode (default): heading structure, trailing whitespace, blank lines,\n    \
        unclosed code blocks, list markers, image references.\n\n    \
        Deep mode (--deep): all light checks plus link validation (HTTP),\n    \
        code block language, anchor/relative links, TOC, and more.\n\n    \
        Configuration is read from .stylemd.toml in the current directory\n    \
        or ~/.config/stylemd/config.toml. Create one with `stylemd init`."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set log level to verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable progress output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Output language (en, ko)
    #[arg(long, global = true, default_value = "en")]
    pub lang: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check Markdown files for style issues (light mode by default)
    Check {
        /// File or directory path to check
        #[arg(default_value = ".")]
        path: String,

        /// Configuration file path
        #[arg(short, long)]
        config: Option<String>,

        /// Output format (pretty, json, yaml)
        #[arg(short, long, default_value = "pretty")]
        format: String,

        /// Deep mode: enable all checks including link validation
        #[arg(long)]
        deep: bool,

        /// Watch for file changes and re-check
        #[arg(long)]
        watch: bool,

        /// Show summary statistics
        #[arg(long)]
        summary: bool,

        /// Ignore files matching pattern
        #[arg(long)]
        ignore_pattern: Option<String>,

        /// Only check git staged files
        #[arg(long)]
        git_staged: bool,
    },

    /// Auto-format Markdown files
    #[command(name = "fmt")]
    Format {
        /// File or directory path to format
        #[arg(default_value = ".")]
        path: String,

        /// Configuration file path
        #[arg(short, long)]
        config: Option<String>,

        /// Show diff without applying changes
        #[arg(long)]
        diff: bool,

        /// Dry run — show what would change
        #[arg(long)]
        dry_run: bool,
    },

    /// Auto-fix lint errors in Markdown files
    Fix {
        /// File or directory path to fix
        #[arg(default_value = ".")]
        path: String,

        /// Configuration file path
        #[arg(short, long)]
        config: Option<String>,

        /// Deep mode: fix with all rules enabled
        #[arg(long)]
        deep: bool,

        /// Show diff of changes
        #[arg(long)]
        diff: bool,

        /// Dry run — show what would change without modifying files
        #[arg(long)]
        dry_run: bool,
    },

    /// Show quality statistics and scores
    Stats {
        /// File or directory path to analyze
        #[arg(default_value = ".")]
        path: String,

        /// Deep mode: include link and TOC analysis
        #[arg(long)]
        deep: bool,

        /// Output report format (pretty, json, yaml, md, html)
        #[arg(long, default_value = "pretty")]
        report: String,
    },

    /// Check all links in Markdown files (always deep)
    Links {
        /// File or directory path to check
        #[arg(default_value = ".")]
        path: String,
    },

    /// Start Language Server Protocol server
    #[command(name = "lsp")]
    Lsp,

    /// Create a default .stylemd.toml configuration file
    Init,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parse() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_check_command() {
        let cli = Cli::parse_from(["stylemd", "check", "test.md"]);
        match cli.command {
            Some(Commands::Check { path, .. }) => assert_eq!(path, "test.md"),
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_check_deep() {
        let cli = Cli::parse_from(["stylemd", "check", "--deep", "."]);
        match cli.command {
            Some(Commands::Check { deep, .. }) => assert!(deep),
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_check_light_default() {
        let cli = Cli::parse_from(["stylemd", "check", "."]);
        match cli.command {
            Some(Commands::Check { deep, .. }) => assert!(!deep),
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_fix_command() {
        let cli = Cli::parse_from(["stylemd", "fix", "--diff", "test.md"]);
        match cli.command {
            Some(Commands::Fix { path, diff, .. }) => {
                assert_eq!(path, "test.md");
                assert!(diff);
            }
            _ => panic!("Expected Fix command"),
        }
    }

    #[test]
    fn test_init_command() {
        let cli = Cli::parse_from(["stylemd", "init"]);
        assert!(matches!(cli.command, Some(Commands::Init)));
    }

    #[test]
    fn test_no_subcommand_shows_help() {
        let cli = Cli::parse_from(["stylemd"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_format_command() {
        let cli = Cli::parse_from(["stylemd", "fmt", "--dry-run", "."]);
        match cli.command {
            Some(Commands::Format { dry_run, .. }) => assert!(dry_run),
            _ => panic!("Expected Format command"),
        }
    }

    #[test]
    fn test_stats_command() {
        let cli = Cli::parse_from(["stylemd", "stats", "--report", "json"]);
        match cli.command {
            Some(Commands::Stats { report, .. }) => assert_eq!(report, "json"),
            _ => panic!("Expected Stats command"),
        }
    }
}
