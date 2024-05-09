# extname - the lost sibling
There is `basename` and `dirname`, but not tool to get the file extension of a given file.
This is where `extname` comes in.

`extname` is a simple tool to work like `basename` and `dirname`, but provides the extension name of a file.


## Installation
First, clone this git repository
```sh
git clone git@github.com:mimre25/extname.git
```
or
```sh
git clone https://github.com/mimre25/extname.git
```

Then, run
```sh
cargo build --release
```
to build the binaries.

Last, copy the binaries from `target/release/extname` to some place on your path, or add `target/release` to your path.

## Anything else?
Nah, that's all.
Just wanted to solve this little problem for myself and explore `rust` a bit on the way.


# Use at your own risk
