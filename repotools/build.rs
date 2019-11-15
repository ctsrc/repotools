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

use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};
use toml::Value;

fn main ()
{
  // XXX: Because we version the repotools utilities individually, we must extract the version
  //      ourselves, rather than using clap::crate_version!() / env!("CARGO_PKG_VERSION").

  // This build.rs is common to repotools (where it lives) and individual utilities (symlinks).
  // We could have had separate build.rs files for repotools and for individual utilities,
  // but having it all in one shared file means less code to keep track of, maintain and
  // keep in sync.

  let out_dir = std::env::var("OUT_DIR").unwrap();

  // First we read the Cargo.toml of the current package. This will either be
  // the Cargo.toml of repotools or the Cargo.toml of an individual utility,
  // depending on what is being built. To us there is no difference.
  let cargo_toml: Value =
  {
    let cargo_toml_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
      .join("Cargo.toml");
    let mut cargo_toml_f = File::open(&cargo_toml_path).unwrap();
    let mut cargo_toml_str = String::new();
    cargo_toml_f.read_to_string(&mut cargo_toml_str).unwrap();
    toml::from_str(&cargo_toml_str).unwrap()
  };

  // In repotools Cargo.toml there are several bin entries.
  // In individual utility Cargo.toml
  for utility in cargo_toml["bin"].as_array().unwrap()
  {
    let utility_name = utility["name"].as_str().unwrap();
    let utility_package_name = utility_name.to_owned() + "2";
    eprintln!("{}", utility_name);
    eprintln!("{}", utility_package_name);

    let utility_cargo_toml: Value =
    {
      let utility_cargo_toml_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(utility_package_name + ".toml");
      let mut utility_cargo_toml_f = File::open(&utility_cargo_toml_path).unwrap();
      let mut utility_cargo_toml_str = String::new();
      utility_cargo_toml_f.read_to_string(&mut utility_cargo_toml_str).unwrap();
      toml::from_str(&utility_cargo_toml_str).unwrap()
    };

    let utility_version = utility_cargo_toml["package"]["version"].as_str().unwrap();

    let ver_in_path = Path::new(&out_dir).join(utility_name.to_owned() + "_utility_version.in");
    let mut ver_in_f = File::create(&ver_in_path).unwrap();
    ver_in_f.write(utility_version.as_bytes()).unwrap();
  }
}
