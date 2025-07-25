use std::{ops::Range, sync::Arc};

use crate::{render::Renderer, Diagnostic, Help, Label, Severity, Source, Suggestion};

use indexmap::IndexMap;
use owo_colors::{OwoColorize, Style, Styled};

use super::Formatter;

const DEFAULT_TERM_WIDTH: usize = 80;

#[derive(Debug, Clone)]
pub struct ThemeStyle {
    pub error: Style,
    pub warning: Style,
    pub info: Style,
    pub note: Style,
    pub help: Style,

    pub deletion: Style,
    pub insertion: Style,

    pub link: Style,
    pub gutter: Style,
}

impl ThemeStyle {
    /// Defines a preset which utilizes RGB colors within the terminal.
    pub fn rgb() -> Self {
        ThemeStyle {
            error: Style::new().fg_rgb::<233, 114, 99>().bold(),
            warning: Style::new().fg_rgb::<235, 191, 131>().bold(),
            info: Style::new().fg_rgb::<114, 159, 207>(),
            note: Style::new().fg_rgb::<166, 227, 161>(),
            help: Style::new().fg_rgb::<171, 161, 247>(),

            deletion: Style::new().fg_rgb::<233, 114, 99>(),
            insertion: Style::new().fg_rgb::<166, 227, 161>(),

            link: Style::new().fg_rgb::<166, 173, 200>(),
            gutter: Style::new().fg_rgb::<156, 156, 192>(),
        }
    }

    /// Defines a preset which utilizes ANSI color codes within the terminal.
    pub fn ansi() -> Self {
        ThemeStyle {
            error: Style::new().bright_red().bold(),
            warning: Style::new().bright_yellow().bold(),
            info: Style::new().bright_blue().bold(),
            note: Style::new().bright_green().bold(),
            help: Style::new().bright_cyan().bold(),

            deletion: Style::new().bright_red(),
            insertion: Style::new().bright_green(),

            link: Style::new().bright_white(),
            gutter: Style::new().bright_white(),
        }
    }

