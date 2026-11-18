use std::fmt::Write as _;

#[derive(Clone, Debug, Default)]
pub struct Styled {
    text: String,
    codes: Vec<&'static str>,
}

impl Styled {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            codes: Vec::new(),
        }
    }

    fn code(mut self, code: &'static str) -> Self {
        self.codes.push(code);
        self
    }

    pub fn red(self) -> Self {
        self.code("\x1b[31m")
    }

    pub fn green(self) -> Self {
        self.code("\x1b[32m")
    }

    pub fn yellow(self) -> Self {
        self.code("\x1b[33m")
    }

    pub fn blue(self) -> Self {
        self.code("\x1b[34m")
    }

    pub fn magenta(self) -> Self {
        self.code("\x1b[35m")
    }

    pub fn cyan(self) -> Self {
        self.code("\x1b[36m")
    }

    pub fn gray(self) -> Self {
        self.code("\x1b[90m")
    }

    pub fn bold(self) -> Self {
        self.code("\x1b[1m")
    }

    pub fn dim(self) -> Self {
        self.code("\x1b[2m")
    }

    pub fn italic(self) -> Self {
        self.code("\x1b[3m")
    }

    pub fn underline(self) -> Self {
        self.code("\x1b[4m")
    }

    pub fn strikethrough(self) -> Self {
        self.code("\x1b[9m")
    }

    pub fn on_red(self) -> Self {
        self.code("\x1b[41m")
    }

    pub fn on_blue(self) -> Self {
        self.code("\x1b[44m")
    }

    pub fn on_gray(self) -> Self {
        self.code("\x1b[100m")
    }

    pub fn render(self) -> String {
        let mut out = String::with_capacity(self.text.len() + self.codes.len() * 5 + 4);
        for code in &self.codes {
            out.push_str(code);
        }
        out.push_str(&self.text);
        if !self.codes.is_empty() {
            out.push_str("\x1b[0m");
        }
        out
    }
}

impl std::fmt::Display for Styled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.clone().render())
    }
}

pub fn table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let column_count = headers.len();
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(column_count) {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }
    let mut out = String::new();
    let separator = {
        let mut s = String::from("+");
        for w in &widths {
            s.push_str(&"-".repeat(w + 2));
            s.push('+');
        }
        s
    };
    out.push_str(&separator);
    out.push('\n');
    let header_row: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    out.push_str(&render_row(&header_row, &widths));
    out.push('\n');
    out.push_str(&separator);
    out.push('\n');
    for row in rows {
        let mut padded = row.clone();
        padded.resize(column_count, String::new());
        out.push_str(&render_row(&padded, &widths));
        out.push('\n');
    }
    out.push_str(&separator);
    out
}

fn render_row(cells: &[String], widths: &[usize]) -> String {
    let mut out = String::from("|");
    for (i, cell) in cells.iter().enumerate() {
        let width = widths.get(i).copied().unwrap_or(cell.len());
        let _ = write!(out, " {cell:<width$} |");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styled_wraps_with_ansi_codes() {
        let out = Styled::new("alert").red().bold().render();
        assert!(out.starts_with("\x1b[31m"));
        assert!(out.contains("\x1b[1m"));
        assert!(out.ends_with("\x1b[0m"));
        assert!(out.contains("alert"));
    }

    #[test]
    fn plain_text_has_no_codes() {
        let out = Styled::new("plain").render();
        assert_eq!(out, "plain");
    }

    #[test]
    fn table_aligns_columns() {
        let rendered = table(
            &["name", "score"],
            &[
                vec!["alpha".into(), "100".into()],
                vec!["b".into(), "7".into()],
            ],
        );
        assert!(rendered.contains("| name  |"));
        assert!(rendered.contains("| score |"));
        assert!(rendered.contains("| alpha | 100   |"));
        assert!(rendered.contains("| b     | 7     |"));
        assert!(rendered.starts_with('+'));
    }

    #[test]
    fn color_helpers_exist() {
        let _ = Styled::new("x")
            .green()
            .yellow()
            .blue()
            .magenta()
            .cyan()
            .gray();
        let _ = Styled::new("x").dim().italic().underline().strikethrough();
        let _ = Styled::new("x").on_red().on_blue().on_gray();
    }
}
