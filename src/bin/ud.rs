/*
 * Copyright (c) 2023 Erik Nordstrøm <erik@nordstroem.no>
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
use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    let yaml = load_yaml!("ud.yaml");
    let command = App::from_yaml(yaml);
    let command_name: String = command.get_name().into();
    let args = command.get_matches();

    let paths: Vec<_> = match args.values_of("path") {
        Some(paths) => paths.collect(),
        None => vec![],
    };

    let output = std::process::Command::new("git")
        .arg("restore")
        .arg("--staged")
        .arg("--")
        .args(&paths)
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    if !output.status.success() {
        match output.status.code() {
            Some(status_code) => {
                std::process::exit(status_code);
            }
            None => {
                std::process::exit(1);
            }
        }
    }
    let err = exec::Command::new("git")
        .arg("restore")
        .arg("--")
        .args(&paths)
        .exec();
    eprintln!("{}: {}", command_name, err);

    // Getting to this point means preflight failed or exec failed.
    std::process::exit(1);
}
