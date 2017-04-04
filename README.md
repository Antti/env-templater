env-templater replaces names of env variables with their values.

## Code Status
[![Build Status](https://travis-ci.org/Antti/env-templater.svg?branch=master)](https://travis-ci.org/Antti/env-templater)

## Installing env-templater

## Compiling from Source

```
cargo build --release
```

## Usage

```
env-templater 0.1.0
Andrii Dmytrenko <andrii@dmytrenko.uk>
templates files with environment variables

USAGE:
    env-templater [FLAGS] [ARGS]

FLAGS:
    -h, --help           Prints help information
    -l, --list           List required environment variables
    -r, --require-all    Fail if not all enrionent variables available
    -V, --version        Prints version information

ARGS:
    <input>      [default: /dev/stdin]
    <output>     [default: /dev/stdout]
```

## License

env-templater is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
