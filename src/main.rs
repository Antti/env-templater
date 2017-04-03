extern crate clap;
extern crate regex;
#[macro_use] extern crate lazy_static;

use clap::{Arg, App};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, BufRead, Read, Write};
use regex::{Regex, Captures};
use std::env;
use std::collections::HashMap;
use std::iter::FromIterator;

fn template<I, O>(input: I, output: O) -> io::Result<usize> where I: Read, O: Write {
    let input = BufReader::new(input);
    let mut output = BufWriter::new(output);
    let mut total_len = 0;
    for line in input.lines() {
        let l = format!("{}\n", line.unwrap());
        total_len += l.len();
        output.write_all(template_str(&l).as_bytes())?;
    }
    Ok(total_len)
}

fn template_str<'t>(input: &'t str) -> ::std::borrow::Cow<'t, str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\$\{(\w+)\}").unwrap();
        static ref ENV: HashMap<String, String> = HashMap::from_iter(env::vars());
    }
    RE.replace_all(input, |caps: &Captures| {
        let var_name = caps.get(1).unwrap().as_str();
        ENV.get(var_name).map(|x| x.clone()).unwrap_or(caps.get(0).unwrap().as_str().to_string())
    })
}

fn main() {
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author("Andrii Dmytrenko <andrii@dmytrenko.uk>")
    .about("templates files with environment variables")
    .arg(Arg::with_name("input").index(1))
    .arg(Arg::with_name("output").index(2))
    .get_matches();

  let input_name = matches.value_of("input").unwrap_or("/dev/stdin");
  let output_name = matches.value_of("output").unwrap_or("/dev/stdout");
  let f1 = File::open(input_name).unwrap();
  let f2 = File::create(output_name).unwrap();
  template(f1, f2).unwrap();
}
