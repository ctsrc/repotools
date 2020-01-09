/*
 * Copyright (c) 2019, 2020 Erik Nordstrøm <erik@nordstroem.no>
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

fn main ()
{
  let yaml = load_yaml!("di.yaml");
  let command = App::from_yaml(yaml);
  let command_name: String = command.get_name().into();
  let args = command.get_matches();

  let paths: Vec<_> = match args.values_of("path")
  {
    Some(paths) => paths.collect(),
    None => vec![],
  };

  // TODO: Check that both files are inside of the current git repository.
  //       Otherwise, git calls diff and the resulting message can be confusing
  //       to users; "fatal: invalid diff option/value: --cached".

  let err = exec::Command::new("git")
    .arg("diff").arg("--cached").arg("--").args(&paths)
    .exec();
  eprintln!("{}: {}", command_name, err);
  std::process::exit(1);
}
