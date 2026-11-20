# rich-guru

**rich-style terminal output for Rust** — ANSI-styled text (colors, bold, italic, underline) and aligned box-drawing tables, zero dependencies.

Formerly published on crates.io as `richguru`.

For developers coming from Python's [rich](https://github.com/Textualize/rich) or Node's [chalk](https://github.com/chalk/chalk) who want pretty CLI output without pulling a framework.

## Usage

```rust
use rich_guru::{table, Styled};

fn main() {
    println!("{}", Styled::new("status: ok").green().bold());

    let warning = Styled::new("disk almost full").red().underline();
    println!("{warning}");

    println!(
        "{}",
        table(
            &["name", "score"],
            &[
                vec!["alpha".into(), "100".into()],
                vec!["beta".into(), "87".into()],
            ],
        )
    );
}
```

## API

- `Styled::new(text)` — chain `.red() .green() .blue() .yellow() .magenta() .cyan() .gray()`, `.bold() .dim() .italic() .underline() .strikethrough()`, `.on_red() .on_blue() .on_gray()`, finish with `.render()` (returns the ANSI string) or use `Display`
- `table(headers, rows)` — aligned box-drawing table; column widths auto-fit content

## License

Licensed under either of MIT or Apache-2.0, at your option.

Name is a tribute to Python's rich and the styled-terminal tradition (chalk et al); this crate is an independent implementation for Rust with zero dependencies.
