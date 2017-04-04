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

lazy_static! {
    static ref RE: Regex = Regex::new(r"\$\{(\w+)\}").unwrap();
}

fn template<'t>(input: &'t [u8]) -> (::std::borrow::Cow<'t, [u8]>, bool) {
    lazy_static! {
        static ref ENV: HashMap<String, String> = HashMap::from_iter(env::vars());
    }
    let mut replaced_all = true;
    let result = RE.replace_all(input, |caps: &Captures| {
        let var_name = caps.get(1).unwrap().as_bytes();
        let stringified_name = String::from_utf8_lossy(var_name).into_owned();
        ENV.get(&stringified_name).map(|x| x.as_bytes()).unwrap_or_else(|| {
            replaced_all = false;
            caps.get(0).unwrap().as_bytes()
        }).to_vec()
    });
    (result, replaced_all)
}

fn list_matches(input: &[u8]) -> Vec<String> {
    Vec::from_iter(RE.captures_iter(input).map(|caps: Captures|
        String::from_utf8_lossy(caps.get(1).unwrap().as_bytes()).into_owned()
    ))
}

/// = env-templater
/// templates files with environment variables
/// replaces accurances of patterns like ${SOME_NAME} with a value of SOME_NAME taken from environment
///
/// USAGE:
///     env-templater [FLAGS] [ARGS]
///
/// FLAGS:
///     -h, --help           Prints help information
///     -l, --list           List required environment variables
///     -r, --require-all    Fail if not all enrionent variables available
///     -V, --version        Prints version information
///
/// ARGS:
///     <input>      [default: /dev/stdin]
///     <output>     [default: /dev/stdout]

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Andrii Dmytrenko <andrii@dmytrenko.uk>")
        .about("templates files with environment variables")
        .arg(Arg::with_name("list").long("list").short("l").help("List required environment variables"))
        .arg(Arg::with_name("require-all").long("require-all").short("r").help("Fail if not all enrionent variables available"))
        .arg(Arg::with_name("input").default_value("/dev/stdin"))
        .arg(Arg::with_name("output").default_value("/dev/stdout"))
        .get_matches();

    let input_file_name = matches.value_of("input").unwrap();
    let output_file_name = matches.value_of("output").unwrap();
    let mut f1 = File::open(input_file_name).expect("Can't open input file");
    let mut f2 = File::create(output_file_name).expect("Can't open output file");

    let mut input = Vec::with_capacity(4_194_304);
    f1.read_to_end(&mut input).expect("Can't read input file");

    if matches.is_present("list") {
        for m in list_matches(&input) {
            f2.write_fmt(format_args!("{}\n", m)).unwrap();
        }
    } else {
        let (result, replaced_all) = template(&input);
        if matches.is_present("require-all") && !replaced_all {
            drop(f2);
            ::std::process::exit(1);
        }
        f2.write_all(result.as_ref()).expect("Can't write output file");
    }
}