    /// Retrieves the style which is utilized for the given severity.
    pub fn from_severity(&self, severity: Severity) -> Style {
        match severity {
            Severity::Error => self.error,
            Severity::Warning => self.warning,
            Severity::Info => self.info,
            Severity::Note => self.note,
            Severity::Help => self.help,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeSymbols {
    pub error: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub note: &'static str,
    pub help: &'static str,
}

impl ThemeSymbols {
    pub fn unicode() -> Self {
        ThemeSymbols {
            error: "×",
            warning: "⚠",
            info: "☞",
            note: "☞",
            help: "☞",
        }
    }

    pub fn from_severity(&self, severity: Severity) -> &'static str {
        match severity {
            Severity::Error => self.error,
            Severity::Warning => self.warning,
            Severity::Info => self.info,
            Severity::Note => self.note,
            Severity::Help => self.help,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrowSymbols {
    /// "─"
    pub horizontal: char,

    /// "│"
    pub vertical: char,

    /// "∶"
    pub vertical_break: char,

    /// "╭"
    pub top_left: char,

    /// "╰"
    pub bottom_left: char,

    /// "├"
    pub horizontal_right: char,

    /// "^"
    pub arrow_up: char,

    /// "↓"
    pub arrow_down: char,
}

impl ArrowSymbols {
    pub fn unicode() -> Self {
        ArrowSymbols {
            horizontal: '─',
            vertical: '│',
            vertical_break: '∶',
            top_left: '╭',
            bottom_left: '╰',
            horizontal_right: '├',
            arrow_up: '^',
            arrow_down: '↓',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub style: ThemeStyle,
    pub symbols: ThemeSymbols,
    pub arrows: ArrowSymbols,
}

impl Theme {
    /// Returns an instance of [`Theme`] which uses the "fancy" preset.
    ///
    /// The fancy preset uses RGB colors and unicode symbols for the diagnostics.
    pub fn fancy() -> Self {
        Theme {
            style: ThemeStyle::rgb(),
            symbols: ThemeSymbols::unicode(),
            arrows: ArrowSymbols::unicode(),
        }
    }
}

struct LabelGroup {
    /// Defines all the labels in the group
    pub labels: Vec<Label>,

    /// Defines the common source for the labels
    pub source: Arc<dyn Source>,
}

/// An implementation of [`Renderer`] which displays diagnostics in a graphical way
/// in the console using colors, Unicode symbols and highlighting.
///
/// # Examples
///
/// ```
/// use error_snippet::{Renderer, GraphicalRenderer};
///
/// let renderer = GraphicalRenderer::new();
/// ```
#[derive(Debug, Clone)]
pub struct GraphicalRenderer {
    /// Defines the theme of the renderer.
    ///
    /// The theme defines which colors and symbols to use when rendering diagnostics.
    pub theme: Theme,

    /// Defines the maximum length of the terminal.
    pub width: usize,

    /// Defines the padding to use per level of identation.
    pub padding: usize,

    /// Defines the margin to use in the gutter of snippets.
    pub gutter_margin: usize,

    /// Defines the amount of lines surrounding a label to include as context.
    pub context_lines: usize,

    /// Defines whether to use colors in the output.
    pub use_colors: bool,

    /// Defines whether to highlight the source code where a label
    /// is marked. This is only used if `use_colors` is `true`.
    pub highlight_source: bool,

    /// Defiens the current indentation level.
    current_indent: usize,
}

impl Default for GraphicalRenderer {
    fn default() -> Self {
        GraphicalRenderer::new()
    }
}

impl Renderer for GraphicalRenderer {
    fn render_fmt(
        &mut self,
        f: &mut Formatter<'_>,
        diagnostic: &dyn Diagnostic,
    ) -> std::fmt::Result {
        self.render_diagnostic(f, diagnostic)
    }
}

impl GraphicalRenderer {
    /// Creates a new instance of [`GraphicalRenderer`] with default settings.
    pub fn new() -> Self {
        GraphicalRenderer {
            theme: Theme::fancy(),
            width: terminal_width(),
            padding: 6,
            gutter_margin: 2,
            context_lines: 1,
            use_colors: true,
            highlight_source: false,
            current_indent: 0,
        }
    }

    /// Gets the current indentation to use, in amounts of spaces.
    fn ident(&self) -> usize {
        self.current_indent * self.padding
    }

    /// Writes the the given amount of padding to the provided writer.
    fn write_padding(&self, f: &mut impl std::fmt::Write, padding: usize) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(padding))
    }

    /// Writes the current indentation to the given writer.
    fn write_ident(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.write_padding(f, self.ident())
    }

    /// Styles the given value with the provided style.
    ///
    /// If colors are disabled on the renderer, no styles are applied and the
    /// value is kept unstyled.
    fn style<'a, T: std::fmt::Display>(&self, val: &'a T, style: Style) -> Styled<&'a T> {
        if self.use_colors {
            val.style(style)
        } else {
            val.style(Style::new())
        }
    }

    /// Styles a substring the given value with the provided style.
    ///
    /// If colors are disabled on the renderer, no styles are applied and the
    /// value is kept unstyled.
    fn style_substring(
        &self,
        val: impl Into<String>,
        span: impl Into<Range<usize>>,
        style: Style,
    ) -> Styled<String> {
        let val: String = val.into();

        if !self.use_colors {
            return Style::new().style(val);
        }

        let span: Range<usize> = span.into();

        let (middle, after) = val.split_at(span.end);
        let middle = middle.to_string();

        let (before, middle) = middle.split_at(span.start);

        let styled = format!("{}{}{}", before, middle.style(style), after);

        Style::new().style(styled)
    }

    /// Determines how much padding to use for the gutter of the
    /// given source code. The gutter margin is included in the result.
    fn gutter_size_of(&self, source: &str) -> usize {
        let largest_line_size = source.lines().count().to_string().len();

        largest_line_size + self.gutter_margin
    }

    /// Renders the given diagnostic to the provided writer.
    ///
    /// # Example
    ///
    /// ```text
    /// error[E4012]
    ///    × invalid doc comment found
    ///     ╭─[std/array.lm:32:8]
    ///  31 │
    ///  32 │        /// Allocate the minimum amount of capacity in the array.
    ///  33 │        array.reserve(capacity);
    ///     │        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doc comment found on statement
    ///     ╰──
    ///    help: doc comments are only allowed on definitions
    /// ```
    fn render_diagnostic(
        &mut self,
        f: &mut impl std::fmt::Write,
        diagnostic: &dyn Diagnostic,
    ) -> std::fmt::Result {
        owo_colors::with_override(self.use_colors, || {
            self.render_header(f, diagnostic)?;
            self.render_source(f, diagnostic)?;
            self.render_footer(f, diagnostic)?;

            Result::Ok(())
        })
    }

    /// Renders the header of the diagnostic message, which includes severity and diagnostic code (if any).
    ///
    /// # Example
    ///
    /// ```text
    ///   × error[E4012]: invalid doc comment found
    /// ```
    fn render_header(
        &self,
        f: &mut impl std::fmt::Write,
        diagnostic: &dyn Diagnostic,
    ) -> std::fmt::Result {
        let severity_symbol = self.theme.symbols.from_severity(diagnostic.severity());
        let severity_style = self.theme.style.from_severity(diagnostic.severity());
        let severity_str = diagnostic.severity().to_string();

        self.write_ident(f)?;
        write!(
            f,
            "{} {}",
            self.style(&severity_symbol, severity_style),
            self.style(&severity_str, severity_style)
        )?;

        if let Some(code) = &diagnostic.code() {
            write!(f, "{}", self.style(&format!("[{code}]"), severity_style))?;
        }

        writeln!(f, ": {}", diagnostic.message())
    }

    /// Renders the source span of the diagnostic, if any, attached with any associated labels.
    ///
    /// # Example
    ///
    /// ```text
    ///       ╭─[std/array.lm:29:46]
    ///    28 │    /// When creating an array with a set capacity, it's length will still be zero.
    ///    29 │    pub fn with_capacity(capacity: UInt64) -> Array<T> {
    ///       │                                              ^^^^^^^^ expected type `Array<T>` found here
    ///       ∶
    ///    34 │
    ///    35 │        return true;
    ///       │        ^^^^^^^^^^^^ expected `Array<T>`, found `Boolean`
    ///       ╰──
    /// ```
    fn render_source(
        &mut self,
        f: &mut impl std::fmt::Write,
        diagnostic: &dyn Diagnostic,
    ) -> std::fmt::Result {
        for cause in diagnostic.causes() {
            self.current_indent += 1;

            self.render_diagnostic(f, cause)?;
            writeln!(f)?;

            self.current_indent -= 1;
        }

        if let Some(labels) = diagnostic.labels() {
            let mut label_groups: IndexMap<Option<String>, LabelGroup> = IndexMap::new();

            // Group the labels into groups where all elements have
            // the same source file. This helps prevent multiple label
            // headers in a row from defining the same file path.
            for label in labels {
                // If no source code is attached to the label itself, see if
                // a source is attached to the parent diagnostic.
                //
                // If no source is found on either, skip over the label entirely.
                //
                // TODO: should be print a warning when no source is found?
                let source = match label.source() {
                    Some(s) => s.clone(),
                    None => match diagnostic.source_code() {
                        Some(s) => s,
                        None => continue,
                    },
                };

                let source_name = source.name().map(|n| n.to_string());

                if let Some(group) = label_groups.get_mut(&source_name) {
                    group.labels.push(label);
                } else {
                    label_groups.insert(
                        source_name,
                        LabelGroup {
                            source,
                            labels: vec![label],
                        },
                    );
                }
            }

            for (_, group) in label_groups {
                self.render_label_group(f, group, diagnostic.severity())?;
            }
        }

        for related in diagnostic.related() {
            self.current_indent += 1;

            self.render_diagnostic(f, related)?;
            writeln!(f)?;

            self.current_indent -= 1;
        }

        Ok(())
    }

    /// Renders a label group with one-or-more labels, all sharing the same source file.
    ///
    /// # Example
    ///
    /// ```text
    ///       ╭─[std/array.lm:29:46]
    ///    28 │    /// When creating an array with a set capacity, it's length will still be zero.
    ///    29 │    pub fn with_capacity(capacity: UInt64) -> Array<T> {
    ///       │                                              ^^^^^^^^ expected type `Array<T>` found here
    ///       ∶
    ///    34 │
    ///    35 │        return true;
    ///       │        ^^^^^^^^^^^^ expected `Array<T>`, found `Boolean`
    ///       ╰──
    /// ```
    fn render_label_group(
        &self,
        f: &mut impl std::fmt::Write,
        group: LabelGroup,
        severity: Severity,
    ) -> std::fmt::Result {
        if group.labels.is_empty() {
            return Ok(());
        }

        // We're assuming the first label is the "most important one", for no
        // reason in particular, but it seems the most intuitive.
        let first_label = group.labels.first().unwrap();

        let source = group.source;
        let source_name = source.name();
        let source_content = source.content();
        let gutter_size = self.gutter_size_of(&source_content);

        // Render header for the label group.
        //
        //    ╭─[std/array.lm:35:8]
        //
        let (line_num, columns) = coords_of_span(&source_content, first_label.range().clone());
        self.render_snippet_header(f, source_name, gutter_size, line_num, columns.start)?;

        // Render all the labels in in the group, along with joiners in the vertical gutter.
        //
        //  28 │    /// When creating an array with a set capacity, it's length will still be zero.
        //  29 │    pub fn with_capacity(capacity: UInt64) -> Array<T> {
        //     │                                              ^^^^^^^^ expected type `Array<T>` found here
        //     ∶
        //  34 │
        //  35 │        return true;
        //     │        ^^^^^^^^^^^^ expected `Array<T>`, found `Boolean`
        for (index, label) in group.labels.iter().enumerate() {
            self.render_label(f, label, source.clone(), severity, gutter_size)?;

            // Unless we're at the last label, print a vertical break in the gutter.
            if index < group.labels.len() - 1 {
                self.render_snippet_break(f, gutter_size)?;
            }
        }

        // Render the footer of the label group.
        //
        //    ╰──
        //
        self.render_snippet_footer(f, gutter_size)
    }

    /// Renders a single label without any header or footer attached.
    ///
    /// ```text
    ///  28 │    /// When creating an array with a set capacity, it's length will still be zero.
    ///  29 │    pub fn with_capacity(capacity: UInt64) -> Array<T> {
    ///     │                                              ^^^^^^^^ expected type `Array<T>` found here
    /// ```
    fn render_label(
        &self,
        f: &mut impl std::fmt::Write,
        label: &Label,
        source: Arc<dyn Source>,
        severity: Severity,
        padding: usize,
    ) -> std::fmt::Result {
        // If the label has a severity defined, use that instead of the diagnostic
        // severity. If not, use the severity of the diagnostic.
        let severity = match label.severity() {
            Some(sev) => sev,
            None => severity,
        };

        let severity_style = self.theme.style.from_severity(severity);

        let source_content = source.content();
        let (label_line_num, columns) = coords_of_span(&source_content, label.range().clone());
        let first_line_num = label_line_num.saturating_sub(self.context_lines);

        let (snipped_content, center_line) =
            extract_with_context_offset(&source_content, label.range().clone(), self.context_lines);

        for (idx, snipped_line) in snipped_content.lines().enumerate() {
            let line_num = first_line_num + idx;

            let snippet_line = if self.highlight_source && line_num == center_line {
                self.style_substring(snipped_line, columns.clone(), severity_style)
            } else {
                Style::new().style(snipped_line.to_string())
            };

            self.render_snippet_line(f, padding, snippet_line, line_num + 1)?;

            if line_num == center_line {
                let marker_columns = if columns.end < columns.start {
                    columns.start..snipped_line.len()
                } else {
                    columns.clone()
                };

                self.render_line_marker(
                    f,
                    severity_style,
                    &marker_columns,
                    label.message(),
                    padding,
                )?;
            }
        }

        Ok(())
    }

    /// Renders the header of a source snippet.
    ///
    /// ```text
    //    ╭─[std/array.lm:35:8]
    /// ```
    fn render_snippet_header(
        &self,
        f: &mut impl std::fmt::Write,
        name: Option<&str>,
        padding: usize,
        line: usize,
        column: usize,
    ) -> std::fmt::Result {
        self.write_ident(f)?;

        write!(
            f,
            "{}{}{}",
            " ".repeat(padding),
            self.theme.arrows.top_left,
            self.theme.arrows.horizontal,
        )?;

        if let Some(name) = name {
            self.render_source_path(f, name, line + 1, column)
        } else {
            writeln!(
                f,
                "{}",
                std::iter::repeat_n(self.theme.arrows.horizontal, 10).collect::<String>()
            )
        }
    }

    /// Renders the gutter for a single line in a source snippet.
    ///
    /// ```text
    //    28 │
    /// ```
    fn render_snippet_gutter(
        &self,
        f: &mut impl std::fmt::Write,
        padding: usize,
        gutter: impl std::fmt::Display,
        bar: impl std::fmt::Display,
    ) -> std::fmt::Result {
        self.write_ident(f)?;

        write!(f, "{gutter:^padding$}{bar} ")
    }

    /// Renders an empty gutter for a single line in a source snippet.
    ///
    /// ```text
    //       │
    /// ```
    fn render_snippet_line_empty_gutter(
        &self,
        f: &mut impl std::fmt::Write,
        padding: usize,
    ) -> std::fmt::Result {
        self.render_snippet_gutter(f, padding, "", self.theme.arrows.vertical)
    }

    /// Renders the gutter for a single line in a source snippet.
    ///
    /// ```text
    //    28 │
    /// ```
    fn render_snippet_line_gutter(
        &self,
        f: &mut impl std::fmt::Write,
        padding: usize,
        line_num: usize,
    ) -> std::fmt::Result {
        self.render_snippet_gutter(
            f,
            padding,
            self.style(&line_num, self.theme.style.gutter),
            self.theme.arrows.vertical,
        )
    }

    /// Renders a single line in a source snippet.
    ///
    /// ```text
    //    28 │    /// When creating an array with a set capacity, it's length will still be zero.
    /// ```
    fn render_snippet_line(
        &self,
        f: &mut impl std::fmt::Write,
        padding: usize,
        line: impl std::fmt::Display,
        line_num: usize,
    ) -> std::fmt::Result {
        self.render_snippet_line_gutter(f, padding, line_num)?;

        writeln!(f, "{line}")
    }

    /// Renders a single vertical break in a source snippet.
    ///
    /// ```text
    //      ∶
    /// ```
    fn render_snippet_break(
        &self,
        f: &mut impl std::fmt::Write,
        padding: usize,
    ) -> std::fmt::Result {
        self.render_snippet_gutter(f, padding, "", self.theme.arrows.vertical_break)?;

        writeln!(f)
    }

    /// Renders the footer of a source snippet.
    ///
    /// ```text
    //    ╰──
    /// ```
    fn render_snippet_footer(
        &self,
        f: &mut impl std::fmt::Write,
        padding: usize,
    ) -> std::fmt::Result {
        self.write_ident(f)?;
        self.write_padding(f, padding)?;

        writeln!(
            f,
            "{}{}",
            self.theme.arrows.bottom_left,
            std::iter::repeat_n(self.theme.arrows.horizontal, 2).collect::<String>()
        )
    }

    /// Renders the path of the source file.
    ///
    /// ```text
    ///   std/array.lm:35:8
    /// ```
    fn render_source_path(
        &self,
        f: &mut impl std::fmt::Write,
        name: &str,
        line: usize,
        column: usize,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "[{}:{}:{}]",
            self.style(&name, self.theme.style.link),
            line,
            column + 1
        )
    }

    /// Renders the marker underneath a labeled line.
    ///
    /// ```text
    ///           ^^^^^^^^^^^^ expected `Array<T>`, found `Boolean`
    /// ```
    fn render_line_marker(
        &self,
        f: &mut impl std::fmt::Write,
        style: Style,
        columns: &Range<usize>,
        message: &str,
        padding: usize,
    ) -> std::fmt::Result {
        self.render_line_marker_arrows(f, style, columns, padding)?;

        writeln!(f, " {message}")
    }

    /// Renders the arrow markers underneath a labeled line.
    ///
    /// ```text
    ///           ^^^^^^^^^^^^
    /// ```
    fn render_line_marker_arrows(
        &self,
        f: &mut impl std::fmt::Write,
        style: Style,
        columns: &Range<usize>,
        padding: usize,
    ) -> std::fmt::Result {
        self.render_snippet_line_empty_gutter(f, padding)?;
        self.write_padding(f, columns.start)?;

        for _ in 0..columns.end - columns.start {
            write!(f, "{}", self.style(&self.theme.arrows.arrow_up, style))?;
        }

        Ok(())
    }

    /// Renders the footer of a diagnostic message.
    ///
    /// # Example
    ///
    /// ```text
    ///   help: doc comments are only allowed on definitions
    ///   help: you can use triple forward-slash to denote doc comments
    /// ```
    fn render_footer(
        &self,
        f: &mut impl std::fmt::Write,
        diagnostic: &dyn Diagnostic,
    ) -> std::fmt::Result {
        if let Some(help) = diagnostic.help() {
            for line in help {
                self.render_help(f, &line)?;
            }
        }

        Ok(())
    }

    /// Renders a single help message, which is attached to a diagnostic message.
    ///
    /// # Example
    ///
    /// Single line help message:
    /// ```text
    ///   help: did you mean 'invoke'?
    /// ```
    ///
    /// Multi-line help message:
    /// ```text
    ///   help: did you mean 'invoke'?
    ///         ... or perhaps 'invoke_all'?
    /// ```
    ///
    /// Optionally with a suggestion attached:
    /// ```text
    ///    help: consider removing these parenthesis
    ///  34 │         return (0..10);
    ///     |                ^     ^
    /// ```
    fn render_help(&self, f: &mut impl std::fmt::Write, help: &Help) -> std::fmt::Result {
        let help_gutter = "   help: ";
        let help_padding = help_gutter.to_string().len();

        // If the help message has multiple lines, we need to indent the other lines
        // with the same padding, so it lines up correctly.
        //
        // So, instead of
        // ```text
        //   help: expected type `Array<T>`
        // found type `Boolean`
        // ```
        //
        // we would print:
        // ```text
        //   help: expected type `Array<T>`
        //         found type `Boolean`
        // ```
        for (i, line) in help.message.lines().enumerate() {
            self.write_ident(f)?;

            if i == 0 {
                writeln!(
                    f,
                    "{}{}",
                    self.style(&help_gutter, self.theme.style.help),
                    line
                )?;
            } else {
                writeln!(f, "{}{}", " ".repeat(help_padding), line)?;
            }
        }

        let mut padding = 0;
        let mut suggestion_groups: IndexMap<Option<String>, Vec<Suggestion>> = IndexMap::new();

        for suggestion in &help.suggestions {
            let source = suggestion.source();
            let source_name = source.name().map(|n| n.to_string());
            let source_content = source.content();

            padding = padding.max(self.gutter_size_of(&source_content));

            if let Some(group) = suggestion_groups.get_mut(&source_name) {
                group.push(suggestion.clone());
            } else {
                suggestion_groups.insert(source_name, vec![suggestion.clone()]);
            }
        }

        for (_, suggestions) in suggestion_groups {
            self.render_suggestion_group(f, &suggestions, padding)?;
        }

        Ok(())
    }

    /// Renders a group of suggestions defined within a help message, where
    /// all suggestions share the same source file.
    ///
    /// # Example
    ///
    /// ```text
    ///    help: consider removing these parenthesis
    ///  24 │         return (0..10);
    ///     |                ^     ^
    //      ∶
    ///  39 │         return (start..end);
    ///     |                ^          ^
    /// ```
    fn render_suggestion_group(
        &self,
        f: &mut impl std::fmt::Write,
        suggestions: &[Suggestion],
        padding: usize,
    ) -> std::fmt::Result {
        if suggestions.is_empty() {
            return Ok(());
        }

        let first_suggestion = suggestions.first().unwrap().clone();
        let source = first_suggestion.source();
        let source_content = source.content();

        let mut suggested_lines: IndexMap<usize, Vec<Suggestion>> = IndexMap::new();

        for suggestion in suggestions {
            let start_idx = match &suggestion {
                Suggestion::Insertion { location, .. } => location.offset,
                Suggestion::Deletion { range } => range.span.0.start,
                Suggestion::Replacement { range, .. } => range.span.0.start,
            };

            let (line, _) = coords_of_idx(&source_content, start_idx);

            if let Some(group) = suggested_lines.get_mut(&line) {
                group.push(suggestion.clone());
            } else {
                suggested_lines.insert(line, vec![suggestion.clone()]);
            }
        }

        let suggestion_len = suggested_lines.len();

        for (index, (line, suggestions)) in suggested_lines.into_iter().enumerate() {
            self.render_suggestion_line(f, line, suggestions)?;

            // Unless we're at the last suggestion, print a vertical break in the gutter.
            if index < suggestion_len - 1 {
                self.render_snippet_break(f, padding)?;
            }
        }

        Ok(())
    }

    /// Renders a single line where one-or-more suggestions are defined.
    ///
    /// # Example
    ///
    /// ```text
    ///    help: consider removing these parenthesis
    ///  24 │         return (0..10);
    ///     |                ^     ^
    fn render_suggestion_line(
        &self,
        f: &mut impl std::fmt::Write,
        line_num: usize,
        mut suggestions: Vec<Suggestion>,
    ) -> std::fmt::Result {
        if suggestions.is_empty() {
            return Ok(());
        }

        // Sort all suggestions, so earlier suggestions come first in the vector.
        suggestions.sort();

        // Since styling alters the content of the line, we need to
        // style the line with each suggestion in reverse order, so it
        // has no effect on previous suggestions on the same line.
        suggestions.reverse();

        let first_suggestion = suggestions.first().unwrap();

        let source = first_suggestion.source();
        let source_content = source.content();
        let source_line = extract_with_context(&source_content, first_suggestion.span(), 0);
        let padding = self.gutter_size_of(&source_content);

        // Render the suggestion itself.
        //
        //  24 │         return (0..10);
        //

        let mut styled_line = Box::new(source_line) as Box<dyn std::fmt::Display>;

        for suggestion in &suggestions {
            let span = suggestion.span();
            let (_, columns) = coords_of_span(&source_content, span);

            styled_line = self.style_suggestion_line(suggestion, styled_line, columns);
        }

        self.render_snippet_line(f, padding, styled_line, line_num + 1)?;

        // Render the arrows below the suggestions
        //
        //     |                ^     ^
        //

        self.render_snippet_gutter(f, padding, "", self.theme.arrows.vertical)?;

        // Un-reverse the suggestions again, so we can draw the arrows
        // below the marked sections of the suggestions.
        suggestions.reverse();

        let mut offset = 0;
        for suggestion in &suggestions {
            let span = suggestion.span();
            let (_, columns) = coords_of_span(&source_content, span);

            // Write the padding between the arrows.
            let spacing = columns.start.checked_sub(offset).unwrap_or_default();

            write!(f, "{}", " ".repeat(spacing))?;

            let style = match suggestion {
                Suggestion::Insertion { .. } => self.theme.style.insertion,
                Suggestion::Replacement { .. } => self.theme.style.insertion,
                Suggestion::Deletion { .. } => self.theme.style.deletion,
            };

            let arrow_count = match suggestion {
                Suggestion::Insertion { value, .. } => value.len(),
                Suggestion::Replacement { replacement, .. } => replacement.len(),
                Suggestion::Deletion { range } => range.span.0.len(),
            };

            for _ in 0..arrow_count {
                write!(f, "{}", self.style(&self.theme.arrows.arrow_up, style))?;
            }

            offset = columns.end;
        }

        writeln!(f)
    }

    /// Styles a single suggestion into a "fixed" line.
    fn style_suggestion_line<'a>(
        &self,
        suggestion: &Suggestion,
        line: Box<dyn std::fmt::Display + 'a>,
        span: Range<usize>,
    ) -> Box<dyn std::fmt::Display + 'a> {
        let line = line.to_string();

        let formatted = match suggestion {
            Suggestion::Deletion { .. } => {
                let [before, middle, after] = split_str_at(&line, vec![span.start, span.end]);

                format!(
                    "{}{}{}",
                    before,
                    self.style(&middle, self.theme.style.deletion),
                    after
                )
            }
            Suggestion::Insertion { value, .. } => {
                let [before, middle, after] = split_str_at(&line, vec![span.start, span.end]);

                format!(
                    "{}{}{}{}",
                    before,
                    self.style(&value, self.theme.style.insertion),
                    middle,
                    after
                )
            }
            Suggestion::Replacement { replacement, range } => {
                let length = range.span.0.len();
                let [before, _, after] = split_str_at(&line, vec![span.start, span.start + length]);

                format!(
                    "{}{}{}",
                    before,
                    self.style(&replacement, self.theme.style.insertion),
                    after
                )
            }
        };

        Box::new(formatted) as Box<dyn std::fmt::Display>
    }
}

