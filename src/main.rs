//! Unfrack the changelogs of commits touching libgrust which did not yet use the proper `libgrust/Changelog` file and instead used the root `Changelog` one.

// This tool analyzes the Changelog of the latest commit, fixes it, and amends the commit with its new Changelog
// It is meant to be run as part of a `git rebase`, for example using the `-x` argument of `git rebase`.
// `git rebase gcc-patch-dev -x ./libgrust-changelog-unfracker`
// expected usage: ./libgrust-changelog-unfracker

// example bad commit:
// commit edb31d0589cfa46ab9563d217b7625d7feede5b0
// Author: Pierre-Emmanuel Patry <pierre-emmanuel.patry@embecosm.com>
// Date:   Wed Feb 22 17:14:24 2023 +0100
//
//     librust: Add libproc_macro and build system
//
//     Add some dummy files in libproc_macro along with it's build system.
//
//     ChangeLog:
//
//             * libgrust/Makefile.am: New file.
//             * libgrust/configure.ac: New file.
//             * libgrust/libproc_macro/Makefile.am: New file.
//             * libgrust/libproc_macro/proc_macro.cc: New file.
//             * libgrust/libproc_macro/proc_macro.h: New file.
//
//     Signed-off-by: Pierre-Emmanuel Patry <pierre-emmanuel.patry@embecosm.com>

// expected output:
// commit edb31d0589cfa46ab9563d217b7625d7feede5b0
// Author: Pierre-Emmanuel Patry <pierre-emmanuel.patry@embecosm.com>
// Date:   Wed Feb 22 17:14:24 2023 +0100
//
//     librust: Add libproc_macro and build system
//
//     Add some dummy files in libproc_macro along with it's build system.
//
//     libgrust/ChangeLog:
//
//             * Makefile.am: New file.
//             * configure.ac: New file.
//             * libproc_macro/Makefile.am: New file.
//             * libproc_macro/proc_macro.cc: New file.
//             * libproc_macro/proc_macro.h: New file.
//
//     Signed-off-by: Pierre-Emmanuel Patry <pierre-emmanuel.patry@embecosm.com>

// ## Pseudo code:
// find lines which concern libgrust -> so first parse the commit as a commit entry as outlined later
// remove them
// treat them
// if all the lines in `Changelog` were removed, remove the entire entry
// append them as `libgrust/Changelog` entries
// fix Signed-off-by line if necessary

// this gives us an idea of the architecture we want for a commit:
// {Changelog -> [lines]}
// which then becomes
// {Changelog -> [lines], libgrust/Changelog -> [lines]}

// if `lines` is empty in a "changelog entry" then we remove the entire entry

use std::path::PathBuf;

use clap::Parser;

mod format;
mod parser;

#[derive(Debug)]
struct ChangelogLine {
    file: PathBuf,
    message: String,
}
// format as `\t* <file>: <message>\n`

impl ChangelogLine {
    fn is_libgrust_line(&self) -> bool {
        self.file.starts_with("libgrust/") || self.file.starts_with("librust")
    }

    fn into_libgrust_line(self) -> ChangelogLine {
        let file = self.file.strip_prefix("libgrust/").unwrap().to_owned();

        ChangelogLine { file, ..self }
    }
}

#[derive(Debug)]
struct ChangelogEntry {
    file: String,              // PathBuf?
    lines: Vec<ChangelogLine>, // NonEmptyVec?
}
// format as `<file>:\n <lines>`

impl ChangelogEntry {
    fn split_in_libgrust_entry(self) -> (Option<ChangelogEntry>, Option<ChangelogEntry>) {
        // split the lines based on `is_libgrust_line`
        // if they are libgrust lines, fix them using `to_libgrust_line`
        // put the first part in the original entry
        // put the second part in a new entry

        let entry_or_none = |file, lines: Vec<ChangelogLine>| {
            if lines.is_empty() {
                None
            } else {
                Some(ChangelogEntry { file, lines })
            }
        };

        let (libgrust_lines, lines) = self
            .lines
            .into_iter()
            .partition(ChangelogLine::is_libgrust_line);

        (
            entry_or_none(self.file, lines),
            entry_or_none(
                String::from("libgrust/Changelog"),
                libgrust_lines
                    .into_iter()
                    .map(ChangelogLine::into_libgrust_line)
                    .collect(),
            ),
        )
    }
}

// first line of a commit
#[derive(Debug)]
struct Title(String);

// text between the title and the Changelog
#[derive(Debug)]
struct Body(String);

// Signed-off-by line - optional
#[derive(Debug)]
struct SoB(String);
// format as `<0>`

#[derive(Debug)]
pub struct Commit {
    title: Title,
    body: Option<Body>,
    changelog_entries: Vec<ChangelogEntry>, // but cull away the empty ChangelogEntries,
    sob: Option<SoB>,
}

impl Commit {
    fn unfrack_libgrust_entries(self) -> Commit {
        let changelog_entries = self
            .changelog_entries
            .into_iter()
            .map(|entry| entry.split_in_libgrust_entry())
            .fold(Vec::new(), |mut acc, (entry, maybe_new_entry)| {
                if let Some(e) = entry {
                    acc.push(e)
                }
                if let Some(e) = maybe_new_entry {
                    acc.push(e)
                }

                acc
            });

        Commit {
            changelog_entries,
            ..self
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(
        short,
        long,
        help = "Commit message to unfrack. Generate using `git log -1 <sha> --format=%B`"
    )]
    input: String,
}

fn main() {
    let args = Args::parse();
    let commit = parser::commit(args.input);
    let commit = commit.unfrack_libgrust_entries();

    println!("{}", commit);
}
