Level
=====

Change levels of screen backlight and audio volume in the CLI.


Installation
------------

This tool is a wrapper around `amixer` and `xbacklight` so you should install
them first.

Then you should install Rust:

    $ curl https://sh.rustup.rs -sSf | sh

And finally you can install the latest stable version with cargo:

    $ cargo install level

Or the development version by fetching the git repository:

    $ git clone git://github.com/vinc/level.git
    $ cd level
    $ cargo install --path "."


Usage
-----

Use arrow keys to update screen backlight or audio volume, and the escape key
to exit.

```
$ level screen
Screen [########################------------]  67%
```


License
-------

Copyright (c) 2020 Vincent Ollivier. Released under MIT.
