/*
 * Copyright (c) 2024 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use clap::load_yaml;
use clap::App;
use std::process::Command;
use ulid::Ulid;

fn main() {
    let yaml = load_yaml!("sq.yaml");
    let command = App::from_yaml(yaml);
    let command_name: String = command.get_name().into();
    command.get_matches();

    // Get current branch name, as described at
    // <https://stackoverflow.com/questions/6245570/how-do-i-get-the-current-branch-name-in-git/12142066#12142066>
    let outp = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Get current branch name");
    let out_s = String::from_utf8_lossy(&outp.stdout);
    let curr_branch = out_s.trim();
    eprintln!("Current branch: {curr_branch}");

    // TODO: Be able to work when remote name is not "origin".

    // Get default branch name, as described at
    // <https://stackoverflow.com/questions/28666357/how-to-get-default-git-branch/44750379#44750379>
    let outp = Command::new("git")
        .arg("symbolic-ref")
        .arg("refs/remotes/origin/HEAD")
        .output()
        .expect("Get default branch name");
    let out_s = String::from_utf8_lossy(&outp.stdout);
    let default_branch = out_s
        .trim()
        .strip_prefix("refs/remotes/origin/")
        .expect("branch name");
    eprintln!("Default branch: {default_branch}");

    // Get default branch name with remote prefix, as also described at
    // <https://stackoverflow.com/questions/28666357/how-to-get-default-git-branch/44750379#44750379>
    let outp = Command::new("git")
        .arg("symbolic-ref")
        .arg("refs/remotes/origin/HEAD")
        .arg("--short")
        .output()
        .expect("Get default branch name with remote prefix");
    let out_s = String::from_utf8_lossy(&outp.stdout);
    let default_branch_w_remote_prefix = out_s.trim();
    eprintln!("Default branch with remote prefix: {default_branch_w_remote_prefix}");

    // Ensure that all three are non-empty. Otherwise refuse to continue.
    if curr_branch.is_empty() {
        panic!("Name for current branch seems empty.");
    } else if default_branch.is_empty() {
        panic!("Name for default branch seems empty.");
    } else if default_branch_w_remote_prefix.is_empty() {
        panic!("Name for default branch with remote prefix seems empty.");
    }

    // If we are on the default branch currently then the operation we are trying to do is not applicable.
    if curr_branch == default_branch {
        panic!("This squash operation is not applicable on default branch.");
    }

    /*
     * Now for the squash operation, à la
     * <https://stackoverflow.com/questions/30136558/how-to-squash-commits-which-have-merge-commit-in-between/69827502#69827502>
     */

    // Generate a unique name for temporary branch
    let ulid = Ulid::new();
    let temp_branch = format!("temp-{ulid}");
    eprintln!("Temporary branch: {temp_branch}");

    // Squash op step 1: Create temporary branch
    let status = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(&temp_branch)
        .arg(default_branch_w_remote_prefix)
        .status()
        .expect("Create temporary branch");
    if !status.success() {
        panic!("Failed to create temporary branch");
    }

    // TODO: If any subsequent command fails after this point,
    //       go back to branch we were on and then delete the temporary branch.

    // Squash op step 2: Squash the branch
    let status = Command::new("git")
        .arg("merge")
        .arg("--squash")
        .arg(curr_branch)
        .status()
        .expect("Squash branch");
    if !status.success() {
        panic!("Failed to squash branch");
    }

    // Squash op step 3: Commit the changes (the commit message contains all squashed commit messages)
    let status = Command::new("git")
        .arg("commit")
        .arg("--no-edit")
        .status()
        .expect("Commit squash");
    if !status.success() {
        panic!("Failed to commit squash");
    }

    // Squash op step 4: Go back to the original branch and point it to the temp branch
    let status = Command::new("git")
        .arg("checkout")
        .arg(curr_branch)
        .status()
        .expect("Checkout original branch");
    if !status.success() {
        panic!("Failed to checkout original squash");
    }

    // Squash op step 5: Delete temporary branch
    let status = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(&temp_branch)
        .status()
        .expect("Delete temporary branch");
    if !status.success() {
        panic!("Failed to delete temporary branch");
    }

    // Finally, amend commit, resetting author and date, and letting user edit commit message.
    let err = exec::Command::new("git")
        .arg("commit")
        .arg("--amend")
        .arg("--reset-author")
        .arg("--date=now")
        .exec();
    eprintln!("{}: {}", command_name, err);
    std::process::exit(1);
}