/// Gets the width of the current terminal window.
///
/// If the `termsize` feature is enabled, the width of the terminal is determined at runtime
/// using `termios` on macOS/Linux and Win32 for Windows.
///
/// If the `termsize` is not enabled, the default terminal width is returned (defaults to `80`).
fn terminal_width() -> usize {
    #[cfg(feature = "termsize")]
    if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
        w as usize
    } else {
        DEFAULT_TERM_WIDTH
    }

    #[cfg(not(feature = "termsize"))]
    DEFAULT_TERM_WIDTH
}

/// Splits the given string into `N` slices, where each index defines
/// where the source string should be split.
fn split_str_at<const N: usize>(str: &str, mut indices: Vec<usize>) -> [&str; N] {
    indices.sort();
    indices.reverse();

    let mut current = str;
    let mut slices = [""; N];

    for (i, index) in indices.iter().enumerate() {
        let (before, after) = current.split_at(*index);

        current = before;
        slices[N - i - 1] = after;
    }

    slices[0] = current;

    slices
}

/// Gets the line number and column indices which contains the given span.
fn coords_of_span(str: &str, span: impl Into<Range<usize>>) -> (usize, Range<usize>) {
    let span: Range<usize> = span.into();

    let (line, start) = coords_of_idx(str, span.start);
    let (_, end) = coords_of_idx(str, span.end);

    (line, start..end)
}

