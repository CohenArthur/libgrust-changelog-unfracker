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
    path::PathBuf,
    process::{Command, Stdio},
};

struct ChangelogLine {
    file: PathBuf,
    message: String,
}
// format as `\t* <file>: <message>\n`

struct ChangelogEntry {
    file: String,              // PathBuf?
    lines: Vec<ChangelogLine>, // NonEmptyVec?
}
// format as `<file>:\n <lines>`

// first line of a commit
struct Title(String);

// text between the title and the Changelog
struct Body(String);

// Signed-off-by line - optional
struct SoB(String);
// format as `<0>`

struct Commit {
    title: Title,
    body: Body,
    changelog_entries: Vec<ChangelogEntry>, // but cull away the empty ChangelogEntries,
    sob: Option<SoB>,
}

impl Commit {
    /// This does NOT return a Result - if this fails, our tool is in the wrong and should panic.
    fn parse(input: String) -> Commit {
        // title is until two newlines
        // then message, until two newlines
        // then Changelog entries

        let blocks: Vec<&str> = input.split("\n\n").collect();

        dbg!(blocks);

        todo!()
    }
}

fn main() {
    // TODO: Should we take the commit's body message as argument?

    let input = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%B")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // FIXME: No unwrap

    let commit = Commit::parse(String::from_utf8(input.stdout).unwrap());
}
