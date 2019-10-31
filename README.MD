![DataDot](./img/datadot.png?raw=true) 

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/datadot/badge.svg?branch=master)](https://coveralls.io/github/dsietz/datadot?branch=master)
[![Docs.rs](https://docs.rs/datadot/badge.svg)](https://docs.rs/datadot)

Linux: [![Build Status](https://travis-ci.org/dsietz/datadot.svg?branch=master)](https://travis-ci.org/dsietz/datadot)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/klevb6aknr6tgohi/branch/master?svg=true)](https://ci.appveyor.com/project/dsietz/datadot/branch/master)


---


>Looking for a cluster of syncronized bots to do your work? Just spin up a colony of DataDots.

**A Data Dot is a micro executable that acts on its specified data point that is defined at start up. Once started, a DataDot ...**

- Interacts with the user on the command line
- Collaboratively works together with other DataDots by speaking and listening
- Learns about its own datapoint by continuously discovering and analyzing the DataDots in its colony

---

#### Reference Material
- [multicasting](https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html)
- [sockets2](https://crates.io/crates/socket2)
- [Multicast Example](https://github.com/bluejekyll/multicast-example)
- [using a worker thread](https://matklad.github.io/2018/03/02/stopping-a-rust-worker.html)
- [Dynamic Reload for plugins](https://github.com/emoon/dynamic_reload)

#### RUNNING
testing
```
[machine-name sadp]$ cargo test -- --nocapture
```

usage
```
[machine-name datadot]$ ./datadot --help
DataDot

USAGE:
    datadot [OPTIONS] --bind <bind> --data <data>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bind <bind>             The multicast address to bind to (e.g.: 224.0.1.255
    -d, --data <data>             The data that the dot represents
    -x, --plg_dir <plugin_dir>    [Optional] The path to the directory where the plugins are located.
    -p, --port <port>             [Optional] The port number to use for multicasting, (default 7645).
```
starting datadot
```
[machine-name datadot]$ ./datadot -d "Hello" -b 224.0.0.1 -x "./plugins"
dot:listener: joined: 224.0.0.1:7645
dot:listener: is ready
```
braodcast a message
```
get::hi
dot:listener: received request: hi from: 10.172.12.144:54260
```
calling a plugin function (e.g.: plugin uses core::)
```
core::datapoint
datapoint is Hello
"processed..."
core::mask
"processed..."
core::datapoint
datapoint is secret
"processed..."
```
stopping datadot
```
stop
dot:listener: received request: stop from: 10.172.12.144:35821
dot:listener: stopped!
dropping plugins...
Bye!
```

##### NOTE
- [dynamic library](https://doc.rust-lang.org/cargo/reference/manifest.html#building-dynamic-or-static-libraries)
- [dynamic loading plugins](https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html)
- [plugin model not dynamic](https://stackoverflow.com/questions/34968488/is-it-possible-to-make-plugin-extension-hooks-like-wordpress-actions-in-rust)
- [hook model](https://mattgathu.github.io/simple-events-hook-rust/)
