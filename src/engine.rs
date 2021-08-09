use regex::Regex;

const REGEX_SPECIALS: &str = r"^$.,{}\<>?*:()[]-+";

struct Compiler {
    expr: Vec<char>,
    pos: usize,
    readpos: usize,
    ch: char,
}

impl Compiler {
    fn new(s: impl AsRef<str>) -> Self {
        let expr: Vec<_> = s.as_ref().chars().collect();

        let ch = expr[0];

        Self {
            expr,
            pos: 0,
            readpos: 1,
            ch,
        }
    }

    fn read(&mut self) {
        if self.readpos >= self.expr.len() {
            self.ch = char::REPLACEMENT_CHARACTER;
        } else {
            self.ch = self.expr[self.readpos];
        }
        self.pos = self.readpos;
        self.readpos += 1;
    }

    fn ahead_is(&self, s: &str) -> bool {
        self.expr
            .get(self.readpos..self.readpos + s.len())
            .map(|i| i.iter().copied().collect::<String>().eq(s))
            .unwrap_or(false)
    }
}

impl Compiler {
    fn compile(mut self) -> String {
        let mut buf = "(?i)^".to_string();
        while self.ch != char::REPLACEMENT_CHARACTER {
            match self.ch {
                '*' => buf.push_str(".*?"),
                '?' => buf.push_str(".?"),
                c if c.is_ascii_whitespace() => buf.push_str("\\s"),
                '@' if self.ahead_is("N@") => {
                    self.read();
                    self.read();
                    buf.push_str("(\\d+)");
                }
                '\\' => {
                    self.read();
                    buf.push('\\');
                    if self.ch != char::REPLACEMENT_CHARACTER {
                        buf.push(self.ch);
                    }
                }
                c if REGEX_SPECIALS.contains(c) => {
                    buf.push('\\');
                    buf.push(c);
                }
                c => buf.push(c),
            };
            self.read();
        }

        buf.push('$');

        buf
    }
}

pub fn compile(s: impl AsRef<str>) -> Result<Regex, regex::Error> {
    let c = Compiler::new(s.as_ref());

    Regex::new(&c.compile())
}

#[cfg(test)]
mod tests {
    use super::Compiler;
    #[test]
    fn compile() {
        let tests = [
            ("lmao *.txt", r"lmao\s.*?\.txt"),
            ("$1=2?", r"\$1=2.?"),
            (
                "lmao season 1 episode @N@.mp4",
                r"lmao\sseason\s1\sepisode\s(\d+)\.mp4",
            ),
        ];

        for test in tests {
            let got = Compiler::new(test.0).compile();
            let expected = format!("(?i)^{}$", test.1);
            assert_eq!(expected, got,);
        }
    }
}