/// Gets the line number and column number which contains the character at the given index.
fn coords_of_idx(str: &str, index: usize) -> (usize, usize) {
    if index > str.len() {
        return (0, 0);
    }

    let mut line = 0;
    let mut column = 0;

    for (i, c) in str.chars().peekable().enumerate() {
        if i == index {
            return (line, column);
        }

        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }

    if index == str.len() {
        return (line, column);
    }

    (0, 0)
}

#[cfg(test)]
mod coords_of_idx_tests {
    use super::coords_of_idx;

    #[test]
    fn test_index_out_of_range() {
        let source = "let a = 1;";
        let (line, column) = coords_of_idx(source, 12);

        assert_eq!(line, 0);
        assert_eq!(column, 0);
    }

    #[test]
    fn test_index_at_end_boundary() {
        let source = "let a = 1;";
        let (line, column) = coords_of_idx(source, 10);

        assert_eq!(line, 0);
        assert_eq!(column, 10);
    }

    #[test]
    fn test_multiline() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (line, column) = coords_of_idx(source, 26);

        assert_eq!(line, 2);
        assert_eq!(column, 4);
    }

    #[test]
    fn test_multiline_line_boundary_start() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (line, column) = coords_of_idx(source, 22);

        assert_eq!(line, 2);
        assert_eq!(column, 0);
    }

    #[test]
    fn test_multiline_line_boundary_end() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (line, column) = coords_of_idx(source, 36);

        assert_eq!(line, 2);
        assert_eq!(column, 14);
    }
}

