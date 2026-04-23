use imara_diff::{
    Algorithm, Diff, InternedInput, Interner, Token, UnifiedDiffConfig, UnifiedDiffPrinter,
};

struct ColoredLineDiffPrinter<'a> {
    interner: &'a Interner<&'a str>,
    use_color: bool,
}

impl<'a> ColoredLineDiffPrinter<'a> {
    const RESET: &'static str = "\x1b[0m";
    const RED: &'static str = "\x1b[31m";
    const GREEN: &'static str = "\x1b[32m";
    const CYAN: &'static str = "\x1b[36m";

    fn write_header(
        &self,
        mut f: impl std::fmt::Write,
        start_before: u32,
        start_after: u32,
        len_before: u32,
        len_after: u32,
    ) -> std::fmt::Result {
        if self.use_color {
            write!(
                f,
                "{}@@ -{},{} +{},{} @@\n{}",
                Self::CYAN,
                start_before + 1,
                len_before,
                start_after + 1,
                len_after,
                Self::RESET
            )
        } else {
            write!(f, "@@ -{},{} +{},{} @@\n", start_before, len_before, start_after, len_after)
        }
    }

    fn write_hunk_line(
        &self,
        mut f: impl std::fmt::Write,
        style: &'static str,
        marker: char,
        token: Token,
    ) -> std::fmt::Result {
        let line = self.interner[token];
        if self.use_color {
            write!(f, "{}{}{}{}", style, marker, line, Self::RESET)?;
        } else {
            write!(f, "{}{}", marker, line)?;
        }

        if !line.ends_with('\n') {
            writeln!(f)?;
        }

        Ok(())
    }
}

impl UnifiedDiffPrinter for ColoredLineDiffPrinter<'_> {
    fn display_header(
        &self,
        f: impl std::fmt::Write,
        start_before: u32,
        start_after: u32,
        len_before: u32,
        len_after: u32,
    ) -> std::fmt::Result {
        self.write_header(f, start_before, start_after, len_before, len_after)
    }

    fn display_context_token(
        &self,
        mut f: impl std::fmt::Write,
        token: Token,
    ) -> std::fmt::Result {
        let line = self.interner[token];
        write!(f, " {}", line)?;
        if !line.ends_with('\n') {
            writeln!(f)?;
        }
        Ok(())
    }

    fn display_hunk(
        &self,
        mut f: impl std::fmt::Write,
        before: &[Token],
        after: &[Token],
    ) -> std::fmt::Result {
        for token in before {
            self.write_hunk_line(&mut f, Self::RED, '-', *token)?;
        }

        for token in after {
            self.write_hunk_line(&mut f, Self::GREEN, '+', *token)?;
        }

        Ok(())
    }
}

pub fn run(file1: String, file2: String) {
    println!("Running diff command...");

    let before = std::fs::read_to_string(&file1).expect("Failed to read before.txt");
    let after = std::fs::read_to_string(&file2).expect("Failed to read after.txt");

    let input = InternedInput::new(before.as_str(), after.as_str());
    let mut diff = Diff::compute(Algorithm::Histogram, &input);
    diff.postprocess_lines(&input);

    let printer = ColoredLineDiffPrinter {
        interner: &input.interner,
        use_color: std::env::var_os("NO_COLOR").is_none(),
    };
    let rendered = diff
        .unified_diff(&printer, UnifiedDiffConfig::default(), &input)
        .to_string();

    println!("Diff:\n{}", rendered);

}