use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lang {
    En,
    Ko,
}

impl Lang {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ko" | "kr" | "korean" => Lang::Ko,
            _ => Lang::En,
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::En => write!(f, "en"),
            Lang::Ko => write!(f, "ko"),
        }
    }
}

pub struct Messages;

impl Messages {
    // === Severity ===
    pub fn error(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "error",
            Lang::Ko => "오류",
        }
    }

    pub fn warning(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "warning",
            Lang::Ko => "경고",
        }
    }

    pub fn info(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "info",
            Lang::Ko => "정보",
        }
    }

    // === Heading rules ===
    pub fn multiple_h1(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Multiple H1 headings found; only one H1 is recommended",
            Lang::Ko => "H1 헤딩이 여러 개 발견됨; H1은 하나만 권장됩니다",
        }
    }

    pub fn heading_level_skip(lang: Lang, level: usize, prev: usize) -> String {
        match lang {
            Lang::En => format!(
                "Heading level skipped: H{} follows H{} (expected H{} or lower)",
                level,
                prev,
                prev + 1
            ),
            Lang::Ko => format!(
                "헤딩 레벨 건너뜀: H{}이 H{} 다음에 위치함 (H{} 이하 예상)",
                level,
                prev,
                prev + 1
            ),
        }
    }

    // === Blank line rules ===
    pub fn too_many_blank_lines(lang: Lang, found: usize, max: usize) -> String {
        match lang {
            Lang::En => format!(
                "Too many consecutive blank lines (found {}, max {})",
                found, max
            ),
            Lang::Ko => format!("연속 빈 줄이 너무 많음 ({}개 발견, 최대 {}개)", found, max),
        }
    }

    pub fn missing_blank_before_heading(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Missing blank line before heading",
            Lang::Ko => "헤딩 앞에 빈 줄 누락",
        }
    }

    pub fn missing_blank_after_heading(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Missing blank line after heading",
            Lang::Ko => "헤딩 뒤에 빈 줄 누락",
        }
    }

    // === Trailing whitespace ===
    pub fn trailing_whitespace(lang: Lang, chars: usize) -> String {
        match lang {
            Lang::En => format!("Trailing whitespace found ({} chars)", chars),
            Lang::Ko => format!("후행 공백 발견 ({}자)", chars),
        }
    }

    pub fn trailing_whitespace_with_tabs(lang: Lang, chars: usize) -> String {
        match lang {
            Lang::En => format!(
                "Trailing whitespace (including tabs) found ({} chars)",
                chars
            ),
            Lang::Ko => format!("후행 공백(탭 포함) 발견 ({}자)", chars),
        }
    }

    // === Code block rules ===
    pub fn code_block_missing_lang(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Code block missing language specifier",
            Lang::Ko => "코드 블록에 언어 지정 누락",
        }
    }

    pub fn unclosed_code_block(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Unclosed code block (missing closing ```)",
            Lang::Ko => "닫히지 않은 코드 블록 (``` 닫힘 누락)",
        }
    }

    // === List rules ===
    pub fn list_marker_inconsistent(lang: Lang, found: &str, expected: &str) -> String {
        match lang {
            Lang::En => format!(
                "List marker '{}' should be '{}' for consistency",
                found, expected
            ),
            Lang::Ko => format!(
                "목록 기호 '{}'은(는) 일관성을 위해 '{}'이어야 합니다",
                found, expected
            ),
        }
    }

    pub fn list_bad_indentation(lang: Lang, spaces: usize) -> String {
        match lang {
            Lang::En => format!(
                "Inconsistent list indentation ({} spaces); use multiples of 2 or 4",
                spaces
            ),
            Lang::Ko => format!(
                "불일치하는 목록 들여쓰기 ({}칸); 2 또는 4의 배수를 사용하세요",
                spaces
            ),
        }
    }

    // === Link rules ===
    pub fn broken_anchor(lang: Lang, url: &str) -> String {
        match lang {
            Lang::En => format!("Broken anchor link: {} (heading not found)", url),
            Lang::Ko => format!("깨진 앵커 링크: {} (헤딩을 찾을 수 없음)", url),
        }
    }

    pub fn broken_relative(lang: Lang, url: &str) -> String {
        match lang {
            Lang::En => format!("Broken relative link: {} (file not found)", url),
            Lang::Ko => format!("깨진 상대 링크: {} (파일을 찾을 수 없음)", url),
        }
    }

    pub fn broken_external(lang: Lang, url: &str, status: u16) -> String {
        match lang {
            Lang::En => format!("Broken external link: {} (HTTP {})", url, status),
            Lang::Ko => format!("깨진 외부 링크: {} (HTTP {})", url, status),
        }
    }

    pub fn unreachable_link(lang: Lang, url: &str, err: &str) -> String {
        match lang {
            Lang::En => format!("Unreachable link: {} ({})", url, err),
            Lang::Ko => format!("접근 불가 링크: {} ({})", url, err),
        }
    }

    // === Summary / formatter ===
    pub fn summary(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Summary:",
            Lang::Ko => "요약:",
        }
    }

    pub fn files_checked(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "files checked",
            Lang::Ko => "파일 검사 완료",
        }
    }

    pub fn with_issues(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "with issues",
            Lang::Ko => "이슈 있음",
        }
    }

    pub fn errors(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "errors",
            Lang::Ko => "오류",
        }
    }

    pub fn warnings(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "warnings",
            Lang::Ko => "경고",
        }
    }

    pub fn auto_fixable(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "auto-fixable",
            Lang::Ko => "자동 수정 가능",
        }
    }

    pub fn use_fix(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "use `stylemd fix` to fix",
            Lang::Ko => "`stylemd fix`로 수정하세요",
        }
    }

    pub fn all_clean(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "All files are clean!",
            Lang::Ko => "모든 파일이 깨끗합니다!",
        }
    }

    pub fn fixable_tag(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "fixable",
            Lang::Ko => "수정가능",
        }
    }

    // === Commands ===
    pub fn done(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Done:",
            Lang::Ko => "완료:",
        }
    }

    pub fn formatted(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "formatted",
            Lang::Ko => "포맷 완료",
        }
    }

    pub fn would_format(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "would format",
            Lang::Ko => "포맷 예정",
        }
    }

    pub fn files_formatted(lang: Lang, n: usize, dry_run: bool) -> String {
        match lang {
            Lang::En => format!(
                "{} file(s) {}",
                n,
                if dry_run {
                    "would be formatted"
                } else {
                    "formatted"
                }
            ),
            Lang::Ko => format!(
                "{}개 파일 {}",
                n,
                if dry_run {
                    "포맷 예정"
                } else {
                    "포맷 완료"
                }
            ),
        }
    }

    pub fn fixed(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "fixed",
            Lang::Ko => "수정 완료",
        }
    }

    pub fn issues_fixed(lang: Lang, n: usize) -> String {
        match lang {
            Lang::En => format!("{} issues fixed", n),
            Lang::Ko => format!("{}개 이슈 수정 완료", n),
        }
    }

    pub fn issues_would_fix(lang: Lang, n: usize) -> String {
        match lang {
            Lang::En => format!("{} issues would be fixed", n),
            Lang::Ko => format!("{}개 이슈 수정 예정", n),
        }
    }

    pub fn dry_run(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "dry-run",
            Lang::Ko => "미리보기",
        }
    }

    pub fn total_auto_fixed(lang: Lang, n: usize, dry_run: bool) -> String {
        match lang {
            Lang::En => format!(
                "{} issue(s) {}",
                n,
                if dry_run {
                    "would be auto-fixed"
                } else {
                    "auto-fixed"
                }
            ),
            Lang::Ko => format!(
                "{}개 이슈 {}",
                n,
                if dry_run {
                    "자동 수정 예정"
                } else {
                    "자동 수정 완료"
                }
            ),
        }
    }

    pub fn no_md_files(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "No markdown files found",
            Lang::Ko => "마크다운 파일을 찾을 수 없습니다",
        }
    }

    pub fn config_created(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Created",
            Lang::Ko => "생성 완료",
        }
    }

    pub fn config_exists(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "already exists",
            Lang::Ko => "이미 존재합니다",
        }
    }

    pub fn lsp_not_implemented(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "LSP server mode is not yet implemented",
            Lang::Ko => "LSP 서버 모드는 아직 구현되지 않았습니다",
        }
    }

    // === Stats ===
    pub fn quality_report(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Quality Report",
            Lang::Ko => "품질 리포트",
        }
    }

    pub fn files_scanned(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Files scanned:",
            Lang::Ko => "검사한 파일:",
        }
    }

    pub fn clean_files(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Clean files:",
            Lang::Ko => "깨끗한 파일:",
        }
    }

    pub fn total_issues(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Total issues:",
            Lang::Ko => "전체 이슈:",
        }
    }

    pub fn errors_label(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Errors:",
            Lang::Ko => "오류:",
        }
    }

    pub fn warnings_label(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Warnings:",
            Lang::Ko => "경고:",
        }
    }

    pub fn auto_fixable_label(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Auto-fixable:",
            Lang::Ko => "자동 수정 가능:",
        }
    }

    pub fn quality_score(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Quality score:",
            Lang::Ko => "품질 점수:",
        }
    }

    pub fn issues_by_rule(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Issues by rule:",
            Lang::Ko => "규칙별 이슈:",
        }
    }

    // === Watch ===
    pub fn watching(lang: Lang, path: &str) -> String {
        match lang {
            Lang::En => format!("Watching {} for changes... (Ctrl+C to stop)", path),
            Lang::Ko => format!("{} 변경 감시 중... (Ctrl+C로 중지)", path),
        }
    }

    pub fn change_detected(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Change detected, re-checking...",
            Lang::Ko => "변경 감지, 재검사 중...",
        }
    }

    // === TOC ===
    pub fn toc_missing(lang: Lang, _file: &str, count: usize) -> String {
        match lang {
            Lang::En => format!("Document has {} headings but no Table of Contents", count),
            Lang::Ko => format!("문서에 {}개 헤딩이 있지만 목차(TOC)가 없습니다", count),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_from_str() {
        assert_eq!(Lang::from_str("en"), Lang::En);
        assert_eq!(Lang::from_str("ko"), Lang::Ko);
        assert_eq!(Lang::from_str("kr"), Lang::Ko);
        assert_eq!(Lang::from_str("korean"), Lang::Ko);
        assert_eq!(Lang::from_str("unknown"), Lang::En);
    }

    #[test]
    fn test_lang_display() {
        assert_eq!(format!("{}", Lang::En), "en");
        assert_eq!(format!("{}", Lang::Ko), "ko");
    }

    #[test]
    fn test_messages_en() {
        assert_eq!(Messages::error(Lang::En), "error");
        assert_eq!(Messages::warning(Lang::En), "warning");
        assert_eq!(Messages::all_clean(Lang::En), "All files are clean!");
    }

    #[test]
    fn test_messages_ko() {
        assert_eq!(Messages::error(Lang::Ko), "오류");
        assert_eq!(Messages::warning(Lang::Ko), "경고");
        assert!(Messages::all_clean(Lang::Ko).contains("깨끗"));
    }

    #[test]
    fn test_heading_level_skip_en() {
        let msg = Messages::heading_level_skip(Lang::En, 3, 1);
        assert!(msg.contains("H3"));
        assert!(msg.contains("H1"));
    }

    #[test]
    fn test_heading_level_skip_ko() {
        let msg = Messages::heading_level_skip(Lang::Ko, 3, 1);
        assert!(msg.contains("H3"));
        assert!(msg.contains("건너뜀"));
    }

    #[test]
    fn test_trailing_whitespace_ko() {
        let msg = Messages::trailing_whitespace(Lang::Ko, 3);
        assert!(msg.contains("후행 공백"));
        assert!(msg.contains("3"));
    }

    #[test]
    fn test_broken_anchor_ko() {
        let msg = Messages::broken_anchor(Lang::Ko, "#test");
        assert!(msg.contains("깨진 앵커"));
        assert!(msg.contains("#test"));
    }

    #[test]
    fn test_files_formatted_ko() {
        let msg = Messages::files_formatted(Lang::Ko, 5, false);
        assert!(msg.contains("5"));
        assert!(msg.contains("포맷 완료"));
    }

    #[test]
    fn test_files_formatted_dry_run() {
        let msg = Messages::files_formatted(Lang::En, 3, true);
        assert!(msg.contains("would be formatted"));
    }
}
