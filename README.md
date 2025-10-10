# ctry 1.1.1

Retry CLI commands

```bash
# Stop retrying after 10 tries
retry --max 10 -- curl -I https://unstable.site

# Stop retrying after 10 tries and wait 5 seconds between the each try
retry --max 10 --interval 5 -- curl -I https://unstable.site

# Stop retrying if exit code is 1
retry --exitcode 1 -- curl -I https://unstable.site

# Suppress stdout and stderr from the command
retry --quiet -- curl -I https://unstable.site
```

## Installation
```
cargo-binstall ctry
```

## Usage
```
ctry 1.0.1
Retry CLI commands

USAGE:
    ctry [FLAGS] [OPTIONS] [--] <COMMAND>...

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Suppress output of the command
    -V, --version    Prints version information

OPTIONS:
    -e, --exitcode <EXIT_CODE>    On which exit code retries will stop [default: 0]
    -i, --interval <INTERVAL>     Interval in seconds between the retries [default: 1]
    -m, --max <MAX_RETRIES>       Maximum retries, use 0 for unlimited retries [default: 5]

ARGS:
    <COMMAND>...    Command to run
```
