use std::fmt::{Display, Formatter, Result};

use crate::{ChangelogEntry, ChangelogLine, Commit, Title};

impl Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ChangelogLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "\t* {}: {}", self.file.display(), self.message)
    }
}

impl Display for ChangelogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}: \n{}\n",
            self.file,
            self.lines
                .iter()
                .fold(String::new(), |acc, line| format!("{acc}\n{line}"))
        )
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        dbg!(self);
        write!(
            f,
            "{}{}{}{}",
            self.title,
            self.body
                .as_ref()
                .map(|b| format!("\n\n{}", b.0))
                .unwrap_or_default(),
            self.changelog_entries
                .iter()
                .fold(String::new(), |acc, entry| format!("{acc}\n\n{entry}")),
            self.sob
                .as_ref()
                .map(|b| format!("\n\n{}", b.0))
                .unwrap_or_default(),
        )
    }
}