/// Extracts a slice of the given string, which contains the lines where
/// `span` is contained, along with the `context_lines` amount of surrounding lines.
///
/// # Example
///
/// ```
/// use error_snippet::render::graphical::extract_with_context;
///
/// let source = r#"let a = 1;
/// let b = 2;
/// let c = a + b;
/// let d = c * 2;
/// let e = (d + 3) * 2;"#;
///
/// // indexes "a + b" on line 3
/// let span = 30..35;
///
/// let snipped = extract_with_context(source, span, 1);
///
/// assert_eq!(snipped, r#"let b = 2;
/// let c = a + b;
/// let d = c * 2;"#);
/// ```
pub fn extract_with_context(
    input: &str,
    range: impl Into<Range<usize>>,
    context_lines: usize,
) -> &str {
    let (slice, _) = extract_with_context_offset(input, range, context_lines);

    slice
}

/// Extracts a slice of the given string, which contains the lines where
/// `span` is contained, along with the `context_lines` amount of surrounding lines.
///
/// The function also returns the line number where the "center" of the context starts.
///
/// # Example
///
/// ```
/// use error_snippet::render::graphical::extract_with_context;
///
/// let source = r#"let a = 1;
/// let b = 2;
/// let c = a + b;
/// let d = c * 2;
/// let e = (d + 3) * 2;"#;
///
/// // indexes "a + b" on line 3
/// let span = 30..35;
///
/// let snipped = extract_with_context(source, span, 1);
///
/// assert_eq!(snipped, r#"let b = 2;
/// let c = a + b;
/// let d = c * 2;"#);
/// ```
pub fn extract_with_context_offset(
    input: &str,
    range: impl Into<Range<usize>>,
    context_lines: usize,
) -> (&str, usize) {
    let range: Range<usize> = range.into();

    let mut line_start = 0;
    let mut line_spans = Vec::new();

    for line in input.lines() {
        let line_len = line.len();
        let span = line_start..(line_start + line_len);

        line_spans.push(span);

        // +1 for '\n' (assuming UNIX-style newlines)
        line_start += line_len + 1;
    }

    // Determine the lines that intersect with the byte range
    let mut matching_lines = Vec::new();
    for (i, span) in line_spans.iter().enumerate() {
        if span.end > range.start && span.start < range.end {
            matching_lines.push(i);
        }
    }

    if matching_lines.is_empty() {
        return ("", 0);
    }

    let first_matching_line = *matching_lines.first().unwrap();

    let first_match = first_matching_line.saturating_sub(context_lines);
    let last_match = (matching_lines.last().unwrap() + context_lines).min(line_spans.len() - 1);

    let start_byte = line_spans[first_match].start;
    let end_byte = line_spans[last_match].end;

    (&input[start_byte..end_byte], first_matching_line)
}

