/*
 * Copyright (c) 2019 Erik Nordstrøm <erik@nordstroem.no>
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

// XXX: Because we version the repotools utilities individually, we must extract the version
//      ourselves, rather than using clap::crate_version!() / env!("CARGO_PKG_VERSION").
const PU_COMMAND_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/pu_utility_version.in"));

fn main ()
{
  let yaml = load_yaml!("cli.yaml");
  let command = App::from_yaml(yaml).version(PU_COMMAND_VERSION);
  let command_name: String = command.get_name().into();
  let args = command.get_matches();

  let mut git_args = vec![];

  if args.is_present("push_tags")
  {
    git_args.push("--tags");
  }

  if args.is_present("force_push")
  {
    git_args.push("-f");
  }

  git_args.push("--");

  if let Some(repository) = args.value_of("repository")
  {
    git_args.push(repository);
  }

  let err = exec::Command::new("git").arg("push").args(&git_args).exec();
  eprintln!("{}: {}", command_name, err);
  std::process::exit(1);
}
