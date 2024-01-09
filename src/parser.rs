use std::{path::PathBuf, str::Split};

use crate::{Body, ChangelogEntry, ChangelogLine, Commit, SoB, Title};

// FIXME: we can have more than one line per Changelog "line", e.g
// ```
// * libgrust/libproc_macro/file.cc:
//   (LibProcMacro::SomeClass): New class.
//   (LibProcMacro::some_function): New function.
// ```
// and the message is 3 lines long
fn changelog_line(line: &str) -> ChangelogLine {
    // \t* <file>: <msg>
    let line = line.trim_start_matches(|c| c == '\t' || c == '*' || c == ' ');

    let (file, message) = line.split_once(':').unwrap();

    let file = PathBuf::from(file);
    let message = message.trim_start().to_string();

    ChangelogLine { file, message }
}

fn maybe_changelog_entry(blocks: &mut Split<'_, &str>) -> Option<ChangelogEntry> {
    // this consumes the SoB block if there is one, so that's not great. we need to do look-ahead
    // in `changelog_entries` I guess?
    let changelog = blocks.next();
    let entries = blocks.next();

    let is_changelog_entry = changelog.map(|s| !(s.is_empty() || s.starts_with("Signed-off-by")));

    is_changelog_entry.and_then(|b| {
        b.then(|| ChangelogEntry {
            file: changelog.unwrap().split_once(':').unwrap().0.to_string(),
            lines: entries.unwrap().lines().map(changelog_line).collect(),
        })
    })
}

fn changelog_entries(blocks: &mut Split<'_, &str>) -> Vec<ChangelogEntry> {
    let mut v = Vec::new();

    while let Some(entry) = maybe_changelog_entry(blocks) {
        v.push(entry)
    }

    v
}

/// This does NOT return a Result - if this fails, our tool is in the wrong and should panic.
pub fn commit(input: String) -> Commit {
    // title is until two newlines
    // then message, until two newlines
    // then Changelog entries

    // we split the commit message in blocks separated by two newlines.
    // the first one is the title, the second one is the commit message, and then are the Changelog entries
    let mut blocks = input.split("\n\n");

    // there's always a title, otherwise we panic
    let title = blocks.next().map(|s| Title(String::from(s))).unwrap();

    // there might not be a commit message
    let body = blocks.next().map(|s| Body(String::from(s)));

    let changelog_entries = changelog_entries(&mut blocks);

    // there might not be a SoB line
    let sob = blocks.next().map(|s| SoB(String::from(s)));

    Commit {
        title,
        body,
        changelog_entries,
        sob,
    }
}