#[cfg(test)]
mod extract_with_context_offset_tests {
    use super::extract_with_context_offset;

    #[test]
    fn test_extract_with_context() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 30..35, 1);

        assert_eq!(snipped, "let b = 2;\nlet c = a + b;\nlet d = c * 2;");
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_extract_without_context() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 30..35, 0);

        assert_eq!(snipped, "let c = a + b;");
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_extract_at_beginning_boundary() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 0..10, 2);

        assert_eq!(snipped, "let a = 1;\nlet b = 2;\nlet c = a + b;");
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_extract_at_ending_boundary() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 60..71, 2);

        assert_eq!(
            snipped,
            "let c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;"
        );
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_extract_at_line_start() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 22..36, 1);

        assert_eq!(snipped, "let b = 2;\nlet c = a + b;\nlet d = c * 2;");
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_extract_first_line() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 4..9, 1);

        assert_eq!(snipped, "let a = 1;\nlet b = 2;");
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_extract_last_line() {
        let source = "let a = 1;\nlet b = 2;\nlet c = a + b;\nlet d = c * 2;\nlet e = (d + 3) * 2;";
        let (snipped, offset) = extract_with_context_offset(source, 64..75, 1);

        assert_eq!(snipped, "let d = c * 2;\nlet e = (d + 3) * 2;");
        assert_eq!(offset, 4);
    }
}
