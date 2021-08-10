#[cfg(test)]
mod tests;

use std::{
    error::Error,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use once_cell::unsync::OnceCell;

use crate::{
    app,
    engine,
};

struct Renamee<'a> {
    path: &'a Path,
    old: String,
    indices: Vec<(usize, usize)>,
    new: OnceCell<String>,
    new_path: OnceCell<PathBuf>,
}

impl<'a> Renamee<'a> {
    fn new(path: &'a Path, old: String, indices: Vec<(usize, usize)>) -> Self {
        Self {
            path,
            old,
            indices,
            new: OnceCell::new(),
            new_path: OnceCell::new(),
        }
    }

    fn new_name(&self, max_digits: &[usize]) -> &str {
        self.new
            .get_or_init(|| self.calc_new_name(max_digits))
            .as_str()
    }

    fn calc_new_name(&self, max_digits: &[usize]) -> String {
        let mut pieces = Vec::new();
        for (i, (start, end)) in self.indices.iter().enumerate() {
            let start = *start;
            let end = *end;
            if i == 0 {
                pieces.push(&self.old[..start]);
                if self.indices.len() == 1 {
                    pieces.push(&self.old[end..]);
                    break;
                }
                continue;
            }

            let prev_end = self.indices[i - 1].1;
            pieces.push(&self.old[prev_end..start]);

            if i + 1 == self.indices.len() {
                pieces.push(&self.old[end..]);
            }
        }

        let mut new_nums = self
            .indices
            .iter()
            .map(|(start, end)| self.old[*start..*end].parse::<usize>().unwrap())
            .enumerate()
            .map(|(i, n)| format!("{num:0>width$}", num = n, width = max_digits[i]))
            .collect::<Vec<_>>();

        assert_eq!(pieces.len(), new_nums.len() + 1, "spliced incorrectly");

        let mut buf = pieces[0].to_string();

        for s in &pieces[1..] {
            buf.push_str(&new_nums.remove(0));
            buf.push_str(s);
        }

        buf
    }

    fn new_path(&self, max_digits: &[usize]) -> &Path {
        self.new_path
            .get_or_init(|| {
                let name = self.new_name(max_digits);
                let mut p = self.path.to_path_buf();
                p.set_file_name(name);
                p
            })
            .as_path()
    }
}

pub struct Cmd {
    pattern: String,
    files: Vec<PathBuf>,
    interactive: bool,
    err_abort: bool,
    force: bool,
}

impl Cmd {
    pub fn from_args() -> Self {
        let m = app::new().get_matches();

        let pattern = m.value_of("pattern").unwrap().to_string();
        let force = m.is_present("force");
        let err_abort = m.is_present("err-abort");

        #[cfg(not(windows))]
        let files: Vec<_> = m.values_of("file").unwrap().map(PathBuf::from).collect();
        #[cfg(windows)]
        let files = crate::get_files(&m);

        let interactive = m.is_present("interactive");

        Self {
            pattern,
            files,
            interactive,
            err_abort,
            force,
        }
    }
}

impl Cmd {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let re = engine::compile(&self.pattern)?;

        let items: Vec<_> = self
            .files
            .iter()
            .map(|p| -> Result<Renamee<'_>, String> {
                let name = p
                    .file_name()
                    .map(|s| s.to_os_string())
                    .ok_or_else(|| format!("{} has no file name", p.display()))?
                    .into_string()
                    .map_err(|_| format!("{} is not a valid utf-8 path", p.display()))?;

                let indices = re
                    .captures(&name)
                    .ok_or_else(|| {
                        format!("the file {} does not match the provided pattern", &name)
                    })?
                    .iter()
                    .skip(1)
                    .filter_map(|o| o.map(|c| (c.start(), c.end())))
                    .collect::<Vec<_>>();

                Ok(Renamee::new(p, name, indices))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut max_digits = vec![0_usize; re.captures_len()];

        for r in &items {
            for (i, (start, end)) in r.indices.iter().enumerate() {
                let s = &r.old[*start..*end];
                let s = s.trim_start_matches('0');
                if s.len() > max_digits[i] {
                    max_digits[i] = s.len();
                }
            }
        }

        if !self.force {
            let n_collisions = items
                .iter()
                .filter(|r| {
                    let np = r.new_path(&max_digits);
                    if !np.eq(r.path) && np.exists() {
                        println!(
                            "rename collision: can't rename {} to {}, path already exists",
                            r.path.display(),
                            np.display()
                        );
                        true
                    } else {
                        false
                    }
                })
                .count();

            if n_collisions > 0 {
                return Err(format!("can't perform operation due to {} rename collisions; pass --force to overwrite", n_collisions).into());
            }
        }

        for r in &items {
            let new_path = r.new_path(&max_digits);
            if new_path.eq(r.path) {
                continue;
            }

            if self.interactive
                && !crate::confirm(&format!(
                    "rename {} to {}?",
                    r.path.display(),
                    new_path.display()
                ))
            {
                continue;
            }

            if new_path.exists() {
                eprintln!("warning: overwriting {}", new_path.display());
            }

            if let Err(e) = fs::rename(r.path, &new_path) {
                if self.err_abort {
                    return Err(Box::new(e));
                }
                eprintln!(
                    "error renaming  {} to {}: {}",
                    r.path.display(),
                    new_path.display(),
                    e
                );
            }
        }

        Ok(())
    }
}
