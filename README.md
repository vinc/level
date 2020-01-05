Level
=====

Change levels of screen backlight and audio volume in the CLI.


Installation
------------

First you need to install Rust:

    $ curl https://sh.rustup.rs -sSf | sh

Then you can install the latest stable version with cargo:

    $ cargo install level

Or the development version by fetching the git repository:

    $ git clone git://github.com/vinc/level.git
    $ cd level
    $ cargo install --path "."


Usage
-----

Use arrow keys to update screen backlight or audio volume:

```
$ level screen
Screen [########################------------]  67%
```


License
-------

Copyright (c) 2020 Vincent Ollivier. Released under MIT.
