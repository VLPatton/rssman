# RSS and Atom Feed Manager
## rssman
This project was something I wrote real quick that probably could've been done
in something like Python a little easier, but I don't care that much and Rust is
fun anyway.
## Installation
```sh
cargo install --git https://github.com/VLPatton/rssman.git
```
## Usage
```
rssman [-c|--categories <LIST>] [-n|--number <N>]
    LIST - A space-separated list of categories to check for on entries/feed
    items (if left empty, it ignores categories).

    N - The number of URLs in $XDG_CONFIG_HOME/rssman/urls.txt to actually scan
    (if left empty, it will scan all).
```
## Config
Configuration is done in `$XDG_CONFIG_HOME/rssman/urls.txt` for a list of URLs
to scan. They must be fully qualified (i.e, start with `https://`, etc). Each
URL is separated by a newline.
## Output
```
<TITLE>:
  Description:
<DESCRIPTION>
  Author:
<AUTHORS...>
  Link:
<LINK>
```
The output is best displayed when piped into `more` (ie, `rssman | more`).
`less` has trouble displaying the ANSI escape codes used for color, and may
provide an odd looking output.
## Platform support
I test this on Arch Linux x86\_64, so it'll probably work on most Unix-like
systems that have a Rust toolchain. It doesn't support Windows b/c I don't use
it and I wrote this in the span of about 2 hours for v0.1.0.
