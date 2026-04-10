use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub light: LightConfig,
    #[serde(default)]
    pub deep: DeepConfig,
    #[serde(default)]
    pub format: FormatConfig,
    #[serde(default)]
    pub ignore: Vec<String>,
}

/// Light mode: fast syntax and structure checks (default)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightConfig {
    #[serde(default = "default_true")]
    pub heading_structure: bool,
    #[serde(default = "default_true")]
    pub trailing_whitespace: bool,
    #[serde(default = "default_true")]
    pub blank_lines_around_headings: bool,
    #[serde(default = "default_true")]
    pub unclosed_code_block: bool,
    #[serde(default = "default_true")]
    pub list_marker_consistency: bool,
    #[serde(default = "default_true")]
    pub image_file_exists: bool,
    #[serde(default = "default_list_marker")]
    pub list_marker: String,
}

/// Deep mode: thorough analysis (--deep)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepConfig {
    #[serde(default = "default_true")]
    pub code_block_language: bool,
    #[serde(default = "default_true")]
    pub consecutive_blank_lines: bool,
    #[serde(default = "default_max_blank_lines")]
    pub max_consecutive_blank_lines: usize,
    #[serde(default = "default_true")]
    pub anchor_links: bool,
    #[serde(default = "default_true")]
    pub relative_links: bool,
    #[serde(default = "default_true")]
    pub external_links: bool,
    #[serde(default = "default_true")]
    pub toc_check: bool,
    #[serde(default = "default_timeout")]
    pub link_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    #[serde(default = "default_true")]
    pub ensure_final_newline: bool,
    #[serde(default = "default_true")]
    pub trim_trailing_whitespace: bool,
    #[serde(default = "default_max_blank_lines")]
    pub max_consecutive_blank_lines: usize,
    #[serde(default = "default_list_marker")]
    pub list_marker: String,
}

fn default_true() -> bool {
    true
}
fn default_max_blank_lines() -> usize {
    1
}
fn default_list_marker() -> String {
    "-".to_string()
}
fn default_timeout() -> u64 {
    10
}

impl Default for LightConfig {
    fn default() -> Self {
        Self {
            heading_structure: true,
            trailing_whitespace: true,
            blank_lines_around_headings: true,
            unclosed_code_block: true,
            list_marker_consistency: true,
            image_file_exists: true,
            list_marker: "-".to_string(),
        }
    }
}

impl Default for DeepConfig {
    fn default() -> Self {
        Self {
            code_block_language: true,
            consecutive_blank_lines: true,
            max_consecutive_blank_lines: 1,
            anchor_links: true,
            relative_links: true,
            external_links: true,
            toc_check: true,
            link_timeout_seconds: 10,
        }
    }
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            ensure_final_newline: true,
            trim_trailing_whitespace: true,
            max_consecutive_blank_lines: 1,
            list_marker: "-".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: Option<&str>) -> Self {
        if let Some(p) = path {
            return Self::load_from_file(Path::new(p)).unwrap_or_default();
        }

        for candidate in Self::config_search_paths() {
            if candidate.exists() {
                if let Ok(config) = Self::load_from_file(&candidate) {
                    return config;
                }
            }
        }

        Self::default()
    }

    fn config_search_paths() -> Vec<PathBuf> {
        let mut paths = vec![PathBuf::from(".stylemd.toml")];
        if let Some(home) = dirs_home() {
            paths.push(home.join(".config").join("stylemd").join("config.toml"));
        }
        paths
    }

    fn load_from_file(path: &Path) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn default_toml() -> String {
        r#"# stylemd configuration
# https://github.com/leaf-kit/style.md
#
# stylemd has two check modes:
#
#   stylemd check [path]         Light mode (default) — fast, essential checks
#   stylemd check --deep [path]  Deep mode — thorough analysis including links
#
# Light mode runs: heading structure, trailing whitespace, blank lines around
# headings, unclosed code blocks, list marker consistency, image file existence.
#
# Deep mode adds: code block language, consecutive blank lines, anchor/relative/
# external link validation, TOC check.

# ── Light mode rules (default) ──────────────────────────────────────────
[light]
heading_structure = true           # Detect duplicate H1, heading level skips
trailing_whitespace = true         # Detect trailing spaces and tabs
blank_lines_around_headings = true # Require blank lines before/after headings
unclosed_code_block = true         # Detect unclosed ``` blocks
list_marker_consistency = true     # Enforce consistent list markers (- * +)
image_file_exists = true           # Check that referenced image files exist
list_marker = "-"                  # Preferred list marker (-, *, +)

# ── Deep mode rules (--deep) ────────────────────────────────────────────
[deep]
code_block_language = true         # Require language specifier on code blocks
consecutive_blank_lines = true     # Limit consecutive blank lines
max_consecutive_blank_lines = 1    # Maximum allowed consecutive blank lines
anchor_links = true                # Validate internal anchor links (#section)
relative_links = true              # Check relative file links exist
external_links = true              # HTTP check external URLs (slow)
toc_check = true                   # Verify TOC matches heading structure
link_timeout_seconds = 10          # Timeout for HTTP link checks

# ── Formatting rules (stylemd fmt) ──────────────────────────────────────
[format]
ensure_final_newline = true
trim_trailing_whitespace = true
max_consecutive_blank_lines = 1
list_marker = "-"

# ── Ignore patterns ────────────────────────────────────────────────────
# ignore = ["node_modules/**", "target/**", ".git/**"]
"#
        .to_string()
    }
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.light.heading_structure);
        assert!(config.light.trailing_whitespace);
        assert!(config.light.image_file_exists);
        assert!(config.deep.code_block_language);
        assert!(config.deep.external_links);
    }

    #[test]
    fn test_parse_toml() {
        let toml_str = r#"
[light]
heading_structure = false
trailing_whitespace = true

[deep]
external_links = false

[format]
list_marker = "*"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(!config.light.heading_structure);
        assert!(config.light.trailing_whitespace);
        assert!(!config.deep.external_links);
        assert_eq!(config.format.list_marker, "*");
    }

    #[test]
    fn test_default_toml_is_valid() {
        let toml_str = Config::default_toml();
        let config: Result<Config, _> = toml::from_str(&toml_str);
        assert!(config.is_ok());
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let config = Config::load(Some("/nonexistent/path.toml"));
        assert!(config.light.heading_structure);
    }

    #[test]
    fn test_deep_defaults() {
        let config = DeepConfig::default();
        assert_eq!(config.link_timeout_seconds, 10);
        assert!(config.anchor_links);
        assert!(config.toc_check);
    }

    #[test]
    fn test_light_defaults() {
        let config = LightConfig::default();
        assert!(config.image_file_exists);
        assert!(config.unclosed_code_block);
        assert_eq!(config.list_marker, "-");
    }
}
