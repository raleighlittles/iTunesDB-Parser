# About

This is a Python script that allows you to automatically rename songs on your iPod according to the info in the iTunesDB file, for recovery purposes.

To start, you must've run the parser tool on your iTunesDB file (see README in repository root) and had it produce a CSV file.

You will then copy the song files on your iPod to a separate directory (do NOT override/rename files on your actual iPod itself) on your host PC.

```bash
$ find <IPOD-MUSIC-DIRECTORY> -name "*.mp3|*.m4a" -type f -exec cp {} ./<DIRECTORY-ON-PC> \;
```

Then run this script, passing in the path to the CSV file you generated, and the directory where the song files are.

This script will rename those song files using the song title and song artist in the CSV file.

ie..

```
./RBCG.mp4
./REXL.mp3
./LVXD.mp3
./NNMT.mp3
./SEMY.mp3
./MKIV.mp3
./QPMR.mp3
./NEWF.mp3
./KSMR.mp3
./TWZE.mp3
```

can become:

```
./'Spit It Out - Slipknot.m4a'
./'Storming The Gates Of Hell - Impending Doom.mp3'
./'Strife (Chug Chug) - As Blood Runs Black.mp3'
./'Sulfur - Slipknot.mp3'
./'The Darkest Day Of Man - Whitechapel.mp3'
./'The Day Of Justice - Whitechapel.mp3'
./'There Is No Business To Be Done On A Dead Planet - As Blood Runs Black.mp3'
./'The Takeover - Born of Osiris.mp3'
./'The Tragic Truth - Five Finger Death Punch.mp3'
./'The True Beast - All Shall Perish.mp3'

```

# Usage info

```
usage: Apply recovered song data to music files on an iPod [-h] -i SONG_DIRECTORY -c ITUNES_CSV -o OUTPUT_DIRECTORY

options:
  -h, --help            show this help message and exit
  -i SONG_DIRECTORY, --song-directory SONG_DIRECTORY
                        Directory to look for songs
  -c ITUNES_CSV, --itunes-csv ITUNES_CSV
                        Path to iTunesDB CSV file
  -o OUTPUT_DIRECTORY, --output-directory OUTPUT_DIRECTORY
                        Directory where to put renamed songs

```