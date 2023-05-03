# About

This repository contains an iTunesDB file parser. There are several types of iTunesDB files: there is the 'classic'/main iTunesDB file that stores music information, and then there are other types of iTunesDB files, like 'Photos Database', that contain supplemental info about the Photos on the device.

Currently, only support for the Photos Database is implemented, since that's the only file I have access to.

The source code for the parser is in the `itunesdb_parser` directory.

The documentation about iTunesDB files comes from the [iPod Linux](https://en.wikipedia.org/wiki/IPodLinux) website, here: http://www.ipodlinux.org/ITunesDB. Because this distribution isn't maintained anymore, I was worried that the website would be taken down at some point, so I downloaded the documentation. Both the original website HTML, and a markdown-formatted version are in the `itunesdb-doc` folder: [Link](./itunesdb-doc/README.md)

The `sample-files` directory contains some Photos Database files that I saved from an old iPod.

# Code

The parser is written in Rust. You can build it by running `cargo build`. It requires two arguments: (1) the iTunesDB filename, (2) the type of iTunesDB file, but currently only "photo" (for Photos Database) is supported.

```bash
$ /target/debug/itunesdb_parser <path-to-itunesdb-file> 'photo'
```

# Future roadmap

This project is an early work-in-progress. The biggest thing I'm missing so far is support for extracting the actual photo thumbnails themselves.

At some point, I'd like to add support for the other database types. I don't have an iPod anymore, or any other files from it besides the Photos Database ones. If you're interested in contributing, open an issue.
