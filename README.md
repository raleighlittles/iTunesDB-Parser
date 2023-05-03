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

This will output all of the information about the relevant structures that were in your iTunesDB file. The output is probably going to be pretty large, so I recommend redirecting it to a text file.

Here's an example of what to expect:

```
ImageList#0 info... NumImages=883
==========
ImageItem#0 info... Rating= 0 , ImgSize= 0, OrigDateTS= 2008-08-23 21:50:00 UTC , DigitizedDateTS= 2008-08-23 21:50:00 UTC
==========
DataObject#0 info... Type=5
==========
ImageName#0 info... Size(bytes)=523225 , Height=0 , Width=0
==========
MHOD substring (UTF-16)= E::DCIM:112CANON:IMG_1100.JPG
String MHOD detected... Len=58, Encoding (raw)=2
DataObject#1 info... Type=3
==========
DataObject#2 info... Type=6
==========
ImageItem#1 info... Rating= 0 , ImgSize= 0, OrigDateTS= 2008-08-23 21:50:12 UTC , DigitizedDateTS= 2008-08-23 21:50:12 UTC
==========
DataObject#3 info... Type=5
==========
ImageName#1 info... Size(bytes)=473779 , Height=0 , Width=0
==========
MHOD substring (UTF-16)= E::DCIM:112CANON:IMG_1101.JPG
String MHOD detected... Len=58, Encoding (raw)=2
DataObject#4 info... Type=3
==========
DataObject#5 info... Type=6
==========
ImageItem#2 info... Rating= 0 , ImgSize= 0, OrigDateTS= 2008-08-23 23:14:42 UTC , DigitizedDateTS= 2008-08-23 23:14:42 UTC
==========
DataObject#6 info... Type=5
==========
ImageName#2 info... Size(bytes)=491549 , Height=0 , Width=0
==========
MHOD substring (UTF-16)= E::DCIM:112CANON:IMG_1102.JPG
String MHOD detected... Len=58, Encoding (raw)=2
DataObject#7 info... Type=3
==========
DataObject#8 info... Type=6
==========
ImageItem#3 info... Rating= 0 , ImgSize= 0, OrigDateTS= 2008-08-23 23:15:02 UTC , DigitizedDateTS= 2008-08-23 23:15:02 UTC
==========
DataObject#9 info... Type=5
==========
ImageName#3 info... Size(bytes)=462812 , Height=0 , Width=0
==========
MHOD substring (UTF-16)= E::DCIM:112CANON:IMG_1103.JPG
String MHOD detected... Len=58, Encoding (raw)=2
DataObject#10 info... Type=3
==========
DataObject#11 info... Type=6
```

# Future roadmap

This project is an early work-in-progress. The biggest thing I'm missing so far is support for extracting the actual photo thumbnails themselves.

At some point, I'd like to add support for the other database types. I don't have an iPod anymore, or any other files from it besides the Photos Database ones. If you're interested in contributing, open an issue.
