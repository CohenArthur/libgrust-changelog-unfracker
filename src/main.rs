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

use std::{
    collections::HashSet,
    env,
    path::PathBuf,
    process::{Command, Stdio},
    str::Split,
};

use clap::Parser;

#[derive(Debug)]
struct ChangelogLine {
    file: PathBuf,
    message: String,
}
// format as `\t* <file>: <message>\n`

#[derive(Debug)]
struct ChangelogEntry {
    file: String,              // PathBuf?
    lines: Vec<ChangelogLine>, // NonEmptyVec?
}
// format as `<file>:\n <lines>`

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
struct Commit {
    title: Title,
    body: Option<Body>,
    changelog_entries: Vec<ChangelogEntry>, // but cull away the empty ChangelogEntries,
    sob: Option<SoB>,
}

impl Commit {
    fn parse_changelog_line(line: &str) -> ChangelogLine {
        // \t* <file>: <msg>
        let line = line.trim_start_matches(|c| c == '\t' || c == '*' || c == ' ');

        let (file, message) = line.split_once(':').unwrap();

        let file = PathBuf::from(file);
        let message = message.trim_start().to_string();

        ChangelogLine { file, message }
    }

    fn maybe_parse_changelog_entry(blocks: &mut Split<'_, &str>) -> Option<ChangelogEntry> {
        let changelog = blocks.next();
        let entries = blocks.next();

        let is_changelog_entry =
            changelog.map(|s| !(s.is_empty() || s.starts_with("Signed-off-by")));

        dbg!(changelog, entries, &is_changelog_entry);

        is_changelog_entry.and_then(|b| {
            b.then(|| ChangelogEntry {
                file: changelog.unwrap().split_once(':').unwrap().0.to_string(),
                lines: entries
                    .unwrap()
                    .lines()
                    .map(Commit::parse_changelog_line)
                    .collect(),
            })
        })
    }

    fn parse_changelog_entries(blocks: &mut Split<'_, &str>) -> Vec<ChangelogEntry> {
        let mut v = Vec::new();

        while let Some(entry) = Commit::maybe_parse_changelog_entry(blocks) {
            v.push(entry)
        }

        v
    }

    /// This does NOT return a Result - if this fails, our tool is in the wrong and should panic.
    fn parse(input: String) -> Commit {
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

        let entry = Commit::parse_changelog_entries(&mut blocks);

        dbg!(title, body, entry);
        dbg!(blocks);

        todo!()
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
    let commit = Commit::parse(args.input);
}
