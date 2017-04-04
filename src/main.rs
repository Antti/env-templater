extern crate clap;
extern crate regex;
#[macro_use] extern crate lazy_static;

use clap::{Arg, App};
use std::fs::File;
use std::io::{Read, Write};
use regex::bytes::{Regex, Captures};
use std::env;
use std::collections::HashMap;
use std::iter::FromIterator;

fn template<'t>(input: &'t [u8]) -> ::std::borrow::Cow<'t, [u8]> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\$\{(\w+)\}").unwrap();
        static ref ENV: HashMap<String, String> = HashMap::from_iter(env::vars());
    }
    RE.replace_all(input, |caps: &Captures| {
        let var_name = caps.get(1).unwrap().as_bytes();
        let stringified_name = String::from_utf8_lossy(var_name).into_owned();
        ENV.get(&stringified_name).map(|x| x.as_bytes()).unwrap_or(caps.get(0).unwrap().as_bytes()).to_vec()
    })
}

fn main() {
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author("Andrii Dmytrenko <andrii@dmytrenko.uk>")
    .about("templates files with environment variables")
    .arg(Arg::with_name("input").index(1).default_value("/dev/stdin"))
    .arg(Arg::with_name("output").index(2).default_value("/dev/stdout"))
    .get_matches();

  let input_file_name = matches.value_of("input").unwrap();
  let output_file_name = matches.value_of("output").unwrap();
  let mut f1 = File::open(input_file_name).expect("Can't open input file");
  let mut f2 = File::create(output_file_name).expect("Can't open output file");
  let mut input = Vec::with_capacity(4_194_304);
  f1.read_to_end(&mut input).expect("Can't read input file");
  let result = template(&input);
  f2.write_all(result.as_ref()).expect("Can't write output file");
}
