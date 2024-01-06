<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en" dir="ltr">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
<meta name="KEYWORDS" content="ITunesDB,Template:Lowercase,ITunesDB/Basic Information,ITunesDB/iTunesDB File,ITunesDB/Play Counts File,ITunesDB/OTG Playlist File,ITunesDB/Equalizer Presets File,ITunesDB/Artwork Database,ITunesDB/Photo Database,ITunesDB/Misc. Files">
<meta name="robots" content="index,follow">

<link rel="shortcut icon" href="http://www.ipodlinux.org/favicon.ico">

<link rel="copyright" href="http://www.gnu.org/copyleft/fdl.html">
  
<h1 class="firstHeading">

ITunesDB

</h1>
<div id="bodyContent">
<h3 id="siteSub">

From wikiPodLinux

</h3>
<div id="contentSub">
</div>
<center>

<ins class="adsbygoogle" style="display:block" data-ad-client="ca-pub-2601587627618324" data-ad-slot="8010159564" data-ad-format="auto" data-full-width-responsive="true"></ins>

</center>
<dl>
<dd>

<i>The title of this article should be <b>iTunesDB</b>. The initial
letter is capitalized due to
<a rel="nofollow" href="http://www.wikipedia.org/wiki/Naming_conventions_%28technical_restrictions%29#Lower_case_first_letter" class="extiw" title="Wikipedia:Naming conventions (technical restrictions)">technical
limitations</a>.</i>

</dd>
</dl>

This page details the format of the binary files used on the iPod to
keep track of the music it contains as well as its play history.
Collectively we refer to these files as the iTunesDB however there are
in fact a number of files, each with their own format, that make up this
database.

<b>Note that the information on this page is undergoing constant
revision as new information comes to light. New versions of the iTunes
or iPod software may cause drastic changes to the information in this
document.</b>

<table class="infobox" style="border: 1px solid #aaaaaa; background-color: #f9f9f9; color: black; margin-bottom: 0.5em; margin-left: 1em; padding: 0.2em; float: right; clear: right; font-size: 95%; text-align: left; background-color: #f0f6fa;">
<tbody>
<tr>
<td colspan="2" align="center">

<b>Subpages</b>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a href="http://www.ipodlinux.org/ITunesDB/Basic_Information.html" title="ITunesDB/Basic Information">Basic
Information</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a href="http://www.ipodlinux.org/ITunesDB/iTunesDB_File.html" title="ITunesDB/iTunesDB File">iTunesDB
File</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a title="ITunesDB/Play Counts File">Play Counts File</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a title="ITunesDB/OTG Playlist File">OTG Playlist File</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a title="ITunesDB/Equalizer Presets File">Equalizer Presets File</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a title="ITunesDB/Artwork Database">Artwork Database</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a title="ITunesDB/Photo Database">Photo Database</a>

</td>
</tr>
<tr>
<td colspan="2" align="center">

<a title="ITunesDB/Misc. Files">Misc. Files</a>

</td>
</tr>
</tbody>
</table>
<table border="0" id="toc">
<tbody>
<tr id="toctitle">
<td align="center">

<b>Table of contents</b>

<span class="toctoggle">\[<a href="javascript:toggleToc()" class="internal"><span id="showlink" style="display:none;">show</span><span id="hidelink">hide</span></a>\]</span>

</td>
</tr>
<tr id="tocinside">
<td>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#Basic_information">1 Basic
information</a>  

</div>
<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Endian_Note">1.1 Endian
Note</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesDB_file">2 iTunesDB
file</a>  

</div>
<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Chunk_Encoding">2.1 Chunk
Encoding</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Database_Object">2.2
Database Object</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#DataSet">2.3 DataSet</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#TrackList">2.4
TrackList</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Track_Item">2.5 Track
Item</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Playlist_List">2.6 Playlist
List</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Playlist">2.7 Playlist</a>  

<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#List_Sort_Order">2.7.1 List
Sort Order</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Podcasts">2.7.2
Podcasts</a>  

</div>

<a href="http://www.ipodlinux.org/ITunesDB/#Playlist_Item">2.8 Playlist
Item</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Data_Object">2.9 Data
Object</a>  

<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#String_Types">2.9.1 String
Types</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Podcast_URLs_.28types_15-16.29">2.9.2
Podcast URLs (types 15-16)</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_.28type_17.29">2.9.3
Chapter Data (type 17)</a>  

<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_mhod_details">2.9.3.1
Chapter Data mhod details</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_Atom_Layout">2.9.3.2
Chapter Data Atom Layout</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_Atoms">2.9.3.3
Chapter Data Atoms</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_String_Atoms_.28UTF16.29">2.9.3.4
Chapter Data String Atoms (UTF16)</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_String_Atoms_.28UTF8.29">2.9.3.5
Chapter Data String Atoms (UTF8)</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_hedr_Atom">2.9.3.6
Chapter Data hedr Atom</a>  

</div>

<a href="http://www.ipodlinux.org/ITunesDB/#Smart_Playlist_Data_.28type_50.29">2.9.4
Smart Playlist Data (type 50)</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Smart_Playlist_Rules_.28type_51.29">2.9.5
Smart Playlist Rules (type 51)</a>  

<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Important_Note_about_endian-ness">2.9.5.1
Important Note about endian-ness</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Smart_Playlist_Rule_Fields">2.9.5.2
Smart Playlist Rule Fields</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Smart_Playlist_Rule_Actions">2.9.5.3
Smart Playlist Rule Actions</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Smart_Playlist_Rule_Values">2.9.5.4
Smart Playlist Rule Values</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Putting_It_All_Together">2.9.5.5
Putting It All Together</a>  

</div>

<a href="http://www.ipodlinux.org/ITunesDB/#Library_Playlist_Index_.28type_52.29">2.9.6
Library Playlist Index (type 52)</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Type_53">2.9.7 Type 53</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Type_100">2.9.8 Type
100</a>  

<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Playlist_Column_Definitions_Header">2.9.8.1
Playlist Column Definitions Header</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Column_Definition">2.9.8.2
Column Definition</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Playlist_Order_Entry">2.9.8.3
Playlist Order Entry</a>  

</div>
</div>

<a href="http://www.ipodlinux.org/ITunesDB/#Album_List">2.10 Album
List</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Album_Item">2.11 Album
Item</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#Play_Counts_file">3 Play
Counts file</a>  

</div>
<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Play_Count_Header">3.1 Play
Count Header</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Play_Count_Entry">3.2 Play
Count Entry</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#On_The_Go_Playlist_files">4
On The Go Playlist files</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#Equalizer_Presets_file">5
Equalizer Presets file</a>  

</div>
<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Equalizer_Presets_Container">5.1
Equalizer Presets Container</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Equalizer_Preset">5.2
Equalizer Preset</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#Artwork_Database">6 Artwork
Database</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#Photo_Database">7 Photo
Database</a>  

</div>
<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Data_File_Object">7.1 Data
File Object</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#DataSet_2">7.2 DataSet</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Image_List">7.3 Image
List</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Image_Item">7.4 Image
Item</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Image_Name">7.5 Image
Name</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Album_List_2">7.6 Album
List</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Photo_Album">7.7 Photo
Album</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Album_Item_2">7.8 Album
Item</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#File_List">7.9 File
List</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#File_.28Image.29">7.10 File
(Image)</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#Data_Object_2">7.11 Data
Object</a>  

<div class="tocindent">

<a href="http://www.ipodlinux.org/ITunesDB/#Container_MHODs">7.11.1
Container MHODs</a>  
<a href="http://www.ipodlinux.org/ITunesDB/#String_MHODs">7.11.2 String
MHODs</a>  

</div>
</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesSD_file">8 iTunesSD
file</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesStats_File">9
iTunesStats File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesShuffle_File">10
iTunesShuffle File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesPState_File">11
iTunesPState File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesControl_File">12
iTunesControl File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesPrefs_File">13
iTunesPrefs File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#winPrefs_File">14 winPrefs
File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#DeviceInfo_File">15
DeviceInfo File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iEKInfo_File">16 iEKInfo
File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#WABContactsGroup_File">17
WABContactsGroup File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesLock_File">18
iTunesLock File</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#PhotosFolderName">19
PhotosFolderName</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#PhotoFolderPrefs">20
PhotoFolderPrefs</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#PhotoFolderAlbums">21
PhotoFolderAlbums</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesPlaylists">22
iTunesPlaylists</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesPodcasts">23
iTunesPodcasts</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#iTunesVideoPlaylists">24
iTunesVideoPlaylists</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#Preferences">25
Preferences</a>  

</div>
<div class="tocline">

<a href="http://www.ipodlinux.org/ITunesDB/#_volumelocked">26
\_volumelocked</a>  

</div>
</td>
</tr>
</tbody>
</table>

  

<div align="right">

<small>→
<a href="http://www.ipodlinux.org/ITunesDB/Basic_Information.html" title="ITunesDB/Basic Information">Subpage:
Basic Information</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Basic_information"></a>

<h2>

Basic information

</h2>

The database directory on the iPod is <b>/iPod\_Control/iTunes/</b>.
This is a hidden folder (attrib +h), but is not otherwise protected.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iPod Database Files</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

filename

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

iTunesDB

</td>
<td>

This is the primary database for the iPod. It contains all information
about the songs that the iPod is capable of playing, as well as the
playlists. It's never written to by the Apple iPod firmware. During an
autosync, iTunes completely overwrites this file.

</td>
</tr>
<tr>
<td>

Play Counts

</td>
<td>

This is the return information file for the iPod. It contains all
information that is available to change via the iPod, with regards to
the song. When you autosync, iTunes reads this file and updates the
iTunes database accordingly. After it does this, it erases this file, so
as to prevent it from duplicating data by mistake. The iPod will create
this file on playback if it is not there.

</td>
</tr>
<tr>
<td>

OTGPlaylist / OTGPlaylist\_\#

</td>
<td>

These are the playlist files created by the iPod when you create a new
playlist. 4th Gen and up iPod's can create multiple OTGPlaylists. These
have the same format as the 3rd gen models, they just get numbers
appended to the name. iTunes will turn these into normal playlists and
then delete these files during an autosync. The OTGPlaylists cannot
survive changing the contents of the iPod.

</td>
</tr>
<tr>
<td>

iTunesEQPresets

</td>
<td>

This is where the equalizer presets that you create in iTunes get stored
on the iPod. The format of this file is easy and well understood, but
strangely, no iPod actually uses this file yet. You can create an EQ
preset in iTunes, sync with an iPod, and the EQ will be put in here by
iTunes correctly. But the iPod never reads the file, does not use the
preset in this file, and just doesn't work correctly in this respect.
Perhaps it will be fixed in a future firmware release.

</td>
</tr>
<tr>
<td>

ArtworkDB

</td>
<td>

This is where data about artwork is stored on the iPod Photo devices.
The artwork itself is stored in the \\iPod\_Control\\Artwork folder. On
5g iPods the ArtworkDB is stored in the Artwork folder along with the
data.

</td>
</tr>
<tr>
<td>

iTunesSD

</td>
<td>

This is the database that seems to hold information used by the iPod
Shuffle device.

</td>
</tr>
<tr>
<td>

iTunesStats

</td>
<td>

The equivalent of the "Play Counts" file for the Shuffle.

</td>
</tr>
<tr>
<td>

iTunesShuffle

</td>
<td>

Contains a list of the tracks on the Shuffle in a random order. Possibly
used to specify the order in which the songs are shuffled.

</td>
</tr>
<tr>
<td>

iTunesPState

</td>
<td>

Contains the current playback position and volume of the iPod shuffle.

</td>
</tr>
<tr>
<td>

iTunesControl

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
<tr>
<td>

iTunesPrefs

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
<tr>
<td>

winPrefs

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
<tr>
<td>

iEKInfo

</td>
<td>

This contains the decryption key for protected M4P playback. Previously
iSCInfo(?)

</td>
</tr>
<tr>
<td>

WABContactsGroup

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
<tr>
<td>

iTunesLock

</td>
<td>

Created whenever iTunes needs exclusive control of the iPod, like when
reading or writing the iTunesDB file.

</td>
</tr>
<tr>
<td>

PhotoFolderPrefs

</td>
<td>

Stores Photo Folder preferences.

</td>
</tr>
<tr>
<td>

PhotoFolderAlbums

</td>
<td>

Stores Photo Albums.

</td>
</tr>
<tr>
<td>

iTunesPlaylists

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
<tr>
<td>

iTunesPodcasts

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
<tr>
<td>

iTunesVideoPlaylists

</td>
<td>

Nothing more specific is known, yet.

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Endian_Note"></a>

<h3>

Endian Note

</h3>

With some exceptions, most of the data descriptions in this document are
actually stored in the file in little endian byte order. Meaning that
you have these representations of the data when looking at the file in a
hex editor:

    "01 00 00 00" = decimal 1
    "10 00 00 00" = decimal 16
    "00 01 00 00" = decimal 256

This means when you read the file, you need to reverse the bytes in
memory to make sense of them. Keep this in mind when reading this
document and trying to make sense of an iTunesDB using a hex editor.

The exceptions are noted where appropriate (hopefully). But if you are
looking at a piece of a file and can't understand the value it holds,
try reversing the byte order.

Note that this means the values themselves are in reversed byte order.
The order of the fields described is the same as in this document. So if
you see that field A is before field B in one of these tables, then when
you look at the file with a hex editor, field A really will come before
field B. This has confused some people who always reverse 4 byte chunks
of data. If you're writing code to read/write these files, I recommend
against \*always\* reversing the data in 4 byte blocks. This only makes
the code more confusing in the long run.

  

<div align="right">

<small>→
<a href="http://www.ipodlinux.org/ITunesDB/iTunesDB_File.html" title="ITunesDB/iTunesDB File">Subpage:
iTunesDB File</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesDB_file"></a>

<h2>

iTunesDB file

</h2>

The iTunesDB file consists of a sort of tree structure arranged into a
flat file. Each "object" contains a header followed by some other
objects. The header contains a couple of size fields that are used to
define where the header ends and other objects begin. Here's the basic
structure of it:

    &lt;mhbd&gt; - This is a database
      &lt;mhsd&gt; - This is a list holder, which holds either a mhla
        &lt;mhla&gt; - This holds a list of albums
          &lt;mhia&gt; - This describes a particular Album Item
            &lt;mhod&gt; - These hold strings associated with an album title
            &lt;mhod&gt; - These hold strings associated with an artist name/title
          &lt;mhia&gt; - This is another album. And so on.
            &lt;mhod&gt;...
            &lt;mhod&gt;...
          ...
      &lt;mhsd&gt; - This is a list holder, which holds either a mhlt or an mhlp
        &lt;mhlt&gt; - This holds a list of all the songs on the iPod
          &lt;mhit&gt; - This describes a particular song
            &lt;mhod&gt;... - These hold strings associated with a song
            &lt;mhod&gt;... - Things like Artist, Song Title, Album, etc.
          &lt;mhit&gt; - This is another song. And so on.
            &lt;mhod&gt;...
            &lt;mhod&gt;...
          ...
      &lt;mhsd&gt; - Here's the list holder again.. This time, it's holding an mhlp
        &lt;mhlp&gt; - This holds a bunch of playlists. In fact, all the playlists.
          &lt;mhyp&gt; - This is a playlist.
            &lt;mhod&gt;... - These mhods hold info about the playlists like the name of the list.
            &lt;mhip&gt;... - This mhip holds a reference to a particular song on the iPod.
            ...
          &lt;mhyp&gt; - This is another playlist. And so on.
            &lt;mhod&gt;... - Note that the mhods also hold other things for smart playlists
            &lt;mhip&gt;...
            ...
          ...

And so on. What follows is a detailed description of these various
headers.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chunk_Encoding"></a>

<h3>

Chunk Encoding

</h3>

Each chunk of the file is encoded in the follow form:

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Chunk Encoding</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

Offset

</th>
<th style="background:#efefef;">

Field

</th>
<th style="background:#efefef;">

Size

</th>
<th style="background:#efefef;">

Description

</th>
</tr>
<tr>
<td>

0

</td>
<td>

Chunk Type Identifier

</td>
<td>

4

</td>
<td>

A 4-byte string like "mhbd", "mhlt", etc. This identifies what type of
chunk is at the current location.

</td>
</tr>
<tr>
<td>

4

</td>
<td>

End of Type-specific Header

</td>
<td>

4

</td>
<td>

This is a little-endian encoded 32b value that points to the end of the
chunk-specific header.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

End of Chunk or Number of Children

</td>
<td>

4

</td>
<td>

This is a little-endian encoded 32b value. It either points to the end
of the current chunk, or the number of children the current chunk has.

</td>
</tr>
</tbody>
</table>

The Chunk Type Identifier is intuitive: it's a 4-byte string letting you
know what you're working with.

The End of Type-specific Header tells you where, relative to offset 0,
the header section for this chunk ends. The header starts at offset 12
and runs through to the end of the type-specific header.

The End of Chunk or Number of Children is slightly confusing. Usually,
it's an "End of Chunk" offset: what the last offset in the current chunk
is. That is, it tells you how long this chunk and all its children are.
There are two big exceptions to this: "mhlt" and "mhlp" chunks. In both
of these, this number is how many top-level children the mhl\[pt\] chunk
has.

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Database_Object"></a>

<h3>

Database Object

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhbd format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhbd

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhbd header. For dbversion \<= 0x15 (iTunes 7.2 and
earlier), the length is 0x68. For dbversion \>= 0x17 (iTunes 7.3 and
later), the size is 0xBC.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records (since everything is a child of
MHBD, this will always be the size of the entire file)

</td>
</tr>
<tr>
<td>

12

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

always seems to be 1

</td>
</tr>
<tr>
<td>

16

</td>
<td>

version number

</td>
<td>

4

</td>
<td>

appears to be a version number of the database type. 0x09 = iTunes 4.2,
0x0a = iTunes 4.5, 0x0b = iTunes 4.7, 0x0c = iTunes 4.71/4.8, 0x0d =
iTunes 4.9, 0x0e = iTunes 5, 0x0f = iTunes 6, 0x10 = iTunes 6.0.1(?),
0x11 = iTunes 6.0.2-6.0.4, 0x12 = iTunes 6.0.5., 0x13 = iTunes 7.0, 0x14
= iTunes 7.1, 0x15 = iTunes 7.2, 0x16 = ?, 0x17 = iTunes 7.3.0, 0x18 =
iTunes 7.3.1-7.3.2., 0x19 = iTunes 7.4.

</td>
</tr>
<tr>
<td>

20

</td>
<td>

number of children

</td>
<td>

4

</td>
<td>

the number of MHSD children. This has been observed to be 2 (iTunes 4.8
and earlier) or 3 (iTunes 4.9 and older), the third being the separate
podcast library in iTunes 4.9. Also it has been observed to be 4 (iTunes
7.1, 7.2) or 5 (iTunes 7.3).

</td>
</tr>
<tr>
<td>

24

</td>
<td>

id

</td>
<td>

8

</td>
<td>

appears to a 64 bit id value for this database. Not checked by the iPod,
as far as I can tell.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

unknown

</td>
<td>

2

</td>
<td>

always seems to be 2

</td>
</tr>
<tr>
<td>

38

</td>
<td>

unknown

</td>
<td>

8

</td>
<td>

Observed in dbversion 0x11 and later. It was thought that this field is
used to store some sort of starting point to generate the item's dbid,
but this idea was thrown away.

</td>
</tr>
<tr>
<td>

48

</td>
<td>

unknown

</td>
<td>

2

</td>
<td>

Observed in dbversion 0x19 and later, and must be set to 0x01 for the
new iPod Nano 3G (video) and iPod Classics. The obscure hash at offset
88 needs to be set as well.

</td>
</tr>
<tr>
<td>

50

</td>
<td>

unknown

</td>
<td>

20

</td>
<td>

Observed in dbversion 0x19 and later for the new iPod Nano 3G (video)
and iPod Classics. Meaning unknown so far.

</td>
</tr>
<tr>
<td>

70

</td>
<td>

language

</td>
<td>

2

</td>
<td>

Observed in dbversion 0x13. It looks like this is a language id
(langauge of the iTunes interface). For example for English(United
States) this field has values 0x65 and 0x6E which is 'en'. The size of
the filed might be bigger to distinguish different 'flavors' of a
language.

</td>
</tr>
<tr>
<td>

72

</td>
<td>

library persistent id

</td>
<td>

8

</td>
<td>

Observed in dbversion 0x14. This is a 64-bit Persistent ID for this iPod
Library. This matches the value of "Library Persistent ID" seen in hex
form (as a 16-char hex string) in the drag object XML when dragging a
song from an iPod in iTunes.

</td>
</tr>
<tr>
<td>

88

</td>
<td>

obscure hash

</td>
<td>

20

</td>
<td>

Observed in dbversion 0x19 for iPod Nano 3G (video) and iPod Classics.

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The Database Object has two or three children, which are Data Sets.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="DataSet"></a>

<h3>

DataSet

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhsd format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhsd

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhsd header

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

A type number.

    1 = Track list - contains an MHLT
    2 = Playlist List - contains an MHLP
    3 = Podcast List - optional, probably. Contains an MHLP also.  
    This MHLP is basically the same as the full playlist section, 
    except it contains the podcasts in a slightly different way. 
    See the Playlists section.
    4 = Album List, first seen with iTunes 7.1.
    5 = New Playlist List with Smart Playlists, first seen with iTunes 7.3.

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

Depending on the type of Data Set, it will contain either a Track List
child or a Playlist List child. Order is not guaranteed. Example files
have contained the type 3 MHSD before the type 2 MHSD. In order for the
iPod to list podcasts the type 3 Data Set MUST come between the type 1
and type 2 Data Sets.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="TrackList"></a>

<h3>

TrackList

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhlt format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhlt

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhlt header

</td>
</tr>
<tr>
<td>

8

</td>
<td>

number of songs

</td>
<td>

4

</td>
<td>

the total number of songs in the Track List

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The Track List has Track Items as its children. The number of Track
Items is the same as the number of songs.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Track_Item"></a>

<h3>

Track Item

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhit format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhit

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhit header. For dbversion \<= 0x0b (iTunes 4.7 and
earlier), the length is 0x9c. For dbversion \>= 0x0c (iTunes 4.71 and
later), the size is 0xf4. For dbversion = 0x12 (iTunes 6.0.5), 0x13
(iTunes 7.0) the size is 0x148. For dbversion \>= 0x14 (iTunes 7.1) the
size is 0x184.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of strings

</td>
<td>

4

</td>
<td>

number of strings (mhods) that are children of this mhit

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unique id

</td>
<td>

4

</td>
<td>

unique ID for a track (referenced in playlists)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

visible

</td>
<td>

4

</td>
<td>

If this value is 1, the song is visible on the iPod. All other values
cause the file to be hidden. Was previously known as unk1.

</td>
</tr>
<tr>
<td>

24

</td>
<td>

filetype

</td>
<td>

4

</td>
<td>

This appears to always be 0 on 1st through 4th generation hard
drive-based iPods. For the iTunesDB that is written to the 5th
generation iPod (iPod Video) and the iPod Shuffle, iTunes 4.7.1 (and
greater) writes out the file's type as an ANSI string padded with
spaces. For example, an MP3 file has a filetype of 0x4d503320 -\> 0x4d =
'M', 0x50 = 'P', 0x33 = '3', 0x20 = \<space\>. AAC is 0x41414320 & "new"
AAC which is used by iTunes 7, M4A, is 0x4D344120. Protected AAC files
(purchased from iTunes Store) are M4P = 0x4D345020. Was previously known
as unk2. This really is an integer field and is reversed in iTunesDB
used in mobile phones with reversed endianess.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

type1

</td>
<td>

1

</td>
<td>

CBR MP3s are type 0x00, VBR MP3s are type 0x01, AAC are type 0x00

</td>
</tr>
<tr>
<td>

29

</td>
<td>

type2

</td>
<td>

1

</td>
<td>

CBR MP3s are type 0x01, VBR MP3s are type 0x01, AAC are type 0x00 (type1
and type2 used to be one 2 byte field, but by it doesn't get reversed in
the reversed endian iTunesDB for mobile phones, so it must be two
fields).

</td>
</tr>
<tr>
<td>

30

</td>
<td>

compilation flag

</td>
<td>

1

</td>
<td>

1 if the flag is on, 0 if the flag is off

</td>
</tr>
<tr>
<td>

31

</td>
<td>

stars/rating

</td>
<td>

1

</td>
<td>

the rating of the track \* 20. Note that the iPod does not update this
value here when you change the rating. See the Play Counts file for more
information.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

last modified time

</td>
<td>

4

</td>
<td>

last modified time of the track

</td>
</tr>
<tr>
<td>

36

</td>
<td>

size

</td>
<td>

4

</td>
<td>

size of the track, in bytes

</td>
</tr>
<tr>
<td>

40

</td>
<td>

length

</td>
<td>

4

</td>
<td>

length of the track, in milliseconds

</td>
</tr>
<tr>
<td>

44

</td>
<td>

track number

</td>
<td>

4

</td>
<td>

the track number of the track (the 9 in 9/15)

</td>
</tr>
<tr>
<td>

48

</td>
<td>

total tracks

</td>
<td>

4

</td>
<td>

the total number of tracks on this album (the 15 in 9/15)

</td>
</tr>
<tr>
<td>

52

</td>
<td>

year

</td>
<td>

4

</td>
<td>

year of the track

</td>
</tr>
<tr>
<td>

56

</td>
<td>

bitrate

</td>
<td>

4

</td>
<td>

bitrate of the track (ie, 128, 320, etc)

</td>
</tr>
<tr>
<td>

60

</td>
<td>

sample rate

</td>
<td>

4

</td>
<td>

sample rate of the track (ie. 44100) multiplied by 0x10000.

</td>
</tr>
<tr>
<td>

64

</td>
<td>

volume

</td>
<td>

4

</td>
<td>

Volume adjustment field. This is a value from -255 to 255 that will be
applied to the track on playback. If you adjust the volume slider in
iTunes track info screen, this is what you are adjusting.

</td>
</tr>
<tr>
<td>

68

</td>
<td>

start time

</td>
<td>

4

</td>
<td>

time, in milliseconds, that the song will start playing at

</td>
</tr>
<tr>
<td>

72

</td>
<td>

stop time

</td>
<td>

4

</td>
<td>

time, in milliseconds, that the song will stop playing at

</td>
</tr>
<tr>
<td>

76

</td>
<td>

soundcheck

</td>
<td>

4

</td>
<td>

The SoundCheck value to apply to the song, when SoundCheck is switched
on in the iPod settings. The value to put in this field can be
determined by the equation: X = 1000 \* 10 ^ (-.1 \* Y) where Y is the
adjustment value in dB and X is the value that goes into the SoundCheck
field. The value 0 is special, the equation is not used and it is
treated as "no Soundcheck" (basically the same as the value 1000). This
equation works perfectly well with ReplayGain derived data instead of
the iTunes SoundCheck derived information.

</td>
</tr>
<tr>
<td>

80

</td>
<td>

play count

</td>
<td>

4

</td>
<td>

play count of the song. Note that the iPod does not update this value
here. See the Play Counts file for more information.

</td>
</tr>
<tr>
<td>

84

</td>
<td>

play count 2

</td>
<td>

4

</td>
<td>

Also stores the play count of the song. Don't know if it ever differs
from the above value.

</td>
</tr>
<tr>
<td>

88

</td>
<td>

last played time

</td>
<td>

4

</td>
<td>

time the song was last played. Note that the iPod does not update this
value here. See the Play Counts file for more information.

</td>
</tr>
<tr>
<td>

92

</td>
<td>

disc number

</td>
<td>

4

</td>
<td>

disc number, for multi disc sets

</td>
</tr>
<tr>
<td>

96

</td>
<td>

total discs

</td>
<td>

4

</td>
<td>

total number of discs, for multi disc sets.

</td>
</tr>
<tr>
<td>

100

</td>
<td>

userid

</td>
<td>

4

</td>
<td>

Apple Store/Audible User ID (for DRM'ed files only, set to 0 otherwise).
Previously known as unk5.

</td>
</tr>
<tr>
<td>

104

</td>
<td>

date added

</td>
<td>

4

</td>
<td>

date added to the iPod or iTunes (not certain which)

</td>
</tr>
<tr>
<td>

108

</td>
<td>

bookmark time

</td>
<td>

4

</td>
<td>

the point, in milliseconds, that the track will start playing back at.
This is used for AudioBook filetypes (.AA and .M4B) based on the file
extension. Note that there is also a bookmark value in the play counts
file that will be set by the iPod and can be used instead of this value.
See the Play Counts file for more information.

</td>
</tr>
<tr>
<td>

112

</td>
<td>

dbid

</td>
<td>

8

</td>
<td>

Unique 64 bit value that identifies this song across the databases on
the iPod. For example, this id joins an iTunesDB mhit with a ArtworkDB
mhii. iTunes appears to randomly create this value for a newly formatted
iPod, then increments it by 1 for each additional song added. Previously
known as unk7 and unk8.

</td>
</tr>
<tr>
<td>

120

</td>
<td>

checked

</td>
<td>

1

</td>
<td>

0 if the track is checked, 1 if it is not (in iTunes)

</td>
</tr>
<tr>
<td>

121

</td>
<td>

application rating

</td>
<td>

1

</td>
<td>

This is the rating that the song had before it was last changed, sorta.
If you sync iTunes and the iPod, and they have different (new) ratings,
the rating from iTunes will go here and the iPod rating will take
precedence and go into the normal rating field. I'm uncertain what
exactly this is for, but it's always set to what the iTunes rating is
before each sync.

</td>
</tr>
<tr>
<td>

122

</td>
<td>

BPM

</td>
<td>

2

</td>
<td>

the BPM of the track

</td>
</tr>
<tr>
<td>

124

</td>
<td>

artwork count

</td>
<td>

2

</td>
<td>

The number of album artwork items put into the tags of this song. Even
if you don't put any artwork items into the tags of the song, this value
must at least be 1 for the iPod to display any artwork stored in the
ithmb files.

</td>
</tr>
<tr>
<td>

126

</td>
<td>

unk9

</td>
<td>

2

</td>
<td>

unknown, but always seems to be 0xffff for MP3/AAC songs, 0x0 for
uncompressed songs (like WAVE format), 0x1 for Audible

</td>
</tr>
<tr>
<td>

128

</td>
<td>

artwork size

</td>
<td>

4

</td>
<td>

The total size of artwork (in bytes) attached to this song (i.e. put
into the song as tags). Observed in iPodDB version 0x0b and with an iPod
Photo as well as with iPodDB version 0x0d and an iPod Nano.

</td>
</tr>
<tr>
<td>

132

</td>
<td>

unk11

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

136

</td>
<td>

sample rate 2

</td>
<td>

4

</td>
<td>

The sample rate of the song expressed as an IEEE 32 bit floating point
number. It's uncertain why this is here.

</td>
</tr>
<tr>
<td>

140

</td>
<td>

date released

</td>
<td>

4

</td>
<td>

date/time added to music store? For podcasts this corresponds to the
release date as displayed to the right of the podcast title. Formerly
known as unk13.

</td>
</tr>
<tr>
<td>

144

</td>
<td>

unk14/1

</td>
<td>

2

</td>
<td>

unknown, but MPEG-1 Layer-3 songs appear to be always 0x000c, MPEG-2
Layer 3 songs (extrem low bitrate) appear to be 0x0016, MPEG-2.5 Layer 3
songs are 0x0020, AAC songs are always 0x0033, Audible files are 0x0029,
WAV files are 0x0000.

</td>
</tr>
<tr>
<td>

146

</td>
<td>

unk14/2

</td>
<td>

2

</td>
<td>

probably 1 if played on or more times in iTunes and 0 otherwise (at
least for MP3 -- the value has been observed to be always 1 for AAC and
Audible files, and always 0 for WAV files?)

</td>
</tr>
<tr>
<td>

148

</td>
<td>

unk15

</td>
<td>

4

</td>
<td>

unknown - used for Apple Store DRM songs (always 0x01010100?), zero
otherwise

</td>
</tr>
<tr>
<td>

152

</td>
<td>

unk16

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

156

</td>
<td>

Skip Count

</td>
<td>

4

</td>
<td>

Number of times the track has been skipped. Formerly unknown 17 (added
in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

160

</td>
<td>

Last Skipped

</td>
<td>

4

</td>
<td>

Date/time last skipped. Formerly unknown 18 (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

164

</td>
<td>

has\_artwork

</td>
<td>

1

</td>
<td>

added in dbversion 0xd. Seems to be set to 0x02 for tracks without
associated artwork (even if artwork is present, it will not be shown on
the iPod) and 0x01 for tracks with associated artwork.

</td>
</tr>
<tr>
<td>

165

</td>
<td>

skip\_when\_shuffling

</td>
<td>

1

</td>
<td>

sets "Skip When Shuffling" when set to 0x1 (added in dbversion 0xd,
formerly known as flag2)

</td>
</tr>
<tr>
<td>

166

</td>
<td>

remember\_playback\_position

</td>
<td>

1

</td>
<td>

sets "Remember Playback Position" when set to 0x1 (added in dbversion
0xd). Note that Protected AAC files (.m4b extension) and Audible files
(.aa extension) do not set this flag or the previous one
(skip\_when\_shuffling), and yet are always bookmarkable and are never
included in the song shuffle. To determine if a file is bookmarkable,
therefore, check the file type first. If it's not an .m4b or .aa, then
check this flag in iTunesDB. (Formerly known as flag3)

</td>
</tr>
<tr>
<td>

167

</td>
<td>

flag4

</td>
<td>

1

</td>
<td>

some kind of "Podcast" flag (added in dbversion 0xd)? When this flag is
set to 0x1 then the "Now playing" page will not show the artist name,
but only title and album. When additionally has\_artwork is 0x2 then
there will be a new sub-page on the "Now playing" page with information
about the podcast/song.

If the track item is a kind of podcast then this flag must be set to 0x1
or 0x2, otherwise this flag must be set to 0x0. If this flag do not
follow this, it might be removed from iTunesDB when user change there
iPod to sync podcasts/songs in iTunes.

</td>
</tr>
<tr>
<td>

168

</td>
<td>

dbid2

</td>
<td>

8

</td>
<td>

Until dbversion 0x12, same data as dbid above (added in dbversion 0x0c).
Since 0x12, this field value differs from the dbid one.

</td>
</tr>
<tr>
<td>

176

</td>
<td>

lyrics flag

</td>
<td>

1

</td>
<td>

set to 0x01 if lyrics are stored in the MP3 tags ("USLT"), 0 otherwise.

</td>
</tr>
<tr>
<td>

177

</td>
<td>

movie file flag

</td>
<td>

1

</td>
<td>

if 0x1, it is a movie file. Otherwise, it is an audio file.

</td>
</tr>
<tr>
<td>

178

</td>
<td>

played\_mark

</td>
<td>

1

</td>
<td>

added in dbversion 0x0c, first values observed in 0x0d. Observed to be
0x01 for non-podcasts. With podcasts, a value of 0x02 marks this track
with a bullet as 'not played' on the iPod, irrespective of the value of
play count above. A value of 0x01 removes the bullet. Formerly known as
unk20.

</td>
</tr>
<tr>
<td>

179

</td>
<td>

unk17

</td>
<td>

1

</td>
<td>

unknown - added in dbversion 0x0c. So far always 0.

</td>
</tr>
<tr>
<td>

180

</td>
<td>

unk21

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

184

</td>
<td>

pregap

</td>
<td>

4

</td>
<td>

Number of samples of silence before the songs starts (for gapless
playback).

</td>
</tr>
<tr>
<td>

188

</td>
<td>

sample count

</td>
<td>

8

</td>
<td>

Number of samples in the song (for gapless playback).

</td>
</tr>
<tr>
<td>

196

</td>
<td>

unk25

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

200

</td>
<td>

postgap

</td>
<td>

4

</td>
<td>

Number of samples of silence at the end of the song (for gapless
playback).

</td>
</tr>
<tr>
<td>

204

</td>
<td>

unk27

</td>
<td>

4

</td>
<td>

unknown - added in dbversion 0x0c, first values observed in 0x0d.
Appears to be 0x1 for files encoded using the MP3 encoder, 0x0
otherwise.

</td>
</tr>
<tr>
<td>

208

</td>
<td>

Media Type

</td>
<td>

4

</td>
<td>

(formerly known as unk28; added in dbversion 0x0c). It seems that this
field denotes the type of the file on (e.g.) the 5g video iPod. It must
be set to 0x00000001 for audio files, and set to 0x00000002 for video
files. If set to 0x00, the files show up in both, the audio menus
("Songs", "Artists", etc.) and the video menus ("Movies", "Music
Videos", etc.). It appears to be set to 0x20 for music videos, and if
set to 0x60 the file shows up in "TV Shows" rather than "Movies".  

The following list summarizes all observed types:

-   0x00 00 00 00 - Audio/Video
-   0x00 00 00 01 - Audio
-   0x00 00 00 02 - Video
-   0x00 00 00 04 - Podcast
-   0x00 00 00 06 - Video Podcast
-   0x00 00 00 08 - Audiobook
-   0x00 00 00 20 - Music Video
-   0x00 00 00 40 - TV Show (shows up ONLY in TV Shows
-   0x00 00 00 60 - TV Show (shows up in the Music lists as well)

<b>Caution:</b> Even if a track is marked as "Audiobook" here (value
0x08), it will <b>not</b> show up in the "Audiobooks" menu on the iPod.
Only \*.aa and \*.m4b are shown there by recent firmwares. One proven
exception: On the <i>nano</i> they show if they have the correct media
type set here <b>and</b> the MHIT also has a
<a href="http://www.ipodlinux.org/ITunesDB/#Chapter_Data_.28Type_17.29" title="">chapter
data mhod</a>!

</td>
</tr>
<tr>
<td>

212

</td>
<td>

season number

</td>
<td>

4

</td>
<td>

the season number of the track, for TV shows only. Previously known as
unk29. (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

216

</td>
<td>

episode number

</td>
<td>

4

</td>
<td>

the episode number of the track, for TV shows only - although not
displayed on the iPod, the episodes are sorted by episode number.
Previously known as unk30. (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

220

</td>
<td>

unk31

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c). Has something to do with protected
files - set to 0x0 for non-protected files.

</td>
</tr>
<tr>
<td>

224

</td>
<td>

unk32

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

228

</td>
<td>

unk33

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

232

</td>
<td>

unk34

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

236

</td>
<td>

unk35

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

240

</td>
<td>

unk36

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

244

</td>
<td>

unk37

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x13)

</td>
</tr>
<tr>
<td>

248

</td>
<td>

gaplessData

</td>
<td>

4

</td>
<td>

The size in bytes from first Synch Frame (which is usually the XING
frame that includes the LAME tag) until the 8th before the last frame.
The gapless playback does not work for MP3 files if this is set to zero.
Maybe the iPod prepares the next track when rest 8 frames in the actual
track. For AAC tracks, this may be zero. (added in dbversion 0x13)

</td>
</tr>
<tr>
<td>

252

</td>
<td>

unk38

</td>
<td>

4

</td>
<td>

unknown (added in dbversion 0x0c)

</td>
</tr>
<tr>
<td>

256

</td>
<td>

gaplessTrackFlag

</td>
<td>

2

</td>
<td>

if 1, this track has gapless playback data (added in dbversion 0x13)

</td>
</tr>
<tr>
<td>

258

</td>
<td>

gaplessAlbumFlag

</td>
<td>

2

</td>
<td>

if 1, this track does not use crossfading in iTunes (added in dbversion
0x13)

</td>
</tr>
<tr>
<td>

260

</td>
<td>

unk39

</td>
<td>

20

</td>
<td>

Appears to be a hash, not checked by the iPod

</td>
</tr>
<tr>
<td>

288

</td>
<td>

unk40

</td>
<td>

4

</td>
<td>

unknown (seen set to 0xbf)

</td>
</tr>
<tr>
<td>

300

</td>
<td>

unk41

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

304

</td>
<td>

unk42

</td>
<td>

4

</td>
<td>

unknown (always 0x00 ?)

</td>
</tr>
<tr>
<td>

308

</td>
<td>

unk43

</td>
<td>

4

</td>
<td>

unknown (previously length 8, seen as 0x818080808080)

</td>
</tr>
<tr>
<td>

312

</td>
<td>

unk44

</td>
<td>

2

</td>
<td>

unknown (previously length 8, seen as 0x818080808080)

</td>
</tr>
<tr>
<td>

314

</td>
<td>

AlbumID

</td>
<td>

2

</td>
<td>

album id from the album list (previously unknown length 8, seen as
0x818080808080)

</td>
</tr>
<tr>
<td>

352

</td>
<td>

mhii-link

</td>
<td>

4

</td>
<td>

Setting this offset to != 0 triggers the 'Right-Pane-Artwork-Slideshow'
on late 2007 iPods (3g Nano) and causes the iPod to use this value to do
artwork lookups (dbid\_1 will be ignored!). This value should be set to
the id of the corresponding ArtworkDB mhii (Offset 16)

</td>
</tr>
<tr>
<td colspan="4">

The rest of the mhit is zero padded.

</td>
</tr>
</tbody>
</table>

The MHIT is followed by several Data Objects which have string types. At
minimum, it must have a Location type MHOD, in order to tell the iPod
where the file is located on the iPod itself. It always has a FileType
MHOD as well, although it's not totally necessary.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Playlist_List"></a>

<h3>

Playlist List

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhlp format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhlp

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhlp header

</td>
</tr>
<tr>
<td>

8

</td>
<td>

number of playlists

</td>
<td>

4

</td>
<td>

the number of playlists on the iPod. This includes the Library playlist.

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The Playlist List has all the playlists as its children. The very first
playlist \*must\* be the Library playlist. This is a normal playlist,
but it has the special "hidden" bit set in it, and it contains all the
songs on the iPod (in no particular order).

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Playlist"></a>

<h3>

Playlist

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhyp format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhyp

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhyp header

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

Data Object Child Count

</td>
<td>

4

</td>
<td>

number of Data Objects in the List

</td>
</tr>
<tr>
<td>

16

</td>
<td>

Playlist Item Count

</td>
<td>

4

</td>
<td>

number of Playlist Items in the List

</td>
</tr>
<tr>
<td>

20

</td>
<td>

Is Master Playlist flag

</td>
<td>

1

</td>
<td>

1 if the playlist is the Master (Library) playlist, 0 if it is not. Only
the Master (Library) Playlist should have a 1 here.

</td>
</tr>
<tr>
<td>

21

</td>
<td>

unk

</td>
<td>

3

</td>
<td>

Probably three more flags, the first of which has been observed to have
been set to 1 for some playlists.

</td>
</tr>
<tr>
<td>

24

</td>
<td>

timestamp

</td>
<td>

4

</td>
<td>

time of the playlists creation

</td>
</tr>
<tr>
<td>

28

</td>
<td>

persistent playlist ID

</td>
<td>

8

</td>
<td>

a unique, randomly generated ID for the playlist

</td>
</tr>
<tr>
<td>

36

</td>
<td>

unk3

</td>
<td>

4

</td>
<td>

Always zero?

</td>
</tr>
<tr>
<td>

40

</td>
<td>

String MHOD Count

</td>
<td>

2

</td>
<td>

This appears to be the number of string MHODs (type \< 50) associated
with this playlist (typically 0x01). Doesn't seem to be signficant
unless you include Type 52 MHODs. Formerly known as unk4.

</td>
</tr>
<tr>
<td>

42

</td>
<td>

Podcast Flag

</td>
<td>

2

</td>
<td>

This is set to 0 on normal playlists and to 1 for the Podcast playlist.
If set to 1, the playlist will not be shown under 'Playlists' on the
iPod, but as 'Podcasts' under the Music menu.. The actual title of the
Playlist does not matter. If more than one playlist is set to 1, they
don't show up at all. They also don't show up if you set this to 1 on
one playlist and 2 on the other. Please note that podcast playlists are
organized slightly different than ordinary playlists (see below).

</td>
</tr>
<tr>
<td>

44

</td>
<td>

List Sort Order

</td>
<td>

4

</td>
<td>

The field that the playlist will be sorted by. See list below.

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The structure of the Playlists are different than most others. Each
Playlist looks like this, conceptually:

       &lt;mhyp&gt;
          &lt;mhod type=1&gt;Playlist Name&lt;/mhod&gt;
          &lt;mhod type=50&gt;Smart Playlist Info&lt;/mhod&gt; (optional)
          &lt;mhod type=51&gt;Smart Playlist Rules&lt;/mhod&gt; (optional)
          ...
          &lt;mhip&gt;Playlist Item&lt;/mhip&gt;
                 &lt;mhod type=100&gt;Position Indicator&lt;/mhod&gt;
          &lt;mhip&gt;Playlist Item&lt;/mhip&gt;
                 &lt;mhod type=100&gt;Position Indicator&lt;/mhod&gt;
          ...
       &lt;/mhyp&gt;

The point being that these are all considered in the MHYP for the size
calculations. However, in the "Data Object" child count, ONLY those
MHODs that come before the first MHIP are counted. The "position
indicators" are children of the MHIPs (older firmwares had a bug in this
respect).

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="List_Sort_Order"></a>

<h4>

List Sort Order

</h4>

    1 - playlist order (manual sort order)
    2 -&nbsp;???
    3 - songtitle
    4 - album
    5 - artist
    6 - bitrate
    7 - genre
    8 - kind
    9 - date modified
    10 - track number
    11 - size
    12 - time
    13 - year
    14 - sample rate
    15 - comment
    16 - date added
    17 - equalizer
    18 - composer
    19 -&nbsp;???
    20 - play count
    21 - last played
    22 - disc number
    23 - my rating
    24 - release date (I guess, it's the value for the "Podcasts" list)
    25 - BPM
    26 - grouping
    27 - category
    28 - description
    29 - show
    30 - season
    31 - episode number

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Podcasts"></a>

<h4>

Podcasts

</h4>

The podcasts playlist is organized slightly differently, in the Type 3
MHSD. For example:

    mhyp (MHODs: 2, MHIPs: 5, hidden: 0, list sort order: 0x18)
      mhod (type: 1, string: 'Podcasts')
      mhod (type: 100)
      mhip (MHODs: 1, groupflag: 256, groupid: 8232, trackid: 0, timestamp: 0, groupref: 0)
        mhod (type: 1, string: 'Example podcast')
      mhip (MHODs: 1, groupflag: 0, groupid: 8233, trackid: 8230, timestamp: 3206828281, groupref: 8232)
        mhod (type: 100)
      mhip (MHODs: 1, groupflag: 0, groupid: 8234, trackid: 8226, timestamp: 3206828379, groupref: 8232)
        mhod (type: 100)
      mhip (MHODs: 1, groupflag: 0, groupid: 8235, trackid: 8228, timestamp: 3206828327, groupref: 8232)
        mhod (type: 100)
      mhip (MHODs: 1, groupflag: 0, groupid: 8236, trackid: 8224, timestamp: 3206828394, groupref: 8232)
        mhod (type: 100)

The first mhip (probably identified by groupflag==256) contains the name
of a podcast which will then appear as a submenu of the Podcasts menu on
the iPod. The other mhips (which reference the episodes in that podcast)
seem to use the groupref field to link to their 'parent' mhip (using the
groupid field). At the same time the groupids of the episodes are unique
as well, but don't seem to be used anywhere else.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Playlist_Item"></a>

<h3>

Playlist Item

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhip format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhip

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhip header

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

Data Object Child Count

</td>
<td>

4

</td>
<td>

number of mhod following this Playlist Item (always 1 so far)

</td>
</tr>
<tr>
<td>

16

</td>
<td>

Podcast Grouping Flag

</td>
<td>

2

</td>
<td>

Formerly unk1. 0x0 = normal file. 0x100 = Podcast Group. Podcast Groups
will be followed by a single child, an MHOD type 1 string, which
specifies the name of the Podcast Group. They will also have a 0 (zero)
for the Track ID. This field used to be some kind of correlation ID or
something, but this may have been a bug. In any case, the old way breaks
iPods now, and this should be set to zero on all normal songs.

</td>
</tr>
<tr>
<td>

18

</td>
<td>

unk4

</td>
<td>

1

</td>
<td>

0 or 1 in iTunes 7.2.

</td>
</tr>
<tr>
<td>

19

</td>
<td>

unk5

</td>
<td>

1

</td>
<td>

0 or 8 in iTunes 7.2.

</td>
</tr>
<tr>
<td>

20

</td>
<td>

Group ID (?)

</td>
<td>

4

</td>
<td>

Formerly unk2. A unique ID for the track. It appears it is made sure
that this ID does not correspond to any real track ID. Doesn't seem to
correlate to anything, but other bits reference it. See Podcast Grouping
Reference below.

</td>
</tr>
<tr>
<td>

24

</td>
<td>

track ID

</td>
<td>

4

</td>
<td>

the ID number of the track in the track list. See Track Item for more
info

</td>
</tr>
<tr>
<td>

28

</td>
<td>

timestamp

</td>
<td>

4

</td>
<td>

some kind of time stamp, possibly time the song was added to the
playlist

</td>
</tr>
<tr>
<td>

32

</td>
<td>

Podcast Grouping Reference

</td>
<td>

4

</td>
<td>

Formerly unk3. This is the parent group that this podcast should be
listed under. It should be zero the rest of the time.

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

For purposes of size calculations, Playlist Items have no children.
However, every Playlist Item is invariably followed by a Data Object of
type 100, which contains nothing but a number that is used to order/sort
the playlist. See the Playlist description above for more information.

Please note that starting with iTunes 4.9 (mhbd file version number
0x0d) the Type 100 MHOD following the Playlist Item is considered a
child of the Playlist Item and is included into the size calculation.
The old behaviour was probably a bug in iTunes.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Data_Object"></a>

<h3>

Data Object

</h3>

The Data Object is a complex beast. It's used in many places in the
iTunesDB file, and there are many forms of it. We'll cover the forms one
by one.

<table border="1" cellpadding="1" cellspacing="0">
<caption>

<b>mhod types</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

value

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

1

</td>
<td>

Title

</td>
</tr>
<tr>
<td>

2

</td>
<td>

Location (this string should be less than 112 bytes/56 UTF-16 chars (not
including the terminating \\0) or the iPod will skip the song when
trying to play it)

</td>
</tr>
<tr>
<td>

3

</td>
<td>

Album

</td>
</tr>
<tr>
<td>

4

</td>
<td>

Artist

</td>
</tr>
<tr>
<td>

5

</td>
<td>

Genre

</td>
</tr>
<tr>
<td>

6

</td>
<td>

Filetype

</td>
</tr>
<tr>
<td>

7

</td>
<td>

EQ Setting

</td>
</tr>
<tr>
<td>

8

</td>
<td>

Comment

</td>
</tr>
<tr>
<td>

9

</td>
<td>

Category - This is the category ("Technology", "Music", etc.) where the
podcast was located. Introduced in db version 0x0d.

</td>
</tr>
<tr>
<td>

12

</td>
<td>

Composer

</td>
</tr>
<tr>
<td>

13

</td>
<td>

Grouping

</td>
</tr>
<tr>
<td>

14

</td>
<td>

Description text (such as podcast show notes). Accessible by selecting
the center button on the iPod, where this string is displayed along with
the song title, date, and timestamp. Introduced in db version 0x0d.

</td>
</tr>
<tr>
<td>

15

</td>
<td>

Podcast Enclosure URL. Note: this is either a UTF-8 or ASCII encoded
string (NOT UTF-16). Also, there is no mhod::length value for this type.
Introduced in db version 0x0d.

</td>
</tr>
<tr>
<td>

16

</td>
<td>

Podcast RSS URL. Note: this is either a UTF-8 or ASCII encoded string
(NOT UTF-16). Also, there is no mhod::length value for this type.
Introduced in db version 0x0d.

</td>
</tr>
<tr>
<td>

17

</td>
<td>

Chapter data. This is a m4a-style entry that is used to display subsongs
within a mhit. Introduced in db version 0x0d.

</td>
</tr>
<tr>
<td>

18

</td>
<td>

Subtitle (usually the same as Description). Introduced in db version
0x0d.

</td>
</tr>
<tr>
<td>

19

</td>
<td>

Show (for TV Shows only). Introduced in db version 0x0d?

</td>
</tr>
<tr>
<td>

20

</td>
<td>

Episode \# (for TV Shows only). Introduced in db version 0x0d?

</td>
</tr>
<tr>
<td>

21

</td>
<td>

TV Network (for TV Shows only). Introduced in db version 0x0d?

</td>
</tr>
<tr>
<td>

22

</td>
<td>

Album Artist. Introduced in db version 0x13?

</td>
</tr>
<tr>
<td>

23

</td>
<td>

Artist name, for sorting. Artists with names like "The Beatles" will be
in here as "Beatles, The". Introduced in db version 0x13?

</td>
</tr>
<tr>
<td>

24

</td>
<td>

Appears to be a list of keywords pertaining to a track. Introduced in db
version 0x13?

</td>
</tr>
<tr>
<td>

25

</td>
<td>

Locale for TV show? (e.g. "us-tv\|\|0\|", v.0x18)

</td>
</tr>
<tr>
</tr>
<tr>
<td>

27

</td>
<td>

Title, for sorting.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

Album, for sorting.

</td>
</tr>
<tr>
<td>

29

</td>
<td>

Album-Artist, for sorting.

</td>
</tr>
<tr>
<td>

30

</td>
<td>

Composer, for sorting.

</td>
</tr>
<tr>
<td>

31

</td>
<td>

TV-Show, for sorting.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

Unknown, created by iTunes 7.1 for video tracks. Binary field, no
string.

</td>
</tr>
<tr>
<td>

50

</td>
<td>

Smart Playlist Data

</td>
</tr>
<tr>
<td>

51

</td>
<td>

Smart Playlist Rules

</td>
</tr>
<tr>
<td>

52

</td>
<td>

Library Playlist Index

</td>
</tr>
<tr>
<td>

53

</td>
<td>

Unknown, created by iTunes 7.2, similar to MHOD52

</td>
</tr>
<tr>
<td>

100

</td>
<td>

Seems to vary. iTunes uses it for column sizing info as well as an order
indicator in playlists.

</td>
</tr>
<tr>
<td>

200

</td>
<td>

Album (in Album List, iTunes 7.1)

</td>
</tr>
<tr>
<td>

201

</td>
<td>

Artist (in Album List, iTunes 7.1)

</td>
</tr>
<tr>
<td>

202

</td>
<td>

Artist, for sorting (in Album List, iTunes 7.1)

</td>
</tr>
<tr>
<td>

203

</td>
<td>

Podcast Url (in Album List, iTunes 7.1)

</td>
</tr>
<tr>
<td>

204

</td>
<td>

TV Show (in Album List, v. 0x18)

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="String_Types"></a>

<h4>

String Types

</h4>

The simplest form of MHOD. These are any MHOD with a "type" that is less
than 15.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - strings</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header. This is always 0x18 for string type MHOD's.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and the string it contains

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

24

</td>
<td>

position

</td>
<td>

4

</td>
<td>

In type 100 mhod's in playlists, this is where the playlist order info
is. It does not seem to be significant in string mhod's (except for
location - see following notes).

<i>Note:</i> This field does not exist in ArtworkDB string dohms. This
was observed to be 2 for inversed endian ordered iTunesDBs for mobile
phones with UTF8 strings and 1 for standard iPod iTunesDBs with UTF16
strings.

<i>Note:</i> If you leave this set to zero on the type 2 (location)
string MHOD of a Song (mhit) record, the track will show on the menu,
but will not play.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

length

</td>
<td>

4

</td>
<td>

Length of the string, in bytes. If the string is UTF-16, each char takes
two bytes. The string in the iTunesDB is not NULL-terminated either.
Keep this in mind. Be careful with very long strings - it has been
observed that strings longer than \~512 characters will cause the iPod
to continously reboot when it attempts to read the database.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

unknown. It was thought that this was string encoding: 0 == UTF-16, 1 ==
UTF-8, however, recent iTunesDB files have had this set to 1 even with
UTF-16 strings. Therefore this is definitely incorrect, and the correct
meaning has not yet been discovered yet.

</td>
</tr>
<tr>
<td>

36

</td>
<td>

unk4

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

40

</td>
<td>

string

</td>
<td>

length

</td>
<td>

The string.

</td>
</tr>
<tr>
<td colspan="4">

string mhods are NOT zero padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Podcast_URLs_.28types_15-16.29"></a>

<h4>

Podcast URLs (types 15-16)

</h4>

Introduced in db version 0x0d, MHOD's with type 15 and 16 hold the
Enclosure and RSS URL for the Podcast. The string is probably UTF-8, but
only Unicode symbols U+0000 through U+007F (a.k.a ASCII) have been
observed.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - podcast urls (types 15-16)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header. This is always 0x18 for string type MHOD's.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and the string it contains

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 15 or 16 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown (always 0?)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown (always 0?)

</td>
</tr>
<tr>
<td>

24

</td>
<td>

string

</td>
<td>

(total length - header length)

</td>
<td>

The string.

</td>
</tr>
<tr>
<td colspan="4">

string mhods are NOT zero padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_.28type_17.29"></a>

<h4>

Chapter Data (type 17)

</h4>

The chapter data is a style of data that's very, very out of place in
this file, but is here nonetheless. It defines where the chapter stops
are in the track, as well as what info should be displayed for each
section of the track. It seems that the format of this mhod changed
significantly over time. The following is analysed from an iTunesDB
version 0x13 (iTunes 7.0).

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_mhod_details"></a>

<h5>

Chapter Data mhod details

</h5>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - chapter data (type 17)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header. This is always 0x18.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the mhod

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 0x11 = 17 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown (always 0?)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown (always 0?)

</td>
</tr>
<tr>
<td>

24

</td>
<td>

unk3

</td>
<td>

4

</td>
<td>

unknown (always 0?) This is not part of the header (otherwise the header
length would have been different), but it is also not part of the first
atom.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unk4

</td>
<td>

4

</td>
<td>

unknown (always 0?) This is not part of the header (otherwise the header
length would have been different), but it is also not part of the first
atom.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

unk5

</td>
<td>

4

</td>
<td>

unknown (always 0?) This is not part of the header (otherwise the header
length would have been different), but it is also not part of the first
atom.

</td>
</tr>
<tr>
<td>

36

</td>
<td>

data

</td>
<td>

(total length - header length - 12)

</td>
<td>

The chapter stop data atoms, starting with a "sean" atom (see below).

This part of the mhod is <b>not little-endian</b> like the rest of the
file. It's big-endian/ E.g. the value 0x0123 in a 4 byte word is found
as 00 00 01 23. It is also arranged in a tree like structure, much like
the iTunesDB itself is, only the information is in a slightly different
arrangement.

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_Atom_Layout"></a>

<h5>

Chapter Data Atom Layout

</h5>

The atoms appear to be arranged in the following layout:

    <b>sean</b> (exactly one atom of this type is present,
            childcount is number of chapters + 1 for the hedr atom)
       <b>chap</b> (chapter indicator, childcount is always 1)
          <b>name</b> (contains a UTF16 string with name of the chapter or Artist name, 
                childcount is always 0)
       chap (next chapter)
          name (and so on)...
       ...
       <b>hedr</b> (signals the end, childcount is always 0)

There are multiple "chap" entries, one for each chapter.

Older DBs seem to have a different, more complex layout, like this:

    sean (?)
     chap (chapter indicator&nbsp;?)
      name (contains a UTF16 string with name of the chapter or Artist name)
      ploc (position/location?)
       trak (specifies track number, perhaps? This should be obvious after looking at some examples)
      urlt (contains a UTF16 string with the name of the song in the chapter or some kind of subname)
      url  (contains a UTF8 string with a URL)
     chap (next chapter)
      name (and so on)...
     ...
    hedr (signals the end)

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_Atoms"></a>

<h5>

Chapter Data Atoms

</h5>

Each atom consists of the following:

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>chapter data atom</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

size

</td>
<td>

4

</td>
<td>

size of the atom and all its children  

    <b>Caution:</b> This is different to all other structures in the file, where the
             name always comes first. Here, the length is the first word of the atom!

</td>
</tr>
<tr>
<td>

4

</td>
<td>

atom name

</td>
<td>

4

</td>
<td>

the name of the atom

</td>
</tr>
<tr>
<td>

8

</td>
<td>

startpos/unk1

</td>
<td>

4

</td>
<td>

<i>For chap atoms:</i> the starting position of this chapter in ms,
except for the very first chap where this field is 1 (not 0 as
expected).  

<i>For all other atoms:</i> always 1?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of childs

</td>
<td>

4

</td>
<td>

the number of childs

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

always 0?

</td>
</tr>
<tr>
<td>

20

</td>
<td>

data

</td>
<td>

varies

</td>
<td>

some kind of data or children

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_String_Atoms_.28UTF16.29"></a>

<h5>

Chapter Data String Atoms (UTF16)

</h5>

UTF16 String entries in these atoms (like 'name') fit the following
mold:

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>chapter data atom - UTF16 string entries</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

size

</td>
<td>

4

</td>
<td>

size of the atom and all its children (0x16 + 2\*string length in
characters, e.g. for a 8 char string this is 0x26)

</td>
</tr>
<tr>
<td>

4

</td>
<td>

atom name

</td>
<td>

4

</td>
<td>

the name of the atom

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

always 1?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

always 0? (child count)

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

always 0?

</td>
</tr>
<tr>
<td>

20

</td>
<td>

string length

</td>
<td>

2

</td>
<td>

string length in characters

</td>
</tr>
<tr>
<td>

22

</td>
<td>

string

</td>
<td>

length \*2

</td>
<td>

UTF16 string, 2 bytes per character

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_String_Atoms_.28UTF8.29"></a>

<h5>

Chapter Data String Atoms (UTF8)

</h5>

UTF8 entries fit the following mold:

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>chapter data atom - UTF8 string entries</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

size

</td>
<td>

4

</td>
<td>

size of the atom and all its children

</td>
</tr>
<tr>
<td>

4

</td>
<td>

atom name

</td>
<td>

4

</td>
<td>

the name of the atom

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

always 1?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

varies.. some kind of type?

</td>
</tr>
<tr>
<td>

16

</td>
<td>

null

</td>
<td>

8

</td>
<td>

zeros

</td>
</tr>
<tr>
<td>

24

</td>
<td>

string

</td>
<td>

size - 24

</td>
<td>

UTF8 string, 1 byte per character

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Chapter_Data_hedr_Atom"></a>

<h5>

Chapter Data hedr Atom

</h5>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>chapter data atom - hedr</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

size

</td>
<td>

4

</td>
<td>

size of the atom, 0x1c

</td>
</tr>
<tr>
<td>

4

</td>
<td>

atom name

</td>
<td>

4

</td>
<td>

the name of the atom, 'hedr'

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

always 1?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of childs

</td>
<td>

4

</td>
<td>

the number of childs, always 0

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

always 0

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk3

</td>
<td>

4

</td>
<td>

always 0

</td>
</tr>
<tr>
<td>

24

</td>
<td>

unk4

</td>
<td>

4

</td>
<td>

always 1

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Smart_Playlist_Data_.28type_50.29"></a>

<h4>

Smart Playlist Data (type 50)

</h4>

A slightly more complex MHOD. These are any MHOD with a "type" that is
50.

This MHOD defines the stuff in the Smart playlist that is not the
"rules". Basically all the checkboxes and such. It's pretty
straightforward.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - Smart Playlist Data (type 50)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

total length of the mhod

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 50 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

24

</td>
<td>

live update

</td>
<td>

1

</td>
<td>

Live Update flag. 0x01 = on, 0x00 = off

</td>
</tr>
<tr>
<td>

25

</td>
<td>

check rules

</td>
<td>

1

</td>
<td>

Rules enable flag. 0x01 = on, 0x00 = off. When this is enabled, Rules
from the type 51 MHOD will be used.

</td>
</tr>
<tr>
<td>

26

</td>
<td>

check limits

</td>
<td>

1

</td>
<td>

Limits enable flag. 0x01 = on, 0x00 = off. When this is enabled, Limits
listed below will actually be used.

</td>
</tr>
<tr>
<td>

27

</td>
<td>

limit type

</td>
<td>

1

</td>
<td>

Limit Type. See below for the list of limit types.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

limit sort

</td>
<td>

1

</td>
<td>

Limit Sort. See below for the list of limit sorting types.

</td>
</tr>
<tr>
<td>

29

</td>
<td>

unknown

</td>
<td>

3

</td>
<td>

always zero bytes

</td>
</tr>
<tr>
<td>

32

</td>
<td>

limit value

</td>
<td>

4

</td>
<td>

The actual value used for the limit

</td>
</tr>
<tr>
<td>

36

</td>
<td>

match checked only

</td>
<td>

1

</td>
<td>

match checked only flag, 0x01 = on, 0x00 = off. When this is enabled,
only songs marked as "checked" will be matched. Checked is a field in
the mhit.

</td>
</tr>
<tr>
<td>

37

</td>
<td>

reverse limit sort

</td>
<td>

1

</td>
<td>

Reverse the Limit Sort flag. 0x01 = on, 0x00 = off. When this is
enabled, the sort will be reversed. More on this below.

</td>
</tr>
<tr>
<td colspan="4">

the mhod IS zero padded at the end (58 null bytes)

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="1" cellspacing="0">
<caption>

<b>Limit Types</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

value

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

1

</td>
<td>

Minutes

</td>
</tr>
<tr>
<td>

2

</td>
<td>

Megabytes

</td>
</tr>
<tr>
<td>

3

</td>
<td>

Songs

</td>
</tr>
<tr>
<td>

4

</td>
<td>

Hours

</td>
</tr>
<tr>
<td>

5

</td>
<td>

Gigabytes

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="1" cellspacing="0">
<caption>

<b>Limit Sort Types</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

value

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

0x02

</td>
<td>

Random

</td>
</tr>
<tr>
<td>

0x03

</td>
<td>

Song Name (alphabetical)

</td>
</tr>
<tr>
<td>

0x04

</td>
<td>

Album (alphabetical)

</td>
</tr>
<tr>
<td>

0x05

</td>
<td>

Artist (alphabetical)

</td>
</tr>
<tr>
<td>

0x07

</td>
<td>

Genre (alphabetical)

</td>
</tr>
<tr>
<td>

0x10

</td>
<td>

Most Recently Added

</td>
</tr>
<tr>
<td>

0x14

</td>
<td>

Most Often Played

</td>
</tr>
<tr>
<td>

0x15

</td>
<td>

Most Recently Played

</td>
</tr>
<tr>
<td>

0x17

</td>
<td>

Highest Rating

</td>
</tr>
</tbody>
</table>

When the Reverse Limit Sort flag is set, the sort will be reversed. So
most recently added becomes least recently added, and highest rating
becomes lowest rating, and so on. It's just reversing the sorted list
before applying the limit to it.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Smart_Playlist_Rules_.28type_51.29"></a>

<h4>

Smart Playlist Rules (type 51)

</h4>

The most complex (and annoying) MHOD. These are any MHOD with a "type"
that is 51.

This MHOD defines all the rules in the Smart Playlist. It is not
straightforward at all. In fact, I have to break it down into subtypes
itself.

If you've used iTunes, you know what a rule looks like. "Rating is less
than 3 stars", for example. The rule consists of three parts.

The first part is the "field". In our example, the field is "Rating".
It's the very first thing in the rule and the first pull down box in
iTunes. Simple enough.

The second part is the "action". In our example, the action is "is less
than". In iTunes, it's the second pull down box. Also pretty simple.

The final part is the "value". In our example, it'd be "3 stars". Since
stars are always multiplied by 20 for whatever reason, it'd be "60" for
our example, in the iTunesDB file.

We'll cover these one at a time.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Important_Note_about_endian-ness"></a>

<h5>

Important Note about endian-ness

</h5>

Smart Playlist Rules MHODs are NOT wholly little-endian. Everything
after the "SLst" to the end of the MHOD is big-endian. This is important
to remember, especially when dealing with the Action types.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Smart_Playlist_Rule_Fields"></a>

<h5>

Smart Playlist Rule Fields

</h5>

Fields are easy. Much more so than Actions. The "expected comparison"
column will make more sense when you see the Values explained below.

<table border="1" cellpadding="1" cellspacing="0">
<caption>

<b>Smart Playlist Rule Field Types</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

value

</th>
<th style="background:#efefef;">

description

</th>
<th style="background:#efefef;">

expected comparison

</th>
</tr>
<tr>
<td>

0x02

</td>
<td>

Song Name

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x03

</td>
<td>

Album

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x04

</td>
<td>

Artist

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x05

</td>
<td>

Bitrate

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x06

</td>
<td>

Sample Rate

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x07

</td>
<td>

Year

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x08

</td>
<td>

Genre

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x09

</td>
<td>

Kind

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x0a

</td>
<td>

Date Modified

</td>
<td>

Timestamp

</td>
</tr>
<tr>
<td>

0x0b

</td>
<td>

Track Number

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x0c

</td>
<td>

Size

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x0d

</td>
<td>

Time

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x0e

</td>
<td>

Comment

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x10

</td>
<td>

Date Added

</td>
<td>

Timestamp

</td>
</tr>
<tr>
<td>

0x12

</td>
<td>

Composer

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x16

</td>
<td>

Play Count

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x17

</td>
<td>

Last Played

</td>
<td>

Timestamp

</td>
</tr>
<tr>
<td>

0x18

</td>
<td>

Disc Number

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x19

</td>
<td>

Stars/Rating

</td>
<td>

Integer (multiply by 20 for stars/rating)

</td>
</tr>
<tr>
<td>

0x1f

</td>
<td>

Compilation

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x23

</td>
<td>

BPM

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x27

</td>
<td>

Grouping

</td>
<td>

String (see special note)

</td>
</tr>
<tr>
<td>

0x28

</td>
<td>

Playlist

</td>
<td>

Integer - the playlist ID number (see special note)

</td>
</tr>
<tr>
<td>

0x36

</td>
<td>

Description

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x37

</td>
<td>

Category

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x39

</td>
<td>

Podcast

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x3c

</td>
<td>

Video Kind

</td>
<td>

Logic integer, works on mediatype

</td>
</tr>
<tr>
<td>

0x3e

</td>
<td>

TV Show

</td>
<td>

String

</td>
</tr>
<tr>
<td>

0x3f

</td>
<td>

Season Nr

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x44

</td>
<td>

Skip Count

</td>
<td>

Integer

</td>
</tr>
<tr>
<td>

0x45

</td>
<td>

Last Skipped

</td>
<td>

Timestamp

</td>
</tr>
<tr>
<td>

0x47

</td>
<td>

Album Artist

</td>
<td>

String

</td>
</tr>
</tbody>
</table>

Special Note about Grouping and Playlist fields - They don't work with
Live Updating on 3rd gen iPods, yet. This might get fixed in a future
firmware release. Maybe.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Smart_Playlist_Rule_Actions"></a>

<h5>

Smart Playlist Rule Actions

</h5>

The Action type is a 4 byte field. It is a bitmapped value, meaning that
each bit of these four bytes has a different meaning.

<table border="1" cellpadding="1" cellspacing="0">
<caption>

<b>Smart Playlist Rule Action High Byte</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

bit

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

0

</td>
<td>

The action is referring to a string value if set, not a string if not
set

</td>
</tr>
<tr>
<td>

1

</td>
<td>

NOT flag. If set, this negates the rule. Is becomes is not, contains
becomes does not contain, and so forth.

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="1" cellspacing="0">
<caption>

<b>Smart Playlist Rule Action Low 2 bytes</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

bit

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

0

</td>
<td>

Simple "IS" query

</td>
</tr>
<tr>
<td>

1

</td>
<td>

Contains

</td>
</tr>
<tr>
<td>

2

</td>
<td>

Begins with

</td>
</tr>
<tr>
<td>

3

</td>
<td>

Ends with

</td>
</tr>
<tr>
<td>

4

</td>
<td>

Greater Than

</td>
</tr>
<tr>
<td>

5

</td>
<td>

Greater Than or Equal To

</td>
</tr>
<tr>
<td>

6

</td>
<td>

Less Than

</td>
</tr>
<tr>
<td>

7

</td>
<td>

Less Than or Equal To

</td>
</tr>
<tr>
<td>

8

</td>
<td>

Is in the Range

</td>
</tr>
<tr>
<td>

9

</td>
<td>

In the Last

</td>
</tr>
<tr>
<td>

10

</td>
<td>

Is / Is Not (binary AND, only used for "Video Kind" so far)

</td>
</tr>
</tbody>
</table>

This is complicated, obviously, and iTunes abstracts this all by
defining various actions for various fields. Here's the complete
possible list of actions, including those that iTunes does not have but
which still work on the iPod:

<table border="1" cellpadding="1" cellspacing="2">
<caption>

<b>Smart Playlist Rule Actions</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

value

</th>
<th style="background:#efefef;">

action

</th>
</tr>
<tr>
<td>

0x00000001

</td>
<td>

Is Int (also Is Set in iTunes)

</td>
</tr>
<tr>
<td>

0x00000010

</td>
<td>

Is Greater Than (also Is After in iTunes)

</td>
</tr>
<tr>
<td>

0x00000020

</td>
<td>

Is Greater Than Or Equal To (not in iTunes)

</td>
</tr>
<tr>
<td>

0x00000040

</td>
<td>

Is Less Than (also Is Before in iTunes)

</td>
</tr>
<tr>
<td>

0x00000080

</td>
<td>

Is Less Than Or Equal To (not in iTunes)

</td>
</tr>
<tr>
<td>

0x00000100

</td>
<td>

Is in the Range

</td>
</tr>
<tr>
<td>

0x00000200

</td>
<td>

Is in the Last

</td>
</tr>
<tr>
<td>

0x00000400

</td>
<td>

Is / Is Not (Binary AND, used for media type so far)

</td>
</tr>
<tr>
<td>

0x01000001

</td>
<td>

Is String

</td>
</tr>
<tr>
<td>

0x01000002

</td>
<td>

Contains

</td>
</tr>
<tr>
<td>

0x01000004

</td>
<td>

Starts With

</td>
</tr>
<tr>
<td>

0x01000008

</td>
<td>

Ends With

</td>
</tr>
<tr>
<td>

0x02000001

</td>
<td>

Is Not Int (also Is Not Set in iTunes)

</td>
</tr>
<tr>
<td>

0x02000010

</td>
<td>

Is Not Greater Than (not in iTunes)

</td>
</tr>
<tr>
<td>

0x02000020

</td>
<td>

Is Not Greater Than Or Equal To (not in iTunes)

</td>
</tr>
<tr>
<td>

0x02000040

</td>
<td>

Is Not Less Than (not in iTunes)

</td>
</tr>
<tr>
<td>

0x02000080

</td>
<td>

Is Not Less Than Or Equal To (not in iTunes)

</td>
</tr>
<tr>
<td>

0x02000100

</td>
<td>

Is Not In the Range (not in iTunes)

</td>
</tr>
<tr>
<td>

0x02000200

</td>
<td>

Is Not In The Last

</td>
</tr>
<tr>
<td>

0x03000001

</td>
<td>

Is Not

</td>
</tr>
<tr>
<td>

0x03000002

</td>
<td>

Does Not Contain

</td>
</tr>
<tr>
<td>

0x03000004

</td>
<td>

Does Not Start With (not in iTunes)

</td>
</tr>
<tr>
<td>

0x03000008

</td>
<td>

Does Not End With (not in iTunes)

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Smart_Playlist_Rule_Values"></a>

<h5>

Smart Playlist Rule Values

</h5>

Values are generally pretty straightforward. For String rules, it's a
string. For Integer and Timestamp rules, it's a bit more complicated.
Furthermore, the "In the Last" type action requires a Units value as
well.

So there are two major rule formats, the String Rule and the Non-String
Rule.

<table border="1" cellpadding="3" cellspacing="0">
<caption>

<b>SPLRule String format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

field

</td>
<td>

4

</td>
<td>

The Field type

</td>
</tr>
<tr>
<td>

4

</td>
<td>

action

</td>
<td>

4

</td>
<td>

The Action type

</td>
</tr>
<tr>
<td>

8

</td>
<td>

padding

</td>
<td>

44

</td>
<td>

zero padding

</td>
</tr>
<tr>
<td>

52

</td>
<td>

length

</td>
<td>

4

</td>
<td>

length of the string, in bytes. Maximum length is 255

</td>
</tr>
<tr>
<td>

56

</td>
<td>

string

</td>
<td>

length

</td>
<td>

the string in UTF-16 format (2 bytes per character)

</td>
</tr>
<tr>
<td colspan="4">

rules are NOT zero padded at the end

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="3" cellspacing="0">
<caption>

<b>SPLRule Non-String format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

field

</td>
<td>

4

</td>
<td>

The Field type

</td>
</tr>
<tr>
<td>

4

</td>
<td>

action

</td>
<td>

4

</td>
<td>

The Action type

</td>
</tr>
<tr>
<td>

8

</td>
<td>

padding

</td>
<td>

44

</td>
<td>

zero padding

</td>
</tr>
<tr>
<td>

52

</td>
<td>

length

</td>
<td>

4

</td>
<td>

always 0x44 for non-string types

</td>
</tr>
<tr>
<td>

56

</td>
<td>

from value

</td>
<td>

8

</td>
<td>

the from value in an 8 byte form (unsigned int64)

</td>
</tr>
<tr>
<td>

64

</td>
<td>

from date

</td>
<td>

8

</td>
<td>

the from date in an 8 byte form (signed int64)

</td>
</tr>
<tr>
<td>

72

</td>
<td>

from units

</td>
<td>

8

</td>
<td>

the from units in an 8 byte form (unsigned int64)

</td>
</tr>
<tr>
<td>

80

</td>
<td>

to value

</td>
<td>

8

</td>
<td>

the to value in an 8 byte form (unsigned int64)

</td>
</tr>
<tr>
<td>

88

</td>
<td>

to date

</td>
<td>

8

</td>
<td>

the to date in an 8 byte form (signed int64)

</td>
</tr>
<tr>
<td>

96

</td>
<td>

to units

</td>
<td>

8

</td>
<td>

the to units in an 8 byte form (unsigned int64)

</td>
</tr>
<tr>
<td>

104

</td>
<td>

unknown

</td>
<td>

20

</td>
<td>

unknown, used by all field types, unknown purpose

</td>
</tr>
<tr>
<td colspan="4">

rules are NOT zero padded at the end

</td>
</tr>
</tbody>
</table>

For integer type rules, the from and to values are the ones that you
care about, the date = 0, and the units = 1.

    Example: BPM is less than 150
    field = 0x23 (BPM)
    action = 0x00000040 (is less than)
    from and to value = 150
    from and to date = 0
    from and to units = 1

    Example: BPM is in the range 70 to 150
    field = 0x23 (BPM)
    action = 0x00000100 (is in the range)
    from value = 70
    to value = 150
    from and to date = 0
    from and to units = 1

For binary and type rules, the from and to values are the ones that you
care about, the date = 0, and the units = 1.

    Example: Video Kind is TV-Show
    field = 0x3c (Video Kind)
    action = 0x00000400 (Is / Is Not)
    from and to value = 0x0040
    from and to date = 0
    from and to units = 1

    Example: Video Kind is not TV-Show
    field = 0x3c (Video Kind / mediatype)
    action = 0x00000400 (Is / Is Not)
    from and to value = 0x0e22
    from and to date = 0
    from and to units = 1

For the latter one would expect a value of 0x0022 (either Movie or Music
Video). The additional 0x0e00 in the mask hints for further video types
to come.

Timestamp type rules use the same format as integer type rules, only the
from and to values are Mac timestamps. This is the number of seconds
since 1/1/1904.

    Example: Date Added is in the range 6/19/2004 to 6/20/2004
    field = 0x10 (Date Added)
    action = 0x00000100 (is in the range)
    from value = bcfa83ff (6/19/2004) 
    to value = 0xbcfbd57f (6/20/2004)
    from and to date = 0
    from and to units = 1

For "in the last" type rules, the from and to values are set to a
constant of 0x2dae2dae2dae2dae. The from dates become the value, and the
from units becomes the unit you're measuring in. The way to think of
this is that it's saying "Today (2dae) plus this number of units".

    Example: Last Played Time is in the last 2 weeks.
    field = 0x17 (Last Played Time)
    action = 0x00000200 (is in the last)
    from value = 0x2dae2dae2dae2dae
    from date = -2
    from units = 604800 (number of seconds in a week)
    to value = 0x2dae2dae2dae2dae
    to date = 0
    to units = 1

That rule is saying "Today minus 2 times this number of seconds" which
is 2 weeks before "now", whatever now happens to be.

If you're creating your own rules in an iTunesDB, you may find it more
convienent to leave the units set to "1" all the time and just put the
number of seconds into the date field. This is perfectly acceptable and
the iPod can handle it just fine. If you're not sure how to make a
particular rule, create it or a similar one in iTunes and put it on the
iPod, then examine the iTunesDB file to see how it did it.

  
But if you're programming the iPod side of things, you need to be able
to correctly understand the units field and deal with it accordingly.
The best way to do this is to always compare the contents of the field
in question with (value+date\*unit), and replacing the "value" with the
current timestamp ("now") when it is equal to 3,291,618,603,768,360,366
(0x2dae2dae2dae2dae). This will work for \*all\* integer and timestamp
comparisons if done correctly. It's also exactly what the Apple iPod
firmware does. It's also why you have to set the time/date on an iPod
for these smart playlists to work correctly (3rd gen and up).

Also, remember that all timestamps should be dealt with in Apple format,
which is the number of seconds since 1/1/1904.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Putting_It_All_Together"></a>

<h5>

Putting It All Together

</h5>

So, now you know how to create a given rule. All that's left is to put
all the rules together and shove them into a type 51 MHOD.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - Smart Playlist Rules (type 51)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and the rules it contains

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 51 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

24

</td>
<td>

smart list rules identifier

</td>
<td>

4

</td>
<td>

"SLst" (note that this is the point at which bytes are no longer
little-endian in the mhod.. it switches to big-endian at this point)

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unk5

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

32

</td>
<td>

number of rules

</td>
<td>

4

</td>
<td>

number of rules

</td>
</tr>
<tr>
<td>

36

</td>
<td>

rules operator

</td>
<td>

4

</td>
<td>

0 = AND (as in "Match All"), 1 = OR (as in "Match Any")

</td>
</tr>
<tr>
<td>

40

</td>
<td>

padding

</td>
<td>

120

</td>
<td>

zero padding

</td>
</tr>
<tr>
<td>

160

</td>
<td>

rules

</td>
<td>

whatever size the rules are

</td>
<td>

the rules themselves, one after another.

</td>
</tr>
<tr>
<td colspan="4">

the mhod is NOT is zero padded at the end

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Library_Playlist_Index_.28type_52.29"></a>

<h4>

Library Playlist Index (type 52)

</h4>

The type 52 MHOD is only found as a child of the Main Library Playlist.
It is an index of the mhit's ordered by the major categories in the
Browse menu. The purpose of these mhod's is to speed up the operation of
the Browse menu itself. This is how it displays the information so
quickly when selecting one of the major categories, it's all presorted
for the iPod in these MHOD's.

Note that this MHOD is not mandatory, however the iPod menu system will
operate much slower without it (on large libraries), as it will have to
build the information provided here on the fly. Therefore it is
recommended to build this MHOD for anything more than trivial numbers of
songs.

Essentially, every MHIT is numbered from 0 to the total number of
songs-1. The type 52 MHOD contains a list of these index numbers using
one of the strings contained in these MHIT's, ordered alphabetically.

To build one of these, take all your songs, order them alphabetically by
one of these fields, then simply insert the index numbers of the ordered
songs into the type 52 mhod.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Library Playlist Index types</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

type number

</th>
<th style="background:#efefef;">

field that it indexes

</th>
</tr>
<tr>
<td>

0x03

</td>
<td>

Title

</td>
</tr>
<tr>
<td>

0x04

</td>
<td>

Album, then Disc/Tracknumber, then Title

</td>
</tr>
<tr>
<td>

0x05

</td>
<td>

Artist, then Album, then Disc/Tracknumber, then Title

</td>
</tr>
<tr>
<td>

0x07

</td>
<td>

Genre, then Artist, then Album, then Disc/Tracknumber, then Title

</td>
</tr>
<tr>
<td>

0x12

</td>
<td>

Composer, then Title

</td>
</tr>
<tr>
<td>

0x1d

</td>
<td>

Observed with iTunes 7.2, probably sorted by 'Show' first. Someone with
TV shows on his iPod please fill in the secondary sort orders.

</td>
</tr>
<tr>
<td>

0x1e

</td>
<td>

Observed with iTunes 7.2, probably sorted by 'Season Number' first.
Someone with TV shows on his iPod please fill in the secondary sort
orders.

</td>
</tr>
<tr>
<td>

0x1f

</td>
<td>

Observed with iTunes 7.2, probably sorted by 'Episode Number' first.
Someone with TV shows on his iPod please fill in the secondary sort
orders.

</td>
</tr>
<tr>
<td>

0x23

</td>
<td>

Observed with iTunes 7.3 (but may possibly exist in earlier versions,
too), unknown what this does.

</td>
</tr>
<tr>
<td>

0x24

</td>
<td>

Observed with iTunes 7.3 (but may possibly exist in earlier versions,
too), unknown what this does.

</td>
</tr>
</tbody>
</table>

(Note that the above list roughly matches the limit sort list.. I think
that these lists are actually the same in some way.)

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - Library Playlist Index (type 52)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all the index entries

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 52 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown (always zero)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown (always zero)

</td>
</tr>
<tr>
<td>

24

</td>
<td>

index type

</td>
<td>

4

</td>
<td>

what this index is sorted on (see list above)

</td>
</tr>
<tr>
<td>

28

</td>
<td>

count

</td>
<td>

4

</td>
<td>

number of entries. Always the same as the number of entries in the
playlist, which is the same as the number of songs on the iPod.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

null padding

</td>
<td>

40

</td>
<td>

lots of padding

</td>
</tr>
<tr>
<td>

72

</td>
<td>

index entries

</td>
<td>

4 \* count

</td>
<td>

The index entries themselves. This is an index into the mhit list, in
order, starting from 0 for the first mhit.

</td>
</tr>
<tr>
<td colspan="4">

the mhod is NOT zero padded at the end

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Type_53"></a>

<h4>

Type 53

</h4>

The type 53 MHOD is only found as a child of the Main Library Playlist.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - Letter jump table for fast scrolling (type 53)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all the index entries

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 53 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown (always zero)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown (always zero)

</td>
</tr>
<tr>
<td>

24

</td>
<td>

index type

</td>
<td>

4

</td>
<td>

what this index is sorted on (see list above)

</td>
</tr>
<tr>
<td>

28

</td>
<td>

count

</td>
<td>

4

</td>
<td>

number of entries. Unused letters are left out and umlauts (at least
German ones) are added after the letter they are derived from. Don't yet
know about the order of the French accented letters.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

null padding

</td>
<td>

8

</td>
<td>

lots of padding

</td>
</tr>
<tr>
<td>

40

</td>
<td>

index entries

</td>
<td>

12 \* count

</td>
<td>

The index entries themselves.

</td>
</tr>
<tr>
<td colspan="4">

the mhod is NOT zero padded at the end

</td>
</tr>
</tbody>
</table>

  

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Letter jump table entry</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

letter

</td>
<td>

4

</td>
<td>

The letter of this table entry. Looks like uppercase UTF-16LE with 2
padding null bytes (i.e. A would be 0x00000041 little endian /
0x41000000 big endian)

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

the number of the first entry in the corresponding MHOD52 index starting
with this letter. Zero-based and incremented by one for each entry, not
4.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

the count of entries starting with this letter in the corresponding
MHOD52.

</td>
</tr>
</tbody>
</table>

  
I don't know how these are associated with the corresponding MHOD52s, I
would guess by the sort order field, but I'm not sure because they
always turn up immediately after their MHOD52. Could someone please have
a look at this and the other yet unknown things above?

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Type_100"></a>

<h4>

Type 100

</h4>

There are a couple forms of Type 100 MHOD's (that we know of).

The first form is at the beginning of every playlist (before any MHIP
entries). Only iTunes puts it there, and only iTunes uses it. It
contains information on what columns to display, and what size to
display them as, when displaying the playlist in iTunes when the iPod is
in a manual mode. This is absolutely optional. The iPod itself doesn't
appear to use it in any way.

The second form is after every MHIP in a playlist, there's a small type
100 MHOD. It gives an order number to the song immediately before it. It
is not optional, but it's also not particularly useful.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Playlist_Column_Definitions_Header"></a>

<h5>

Playlist Column Definitions Header

</h5>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - Playlist Column Definition (type 100)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header. This is always 0x18 for this type of MHOD.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and it's data. This is always 0x288 for this type of
MHOD.

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 100 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown, always 0

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown, always 0

</td>
</tr>
<tr>
<td>

24

</td>
<td>

unk3

</td>
<td>

4

</td>
<td>

unknown, always 0

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unk4

</td>
<td>

8

</td>
<td>

unknown, appears to be 0x0088004A02EF0281 for everything except normal
playlists. Some kind of identifier?

</td>
</tr>
<tr>
<td>

36

</td>
<td>

unk8

</td>
<td>

4

</td>
<td>

unknown, always 0

</td>
</tr>
<tr>
<td>

40

</td>
<td>

unk9

</td>
<td>

2

</td>
<td>

unknown, appears to be 130 for normal playlists and 200 for everything
else

</td>
</tr>
<tr>
<td>

42

</td>
<td>

unk10

</td>
<td>

2

</td>
<td>

unknown, appears to be 3 for the iPod library and 1 for everything else

</td>
</tr>
<tr>
<td>

44

</td>
<td>

sort type

</td>
<td>

4

</td>
<td>

the sort type for this playlist. Use this value to figure out which
column is selected by mapping it to the correct column ID (listed
below).

</td>
</tr>
<tr>
<td>

48

</td>
<td>

number of columns

</td>
<td>

4

</td>
<td>

the number of columns visible in iTunes for this playlist. In iTunes 6,
this value can be anywhere from 2 to 28.

</td>
</tr>
<tr>
<td>

52

</td>
<td>

unknown1

</td>
<td>

2

</td>
<td>

unknown (always 1?) (Selected column?)

</td>
</tr>
<tr>
<td>

54

</td>
<td>

unknown2

</td>
<td>

2

</td>
<td>

unknown (always 0?) (Column sort direction? asc/desc)

</td>
</tr>
<tr>
<td>

56

</td>
<td>

column definitions

</td>
<td>

16 \* number of columns

</td>
<td>

the column definitions

</td>
</tr>
<tr>
<td colspan="4">

after the column definitions, the rest of the MHOD is zero-padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Column_Definition"></a>

<h5>

Column Definition

</h5>

Each column definition only consists of an ID for the playlist and the
sort direction for the column. The order they appear in this MHOD are
the order they appear in iTunes, from left to right. The first two
columns are always song position and title, in that order.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Single Column Definition</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

ID

</td>
<td>

2

</td>
<td>

the ID for this column, see below for possible values

</td>
</tr>
<tr>
<td>

2

</td>
<td>

width

</td>
<td>

2

</td>
<td>

the width of the column, in pixels

</td>
</tr>
<tr>
<td>

4

</td>
<td>

sort direction

</td>
<td>

4

</td>
<td>

if equal to 0x1, the sort is reversed for this column. Set to 0x0
otherwise.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

seems to be null padding

</td>
</tr>
<tr>
<td>

12

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

seems to be null padding

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Column IDs</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

ID

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

0x01

</td>
<td>

position; leftmost column in all playlists

</td>
</tr>
<tr>
<td>

0x02

</td>
<td>

Name

</td>
</tr>
<tr>
<td>

0x03

</td>
<td>

Album

</td>
</tr>
<tr>
<td>

0x04

</td>
<td>

Artist

</td>
</tr>
<tr>
<td>

0x05

</td>
<td>

Bit Rate

</td>
</tr>
<tr>
<td>

0x06

</td>
<td>

Sample Rate

</td>
</tr>
<tr>
<td>

0x07

</td>
<td>

Year

</td>
</tr>
<tr>
<td>

0x08

</td>
<td>

Genre

</td>
</tr>
<tr>
<td>

0x09

</td>
<td>

Kind

</td>
</tr>
<tr>
<td>

0x0A

</td>
<td>

Date Modified

</td>
</tr>
<tr>
<td>

0x0B

</td>
<td>

Track Number

</td>
</tr>
<tr>
<td>

0x0C

</td>
<td>

Size

</td>
</tr>
<tr>
<td>

0x0D

</td>
<td>

Time

</td>
</tr>
<tr>
<td>

0x0E

</td>
<td>

Comment

</td>
</tr>
<tr>
<td>

0x10

</td>
<td>

Date Added

</td>
</tr>
<tr>
<td>

0x11

</td>
<td>

Equalizer

</td>
</tr>
<tr>
<td>

0x12

</td>
<td>

Composer

</td>
</tr>
<tr>
<td>

0x14

</td>
<td>

Play Count

</td>
</tr>
<tr>
<td>

0x15

</td>
<td>

Last Played

</td>
</tr>
<tr>
<td>

0x16

</td>
<td>

Disc Number

</td>
</tr>
<tr>
<td>

0x17

</td>
<td>

My Rating

</td>
</tr>
<tr>
<td>

0x19

</td>
<td>

Date Released (Podcasts group only)

</td>
</tr>
<tr>
<td>

0x1A

</td>
<td>

BPM

</td>
</tr>
<tr>
<td>

0x1C

</td>
<td>

Grouping

</td>
</tr>
<tr>
<td>

0x1E

</td>
<td>

Category

</td>
</tr>
<tr>
<td>

0x1F

</td>
<td>

Description

</td>
</tr>
<tr>
<td>

0x21

</td>
<td>

Show

</td>
</tr>
<tr>
<td>

0x22

</td>
<td>

Season

</td>
</tr>
<tr>
<td>

0x23

</td>
<td>

Episode Number

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Playlist_Order_Entry"></a>

<h5>

Playlist Order Entry

</h5>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format - Playlist Order Entry (type 100)</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhod header. This is always 0x18 for this type of MHOD.

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and it's data. This is always 0x2C for this type of
MHOD.

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

4

</td>
<td>

the type indicator ( 100 )

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown, always 0

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown, always 0

</td>
</tr>
<tr>
<td>

24

</td>
<td>

position

</td>
<td>

4

</td>
<td>

Position of the song in the playlist. These numbers do not have to be
sequentially ordered, numbers can be skipped.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unknown

</td>
<td>

16

</td>
<td>

zero padding

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Album_List"></a>

<h3>

Album List

</h3>

Album List, first seen with iTunes 7.1. It was seen on iPod nano 2nd gen
and Shuffle 2nd gen restored with iTunes 7.1.1, but not on iPod Video
30GB(restored with the same version of iTunes).

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhla format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhla

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhla header

</td>
</tr>
<tr>
<td>

8

</td>
<td>

number of album items

</td>
<td>

4

</td>
<td>

the total number of songs in the Album List

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The Album List has Album Items as its children. The number of Album
Items is the same all albums on iPod.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Album_Item"></a>

<h3>

Album Item

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhia format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhia

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhia header. Length is 0x58

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of strings

</td>
<td>

4

</td>
<td>

number of strings (mhods) that are children of this mhia

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unknown

</td>
<td>

2

</td>
<td>

(previously long length 4 with possibly album ID)

</td>
</tr>
<tr>
<td>

18

</td>
<td>

album id for track

</td>
<td>

2

</td>
<td>

album ID (v. 0x18 file) (previously long length 4)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unknown

</td>
<td>

8

</td>
<td>

timestamp? (v. 0x18 file)

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

always 2? (v. 0x18 file)

</td>
</tr>
</tbody>
</table>

Usually mhia has two child strings: album title and artist name.

  

<div align="right">

<small>→ <a title="ITunesDB/Play Counts File">Subpage: Play Counts
File</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Play_Counts_file"></a>

<h2>

Play Counts file

</h2>

The play count information, since the last sync with iTunes, is stored
in the "Play Counts" file.

The play counts file is deleted by the iPod and rebuilt whenever it
detects that the iTunesDB file has changed (immediately after a sync).
So writing anything into this file yourself is more than a little
pointless, as the iPod simply will overwrite it. This is the iPod's way
of sending information back to iTunes (or whatever program you happen to
sync with).

When iTunes syncs, the first thing it does is read the values that the
iPod has put into this file, and merges them back into the iTunes
Library.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Play_Count_Header"></a>

<h3>

Play Count Header

</h3>

The play count header indicates a valid play count file and specifies
how many entries follow and the size of each entry record. The is an
entry record for each song on the iPod; the entry position corresponding
to the position of the song in the iTunesDB.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Play Count Header</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhdp

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

0x60

</td>
</tr>
<tr>
<td>

8

</td>
<td>

single entry length

</td>
<td>

4

</td>
<td>

0x0c (firmware 1.3), 0x10 (firmware 2.0), 0x14 (with version 0x0d of
iTunesDB), 0x1C (with version 0x13 of iTunesDB)

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of entries

</td>
<td>

4

</td>
<td>

number of songs on iPod

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Play_Count_Entry"></a>

<h3>

Play Count Entry

</h3>

The entry record contains the play count data for each song, one record
exists for each song on the iPod.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Play Count Entry</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

play count

</td>
<td>

4

</td>
<td>

number of played times since last sync

</td>
</tr>
<tr>
<td>

4

</td>
<td>

last played

</td>
<td>

4

</td>
<td>

last played time. Set to zero in older firmwares, or to the value from
iTunesDB in newer ones (anything with the "Music" menu).

</td>
</tr>
<tr>
<td>

8

</td>
<td>

audio bookmark

</td>
<td>

4

</td>
<td>

position in file that the song was last paused/stopped at, in
milliseconds. This works for audiobooks, podcasts, and seemingly
anything else with the right bit set in the MHIT (unk19).

</td>
</tr>
<tr>
<td>

12

</td>
<td>

rating

</td>
<td>

4

</td>
<td>

Rating given to song. Number of stars (1-5) \* 0x14. Set to zero in
older firmwares, or to the value from iTunesDB in newer ones (anything
with the "Music" menu).

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

Unknown, yet. Probably something to do with podcast bookmarking or some
such.

</td>
</tr>
<tr>
<td>

20

</td>
<td>

skip count

</td>
<td>

4

</td>
<td>

Number of times skipped since last sync. This field appears with
firmware that supports the 0x13 version iTunesDB.

</td>
</tr>
<tr>
<td>

24

</td>
<td>

last skipped

</td>
<td>

4

</td>
<td>

Last skipped date/time. Set to zero if never skipped. This field appears
with firmware that supports the 0x13 version iTunesDB.

</td>
</tr>
</tbody>
</table>

Older firmware versions had set the last played time and the rating to
"zero" in this database, unless you changed either one on the iPod
itself. Thus you could ignore zero entries.

Newer firmwares, however, set last played time and rating to be the same
as what is in the iTunesDB file, by default. Meaning that you can't
ignore zero entries for rating anymore (somebody could set the rating to
a zero rating on the iPod). So you must compare the rating in Play
Counts and the rating in iTunesDB to determine if it has actually
changed and you need to update it on the PC (in a program on the PC that
talks to the iPod that is).

  

<div align="right">

<small>→ <a title="ITunesDB/OTG Playlist File">Subpage: OTG Playlist
File</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="On_The_Go_Playlist_files"></a>

<h2>

On The Go Playlist files

</h2>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhpo format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhpo

</td>
</tr>
<tr>
<td>

4

</td>
<td>

file size

</td>
<td>

4

</td>
<td>

the header length, always 20 on my 4gen iPod

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown, appears to always be 0 (on my 4gen iPod it seems to be always
4)

</td>
</tr>
<tr>
<td>

12

</td>
<td>

tracks

</td>
<td>

4

</td>
<td>

number of tracks in this OTG playlist

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown, appears to always be 0 (on my 4gen iPod it is a fixed number,
and the OTG file will not work if this number is 0)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

track index number

</td>
<td>

4 \* tracks

</td>
<td>

This is a list of the tracks in the playlist, one after another. The
number is a zero based index into the Track List (MHLT) in the iTunesDB
file. So the first track in the MHLT would be 0, the second would be 1,
and so forth.

</td>
</tr>
</tbody>
</table>

Because the OTG Playlist uses track indexes, it is only valid until the
next time the iTunesDB changes. This means that you should read any
OTGPlaylist files on a sync and create proper Playlists for them in the
iTunesDB file or do something else with them. After you change the
iTunesDB file, they won't be valid anymore.

iTunes handles this by deleting the files once they are read, during an
autosync. Apple's iPod firmware also deletes these files if the iTunesDB
changes and it detects it, after you undock the iPod. iTunes will read
them and create real playlists from them before it syncs anything. Note
that in any sync operation, this \*must\* occur before the iTunesDB
changes, otherwise you'll lose the playlist.

  

<div align="right">

<small>→ <a title="ITunesDB/Equalizer Presets File">Subpage: Equalizer
Presets File</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Equalizer_Presets_file"></a>

<h2>

Equalizer Presets file

</h2>

The EQPresets file consists of a Presets Container to specify the number
of EQPresets in the file, and then a series of EQ Presets as its
children.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Equalizer_Presets_Container"></a>

<h3>

Equalizer Presets Container

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mqed format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mqed

</td>
</tr>
<tr>
<td>

header size

</td>
<td>

4

</td>
<td>

the size of the mqed header

</td>
</tr>
<tr>
<td>

unk1

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

number of presets

</td>
<td>

4

</td>
<td>

the number of presets defined in this file

</td>
</tr>
<tr>
<td>

childsize

</td>
<td>

4

</td>
<td>

the size of an individual preset in bytes. Always 588, thus far.

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Equalizer_Preset"></a>

<h3>

Equalizer Preset

</h3>

Each EQ Preset contains two actual presets. The first one is the
representation you see in iTunes - a 10 band Equalizer. iTunes actually
displays the given bands slightly wrong. The real values it uses are:
32Hz, 64Hz, 128Hz, 256Hz, 512Hz, 1024Hz, 2048Hz, 4096Hz, 8192Hz, 16384Hz

The EQ Preset also contains a 5 band version of the exact same preset.
The algorithim it uses to determine this from the 10 band preset is not
known, but you can assume it uses some form of curve fitting method. In
any case, the 5 band version would be the EQ setting that the iPod
actually would use, if it actually read and used this file as it
obviously was supposed to do. The reason for this is that the iPod's
audio DSP is capable of applying a 5 band EQ directly.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>pqed format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

pqed

</td>
</tr>
<tr>
<td>

length of preset name

</td>
<td>

2

</td>
<td>

the length, in bytes, of the name of the preset

</td>
</tr>
<tr>
<td>

name

</td>
<td>

510

</td>
<td>

The name of the preset, padded with nulls at the end. This is always 510
bytes long.

</td>
</tr>
<tr>
<td>

preamp

</td>
<td>

4

</td>
<td>

The preamp value. Possible values are from -1200 to 1200, measured as dB
\* 100. This is a signed long.

</td>
</tr>
<tr>
<td>

number of bands

</td>
<td>

4

</td>
<td>

number of bands in the preset. This is always 10.

</td>
</tr>
<tr>
<td>

band values

</td>
<td>

40

</td>
<td>

10 bands of values, each a signed long from -1200 to 1200 (measured as
dB \* 100)

</td>
</tr>
<tr>
<td>

number of bands 2

</td>
<td>

4

</td>
<td>

number of bands in the second representation. This is always 5.

</td>
</tr>
<tr>
<td>

band values 2

</td>
<td>

20

</td>
<td>

5 bands of values, each a signed long from -1200 to 1200 (measured as dB
\* 100)

</td>
</tr>
<tr>
<td colspan="3">

rest of pqed is NOT zero padded

</td>
</tr>
</tbody>
</table>

  

<div align="right">

<small>→ <a title="ITunesDB/Artwork Database">Subpage: Artwork
Database</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Artwork_Database"></a>

<h2>

Artwork Database

</h2>

The ArtworkDB file is only found on iPod Photos. It is created by iTunes
4.7 and up. This file, along with the thumbnail files iTunes creates,
are what allows photos and album art to be displayed on the iPod Photo.

The Photo Database file is similar to the ArtworkDB file. The Photo
Database stores photos you add manually, the ArtworkDB stores album
artwork for displaying when playing music.

-Images are concatenated into \*.ithmb files (thumbnails, basically).

-Large thumbnails are stored separately from small thumbnails.

-Large thumbnails are 140x140, small thumbnails are 56x56 - raw RGB565
packed color binary streams (bytes are swapped, little endian).

ArtworkDB Database layout:

    &lt;mhfd&gt;
     &lt;mhsd&gt; (index = 1)
       &lt;mhli&gt;
         &lt;mhii&gt;
           &lt;mhod&gt; (type = 2) Info about full size thumbnail
             &lt;mhni&gt;
               &lt;mhod&gt; (type = 3)
           &lt;mhod&gt; (type = 2) Info about 'now playing' thumbnail
             &lt;mhni&gt;
               &lt;mhod&gt; (type = 3)
         &lt;mhii&gt;
           &lt;mhod&gt; (type = 2)
             &lt;mhni&gt;
               &lt;mhod&gt; (type = 3)
           &lt;mhod&gt; (type = 2)
             &lt;mhni&gt;
               &lt;mhod&gt; (type = 3)
          ...
     &lt;mhsd&gt; (index = 2)
       &lt;mhla&gt;
     &lt;mhsd&gt; (index = 3)
       &lt;mhlf&gt;
         &lt;mhif&gt;
         &lt;mhif&gt;
         ...

  

<div align="right">

<small>→ <a title="ITunesDB/Photo Database">Subpage: Photo
Database</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Photo_Database"></a>

<h2>

Photo Database

</h2>

The Photo Database is created by iTunes, and is stored in "/Photos/Photo
Database". The Photo Database looks similar to the ArtworkDB but has
additional entries in the mhla object (mhba and mhia, see below) as well
as different Thumbnails. The mhiis in the Photo Database look like this
for example:

    mhii (children: 5, id: 117, srcimgsize: 0)
      mhod (type: 5)
        mhni (children: 1, corrid: 1, ithmb offset: 0, imgsize: 1567395, imgdim: 0x0)
          mhod (type: 3, length: 80, string: u':Full Resolution:2005:08:06:simg3609.jpg')
      mhod (type: 2)
        mhni (children: 1, corrid: 1019, ithmb offset: 7603200, imgsize: 691200, imgdim: 0x2c801e0)
          mhod (type: 33554435, length: 42, string: u':Thumbs:F1019_1.ithmb')
      mhod (type: 2)
        mhni (children: 1, corrid: 1020, ithmb offset: 851840, imgsize: 77440, imgdim: 0xa500e1)
          mhod (type: 33554435, length: 42, string: u':Thumbs:F1020_1.ithmb')
      mhod (type: 2)
        mhni (children: 1, corrid: 1009, ithmb offset: 27720, imgsize: 2520, imgdim: 0x29001f)
          mhod (type: 33554435, length: 42, string: u':Thumbs:F1009_1.ithmb')
      mhod (type: 2)
        mhni (children: 1, corrid: 1015, ithmb offset: 251680, imgsize: 22880, imgdim: 0x7b0058)
          mhod (type: 33554435, length: 42, string: u':Thumbs:F1015_1.ithmb')

The type 5 mhod references the full resolution image and is probably
only there if the corresponding check box in iTunes was checked. The
type 2 mhods reference different types of thumbnail versions:

1.  The first thumbnail (imgdim: 0x2c801e0, which decodes as 712x480) is
    of dimension 720x480 and in YUV 4:2:2 format, interlaced. It
    consists of two half-frames concatenated together. This is probably
    used for the TV-out (dimension, color format and interlacing look
    like NTSC).
2.  The second thumbnail (imgdim: 0xa500e1, which decodes as 165x225) is
    of dimension 176x220 and in RGB 565 format, but rotated 90° CCW.
    This is the image that is actually displayed on the iPod screen.
3.  The third thumbnail (imgdim: 0x29001f(41x31)) is 42x30, RGB 565 with
    swapped bytes (e.g. it's more like GBRG 3553). This is the image
    that is used as a thumbnail.
4.  The fourth thumbnail (imgdim: 0x7b0058(123x88)) is 130x88, RGB 565
    with swapped bytes

There is a \*.ithmb file per resolution (in the directory
"/Photos/Thumbs/"), that concatenates all thumbnails with that
resolution.

On an iPod video (5G) there are 4 different thumbnails type:

1.  720x480 interlaced UYVY (YUV 4:2:2) - used for TV output - 691200
    bytes each single thumbnail
2.  320x240 byte swapped RGB565 - used for fullscreen on the iPod -
    153600 bytes each single thumbnail
3.  130x88 byte swapped RGB565 - used on the iPod during slideshow, when
    current photo is displayed on TV - 22880 bytes each single thumbnail
4.  50x41 byte swapped RGB565 - used on the iPod when listing and during
    slideshow - 4100 bytes each single thumbnail

Dimensions of the fields in the <i>Photo Database</i> are very
important. Only one total length field or one padding field with wrong
value could be enough to make the Photo Database file completely
unusable: then nothing will be displayed on the iPod, no photo albums,
no photos.

Here follows a complete structure for a Photo Database file working on
an iPod video 5G:

     'mhfd', 132, 1384, 0, 1, 3, 0, 102, 0, 0, 0, 0, 2, 0, 0, 0, 0
     'mhsd', 96, 1276, 1
     'mhli', 92, 1
     'mhii', 152, 1088, 5, 100, 102, 0, 0, 0, 0, 3221487006, 3221487006, 0
     'mhod', 24, 216, 5, 0, 0
     'mhni', 76, 192, 1, 1, 0, 0, 0, 0, 0, 0
     'mhod', 24, 116, 3, 0, 2
     78, 2, 0, ':Full Resolution:2006:01:22:DSC00090.JPG'
     'mhod', 24, 180, 2, 0, 0
     'mhni', 76, 156, 1, 1019, 0, 691200, 0, 0, 480, 712
     'mhod', 24, 80, 3, 0, 2
     42, 2, 0, ':Thumbs:F1019_1.ithmb'
     'mhod', 24, 180, 2, 0, 0
     'mhni', 76, 156, 1, 1024, 0, 153600, 0, 0, 240, 320
     'mhod', 24, 80, 3, 0, 2
     42, 2, 0, ':Thumbs:F1024_1.ithmb'
     'mhod', 24, 180, 2, 0, 0
     'mhni', 76, 156, 1, 1015, 0, 22880, 0, 0, 88, 123
     'mhod', 24, 80, 3, 0, 2
     42, 2, 0, ':Thumbs:F1015_1.ithmb'
     'mhod', 24, 180, 2, 0, 0
     'mhni', 76, 156, 1, 1036, 0, 4100, 0, 0, 41, 53
     'mhod', 24, 80, 3, 0, 2
     42, 2, 0, ':Thumbs:F1036_1.ithmb'
     'mhsd', 96, 660, 2
     'mhla', 92, 2
     'mhba', 148, 232, 1, 1, 101, 0, 65536, 0, 0, 0, 0, 0, 0, 0, 100
     'mhod', 24, 44, 1, 0, 1
     7, 1, 0, 'Library'
     'mhia', 40, 40, 0, 100
     'mhba', 148, 240, 1, 1, 102, 0, 393216, 0, 0, 0, 0, 0, 0, 0, 101
     'mhod', 24, 52, 1, 0, 3
     13, 1, 0, 'A Photo Album'
     'mhia', 40, 40, 0, 100
     'mhsd', 96, 684, 3
     'mhlf', 92, 4
     'mhif', 124, 124, 0, 1019, 691200
     'mhif', 124, 124, 0, 1015, 22880
     'mhif', 124, 124, 0, 1024, 153600
     'mhif', 124, 124, 0, 1036, 4100

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Data_File_Object"></a>

<h3>

Data File Object

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhfd format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhfd

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhfd header (0x84)

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records (since everything is a child of
MHFD, this will always be the size of the entire file)

</td>
</tr>
<tr>
<td>

12

</td>
<td>

unknown1

</td>
<td>

4

</td>
<td>

always seem to be 0

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unknown2

</td>
<td>

4

</td>
<td>

always seem to be 1 for older databases, in an ArtworkDB generated by
iTunes 4.9 or above, it's 2. Caution: iTunes7 removes the whole database
if this field is 1

</td>
</tr>
<tr>
<td>

20

</td>
<td>

number of children

</td>
<td>

4

</td>
<td>

always 3 since there are 3 list holders

</td>
</tr>
<tr>
<td>

24

</td>
<td>

unknown3

</td>
<td>

4

</td>
<td>

always seem to be 0

</td>
</tr>
<tr>
<td>

28

</td>
<td>

next id for mhii

</td>
<td>

4

</td>
<td>

The id of last mhii + 1. (is probably used as the id of a newly added
mhii and then incremented). On an iPod video seems to be the id of the
last mhii + the total number of photo albums (mhba) + 1.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

unknown5

</td>
<td>

8

</td>
<td>
</td>
</tr>
<tr>
<td>

40

</td>
<td>

unknown6

</td>
<td>

8

</td>
<td>
</td>
</tr>
<tr>
<td>

48

</td>
<td>

unknown7

</td>
<td>

4

</td>
<td>

always seem to be 2

</td>
</tr>
<tr>
<td>

52

</td>
<td>

unknown8

</td>
<td>

4

</td>
<td>

always seem to be 0

</td>
</tr>
<tr>
<td>

56

</td>
<td>

unknown9

</td>
<td>

4

</td>
<td>

always seem to be 0

</td>
</tr>
<tr>
<td>

60

</td>
<td>

unknown10

</td>
<td>

4

</td>
<td>
</td>
</tr>
<tr>
<td>

64

</td>
<td>

unknown11

</td>
<td>

4

</td>
<td>
</td>
</tr>
<tr>
<td>

68

</td>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="DataSet_2"></a>

<h3>

DataSet

</h3>

This is basically the same as the MHSD element in the iTunes DB.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhsd format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhsd

</td>
</tr>
<tr>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhsd header (0x60)

</td>
</tr>
<tr>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

index

</td>
<td>

4

</td>
<td>

An index number. This value is 1 if the child is an Image List, 2 if the
child is an Album List, or 3 if it's a File List.

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

Depending on the index, the Data Set either contains an Image List
(mhli) child, an Album List (mhla) child or a File List child (mhlf).

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Image_List"></a>

<h3>

Image List

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhli format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhli

</td>
</tr>
<tr>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhli header (0x5c)

</td>
</tr>
<tr>
<td>

number of images

</td>
<td>

4

</td>
<td>

the total number of images in the Image List

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The Image List has Image Items as its children. The number of Image
Items is the same as the number of images.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Image_Item"></a>

<h3>

Image Item

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhii format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhii

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhii header (0x98)

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of children

</td>
<td>

4

</td>
<td>

In ArtworkDB there are 2 children: one mhod type 2 record for the full
sized thumbnail, and one mhod type 2 record for the now-playing sized
thumbnail. In Photo Database there are: a child for every thumbnail type
(2 on Nanoes, 4 on Photo/Color/Video iPods) + a child for the reference
to the full resolution image (if chosen to include it). In Photo
Database files generated on Macs, probably by iPhoto, sometimes there
could be an additional child, a type-1 string MHOD containing an UTF-8
string of a label for the image, usually found as first child just after
the MHII header.

</td>
</tr>
<tr>
<td>

16

</td>
<td>

id

</td>
<td>

4

</td>
<td>

First mhii is 0x40, second is 0x41, ... (on mobile phones the first mhii
appears to be 0x64, second 0x65, ...)

</td>
</tr>
<tr>
<td>

20

</td>
<td>

Song ID

</td>
<td>

8

</td>
<td>

Unique ID corresponding to the 'dbid' field in the iTunesDB mhit record,
this is used to map ArtworkDB items to iTunesDB items.

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unknown4

</td>
<td>

4

</td>
<td>

Seems to always be 0

</td>
</tr>
<tr>
<td>

32

</td>
<td>

rating

</td>
<td>

4

</td>
<td>

Rating from iPhoto \* 20

</td>
</tr>
<tr>
<td>

36

</td>
<td>

unknown6

</td>
<td>

4

</td>
<td>

Seems to always be 0

</td>
</tr>
<tr>
<td>

40

</td>
<td>

originalDate

</td>
<td>

4

</td>
<td>

Seems to always be 0 in ArtworkDB; timestamp in Photo Database (creation
date of file)

</td>
</tr>
<tr>
<td>

44

</td>
<td>

digitizedDate

</td>
<td>

4

</td>
<td>

Seems to always be 0 in ArtworkDB; timestamp in Photo Database (date
when the picture was taken, probably from EXIF information)

</td>
</tr>
<tr>
<td>

48

</td>
<td>

source image size

</td>
<td>

4

</td>
<td>

Size in bytes of the original source image

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

MHOD type 2, Annoyingly, not a string like it is in iTunesDB... but
still defines location of the file in question, sorta: its mhni child
record contains everything that is needed about the image file.

    header_size = 0x18
    total_size
    type = 2
    unk1 = 0
    unk2 = 0

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Image_Name"></a>

<h3>

Image Name

</h3>

  

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhni format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhni

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhni header (0x4c)

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

number of children

</td>
<td>

4

</td>
<td>

mhni headers have one mhod type 3 child

</td>
</tr>
<tr>
<td>

16

</td>
<td>

correlation ID

</td>
<td>

4

</td>
<td>

corresponds to mhif's correlation id, it's used to generate the name of
the file containing the image, see below

</td>
</tr>
<tr>
<td>

20

</td>
<td>

ithmb offset

</td>
<td>

4

</td>
<td>

offset where the start of image data can be found in the .ithmb file
corresponding to this image

</td>
</tr>
<tr>
<td>

24

</td>
<td>

image size

</td>
<td>

4

</td>
<td>

size of the image in bytes

</td>
</tr>
<tr>
<td>

28

</td>
<td>

vertical padding

</td>
<td>

2

</td>
<td>

approximate difference between scaled image height and pixmap height
(signed)

</td>
</tr>
<tr>
<td>

30

</td>
<td>

horizontal padding

</td>
<td>

2

</td>
<td>

approximate difference between scaled image width and pixmap width
(signed)

</td>
</tr>
<tr>
<td>

32

</td>
<td>

image height

</td>
<td>

2

</td>
<td>

The height of the image.

</td>
</tr>
<tr>
<td>

34

</td>
<td>

image width

</td>
<td>

2

</td>
<td>

The width of the image.

</td>
</tr>
<tr>
<td>

36

</td>
<td>

unknown

</td>
<td>

4

</td>
<td>

Always zero?

</td>
</tr>
<tr>
<td>

40

</td>
<td>

image size

</td>
<td>

4

</td>
<td>

size of the image in bytes (same as 0x18), written by iTunes 7.4

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The correlation ID gives us the name of the file containing the image.
For example, if the correlation ID is 1016 in decimal, then the
corresponding filename will be F1016\_1.ithmb.

In general, (vertical padding +image height) \~ pixmap height - usually
within one or two pixels, probably due to rounding error. For instance,
on a PhotoPod, an original image with dimensions 1200h x 1600v will have
an NTSC image with image height=480, image width=558, vertical
padding=0, and horizontal padding=162, with 558+162 = 720, the actual
width of the pixel map. For an image scaled to be contained entirely
within the pixel map, such as the video image or the full-screen image
the padding values are basically the total width of the black bars.

For the smallest thumbnails, you can have negative values for padding,
because the pixel map is scaled to be contained <i>within</i> the image
- you get a central "slice" with no black bars.

As noted, there appear to be some rounding errors when the padding
values are calculated, as the sums are sometimes off by 1 to 2.

There is no indication in this object what the pixel format, actual
pixel map dimensions or rotation of images will be, so this must be
entirely derived from the image size.

Here are the dimensions and formats for all known image sizes:

<table border="1" cellpadding="5" cellspacing="0">
<tbody>
<tr>
<td>

size

</td>
<td>

height

</td>
<td>

width

</td>
<td>

format

</td>
<td>

description

</td>
</tr>
<tr>
<td>

691200

</td>
<td>

480

</td>
<td>

720

</td>
<td>

UYVY

</td>
<td>

PhotoPod and VideoPod NTSC image

</td>
</tr>
<tr>
<td>

153600

</td>
<td>

240

</td>
<td>

320

</td>
<td>

RGB565\_LE

</td>
<td>

VideoPod full screen

</td>
</tr>
<tr>
<td>

80000

</td>
<td>

200

</td>
<td>

200

</td>
<td>

RGB565\_LE

</td>
<td>

VideoPod album art big version

</td>
</tr>
<tr>
<td>

77440

</td>
<td>

176

</td>
<td>

220

</td>
<td>

RGB565\_BE\_90

</td>
<td>

PhotoPod full screen

</td>
</tr>
<tr>
<td>

46464

</td>
<td>

132

</td>
<td>

176

</td>
<td>

RGB565\_BE

</td>
<td>

Nano full screen

</td>
</tr>
<tr>
<td>

39200

</td>
<td>

140

</td>
<td>

140

</td>
<td>

RGB565\_LE

</td>
<td>

PhotoPod album art big version

</td>
</tr>
<tr>
<td>

22880

</td>
<td>

88

</td>
<td>

130

</td>
<td>

RGB565\_LE

</td>
<td>

PhotoPod and VideoPod video preview

</td>
</tr>
<tr>
<td>

20000

</td>
<td>

100

</td>
<td>

100

</td>
<td>

RGB565\_LE

</td>
<td>

VideoPod album art small version, Nano album art big version

</td>
</tr>
<tr>
<td>

6272

</td>
<td>

56

</td>
<td>

56

</td>
<td>

RGB565\_LE

</td>
<td>

PhotoPod album art small version

</td>
</tr>
<tr>
<td>

4100

</td>
<td>

41

</td>
<td>

50

</td>
<td>

RGB565\_LE

</td>
<td>

VideoPod list thumbnail

</td>
</tr>
<tr>
<td>

3528

</td>
<td>

42

</td>
<td>

42

</td>
<td>

RGB565\_LE

</td>
<td>

Nano album art small version

</td>
</tr>
<tr>
<td>

3108

</td>
<td>

37

</td>
<td>

42

</td>
<td>

RGB565\_LE

</td>
<td>

Nano list thumbnail

</td>
</tr>
<tr>
<td>

2520

</td>
<td>

30

</td>
<td>

42

</td>
<td>

RGB565\_LE

</td>
<td>

PhotoPod list thumbnail

</td>
</tr>
</tbody>
</table>

where:

-   UYVY is a byte stream where U,Y0,V,Y1 creates two YUV pixels of
    Y0,U,V and Y1,U,V, interlaced, all even fields, then all odd fields.
-   RGB565\_LE is a stream of byte-swapped 16-bit pixels ordered from
    top-\>bottom, left-\>right
-   RGB565\_BE\_90 is a stream of 16-bit pixels ordered right to left,
    top to bottom

The "full screen" images are rotated because the iPod displays used are
actually portrait, not landscape, and this format is just a memory dump
of the frame buffer memory.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Album_List_2"></a>

<h3>

Album List

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhla format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhla

</td>
</tr>
<tr>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhla header (0x5c)

</td>
</tr>
<tr>
<td>

number of children

</td>
<td>

4

</td>
<td>

the total number of children in the Album List (no children in
ArtworkDB, 1 or more children in Photo Database)

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The Album List has no children in the case of the ArtworkDB file, and 1
or more children for the Photo Database: 1 child for the Photo Library
and possible some more children for additional photo albums.

For the Photo Database the layout looks like this, for example:

    mhsd (type: 2)
      mhla (children: 3)
        mhba (number of mhods: 1, number of mhias: 5, unk3: 0x10000)
          mhod (string: "My Pictures")
          mhia (image id: 100)
          mhia (image id: 101)
          mhia (image id: 102)
          mhia (image id: 103)
          mhia (image id: 104)
        mhba (number of mhods: 1, number of mhias: 2, unk3: 0x60000)
          mhod (string: "Folder A")
          mhia (image id: 100)
          mhia (image id: 101)
        mhba (number of mhods: 1, number of mhias: 3, unk3: 0x60000)
          mhod (string: "Folder B")
          mhia (image id: 102)
          mhia (image id: 103)
          mhia (image id: 104)

In this case "My Pictures" was the folder that was selected in iTunes to
synchronize photos with, and it contained 2 folders "Folder A" and
"Folder B" with 2 and 3 photos respectively. The iPod will show this on
its Photo menu as a submenu called "Photo Library" (containing all 5
photos), a submenu called "Folder A" with the first 2 photos and a
submenu "Folder B" with the other 3 photos.

Note that the string MHODs are zero-padded to a length that is a
multiple of 4 so that in the Photo Database all objects start on a
4-byte boundary. The iPod won't list any photo or photo album otherwise.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Photo_Album"></a>

<h3>

Photo Album

</h3>

An MHBA allows you to setup an Album of photos (grouping photos together
in an album). The MHBA is implemented similar to playlist.

These are only supported on models that support photos.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhba format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhba

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhba header (0x94)

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

Data Object Child Count

</td>
<td>

4

</td>
<td>

number of Data Objects in the List, probably always 1. Sometimes seems
to be 2 in the new video iPods' Photo Database, see below.

</td>
</tr>
<tr>
<td>

16

</td>
<td>

Album Item Count

</td>
<td>

4

</td>
<td>

number of pictures in the album

</td>
</tr>
<tr>
<td>

20

</td>
<td>

playlist id

</td>
<td>

4

</td>
<td>

a unique integer for each playlist - starts out at $64 and increments by
1. Really seems to be the total number of pictures (MHII) + photo album
number (+1 for the first album, +2 for the second, etc. etc.)

</td>
</tr>
<tr>
<td>

24

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

unknown, seems to be always 0

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unk3

</td>
<td>

2

</td>
<td>

unknown, seems to be always 0

</td>
</tr>
<tr>
<td>

30

</td>
<td>

Album Type

</td>
<td>

1

</td>
<td>

1 = master photo list ("Photo Library"), 2 = normal album, sometimes 4
and 5

</td>
</tr>
<tr>
<td>

31

</td>
<td>

playMusic

</td>
<td>

1

</td>
<td>

play music during slideshow (from iPhoto setting)

</td>
</tr>
<tr>
<td>

32

</td>
<td>

repeat

</td>
<td>

1

</td>
<td>

repeat the slideshow (from iPhoto setting)

</td>
</tr>
<tr>
<td>

33

</td>
<td>

random

</td>
<td>

1

</td>
<td>

show the slides in random order (from iPhoto setting)

</td>
</tr>
<tr>
<td>

34

</td>
<td>

showTitles

</td>
<td>

1

</td>
<td>

show slide captions (from iPhoto setting)

</td>
</tr>
<tr>
<td>

35

</td>
<td>

transitionDirection

</td>
<td>

1

</td>
<td>

0=none, 1=left-to-right, 2=right-to-left, 3=top-to-bottom,
4=bottom-to-top (from iPhoto setting)

</td>
</tr>
<tr>
<td>

36

</td>
<td>

slideDuration

</td>
<td>

4

</td>
<td>

in seconds (from iPhoto setting)

</td>
</tr>
<tr>
<td>

40

</td>
<td>

transitionDuration

</td>
<td>

4

</td>
<td>

in milliseconds (from iPhoto setting)

</td>
</tr>
<tr>
<td>

44

</td>
<td>

unk7

</td>
<td>

4

</td>
<td>

unknown, seems to always be 0

</td>
</tr>
<tr>
<td>

48

</td>
<td>

unk8

</td>
<td>

4

</td>
<td>

unknown, seems to always be 0

</td>
</tr>
<tr>
<td>

52

</td>
<td>

dbid2

</td>
<td>

8

</td>
<td>

dbid2 of track in iTunesDB to play during slideshow (from iPhoto
setting)

</td>
</tr>
<tr>
<td>

60

</td>
<td>

prev playlist id

</td>
<td>

4

</td>
<td>

the id of the previous playlist  

Seems generated by this formula: value of the same field in the previous
photo album MHBA + number of photos of the previous photo album + 1  
In particular, this formula is valid starting from the second album: in
fact, the value of this field for the library is 100, and for the first
photo album is 101.  
Example: if the first album contains 50 photos, the value of this field
in the second album will be 152.

</td>
</tr>
<tr>
<td colspan="4">

rest of header is zero

</td>
</tr>
</tbody>
</table>

The MHBA has several children: The first is a MHOD containing the album
name (which is ignored if the MHBA is the Photo Library). After that
there are several MHIAs that define which image to show in that album.
The MHBA that is marked as Photo Library contains one MHIA record for
every photo on the iPod.

On the new video iPods and nanos of recent vintage, the MHBA has a
second MHOD as a child which contains a string of which specifies the
transition effect configured in iPhoto for the slideshow associated with
this album. Apparently, the iPod ignores the slideshow settings that
come from iPhoto, except for the slideshow soundtrack.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Album_Item_2"></a>

<h3>

Album Item

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhia format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhia

</td>
</tr>
<tr>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhia header (0x28)

</td>
</tr>
<tr>
<td>

total length?

</td>
<td>

4

</td>
<td>

probably the size of the header and all child records; as there aren't
any child records this is equal to header length (40)

</td>
</tr>
<tr>
<td>

unk1

</td>
<td>

4

</td>
<td>

seems to be zero

</td>
</tr>
<tr>
<td>

image id

</td>
<td>

4

</td>
<td>

the id of the mhii record this mhia refers to

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="File_List"></a>

<h3>

File List

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhlf format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhlf

</td>
</tr>
<tr>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhlf header (0x5c)

</td>
</tr>
<tr>
<td>

number of files

</td>
<td>

4

</td>
<td>

the total number of files in the File List

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

The File List has Files (Images) as its children.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="File_.28Image.29"></a>

<h3>

File (Image)

</h3>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhif format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhif

</td>
</tr>
<tr>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the mhif header (0x7c)

</td>
</tr>
<tr>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and all child records

</td>
</tr>
<tr>
<td>

unknown1

</td>
<td>

4

</td>
<td>

always seems to be 0

</td>
</tr>
<tr>
<td>

correlation id

</td>
<td>

4

</td>
<td>

used to link this entry with a file and an Image Name, see Image Name
for more details.

</td>
</tr>
<tr>
<td>

image size

</td>
<td>

4

</td>
<td>

size of the image in bytes. A full sized thumbnail is 39,200 bytes, a
'Now Playing' thumbnail is 6,272 bytes on the iPod Photo/Color. On the
iPod Nano, a full sized thumbnail is 20,000 bytes while a 'Now Playing'
thumbnail is 3,528 bytes.

</td>
</tr>
<tr>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Data_Object_2"></a>

<h3>

Data Object

</h3>

The MHODs found in the ArtworkDB and Photo Database files are
significantly different than those found in the normal iTunesDB files.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>mhod format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

mhod

</td>
</tr>
<tr>
<td>

4

</td>
<td>

header length

</td>
<td>

4

</td>
<td>

size of the MHOD header (0x18)

</td>
</tr>
<tr>
<td>

8

</td>
<td>

total length

</td>
<td>

4

</td>
<td>

size of the header and content, including child records

</td>
</tr>
<tr>
<td>

12

</td>
<td>

type

</td>
<td>

2

</td>
<td>

type of the MHOD, see below

</td>
</tr>
<tr>
<td>

14

</td>
<td>

unknown

</td>
<td>

1

</td>
<td>

unknown, always 0 so far

</td>
</tr>
<tr>
<td>

15

</td>
<td>

padding length

</td>
<td>

1

</td>
<td>

all MHODs must be zero-padded so that the length is a multiple of 4,
this field contains the number of padding bytes added (e.g. 0, 1, 2 or
3). <font color="darkred">WARNING!</font> This field was always set to 2
for a while. To avoid parser crash, the best way is to ignore it when
parsing.

</td>
</tr>
<tr>
<td>

16

</td>
<td colspan="3">

rest of header is zero padded

</td>
</tr>
</tbody>
</table>

There are 2 groups of types of MHODs in the ArtworkDB: container MHODs
contain a MHNI as a child, while 'normal' string MHODs contain a string.

<b>Attention</b>: Sometimes seems that the MHBAs in the new video and
nano iPods' Photo Database have a second MHOD child which, although
being identified by a type of 2, is a string (and not container) MHOD.
This second string MHOD in photo album is usually found in Photo
Database files generated on Macs, probably by iPhoto, and contains an
UTF-8 string describing a transition effect such as "Dissolve". However
in Photo Database files generated on PCs for example by iTunes 6 for an
iPod video 30Gb this does not happen, and there is only one type-1
string MHOD as child, just like with iPod Photo/Color Photo Database
files.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>MHOD types</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

type

</th>
<th style="background:#efefef;">

group

</th>
<th style="background:#efefef;">

description

</th>
</tr>
<tr>
<td>

1

</td>
<td>

string

</td>
<td>

Album name (in the Photo Database)

</td>
</tr>
<tr>
<td>

2

</td>
<td>

container

</td>
<td>

Thumbnail image

</td>
</tr>
<tr>
<td>

3

</td>
<td>

string

</td>
<td>

File name

</td>
</tr>
<tr>
<td>

5

</td>
<td>

container

</td>
<td>

Full Resolution image (in the Photo Database)

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Container_MHODs"></a>

<h4>

Container MHODs

</h4>

MHODs with type 2 contain a MHNI that (contains a type 3 MHOD that)
references a thumbnail. MHODs with type 5 contain a MHNI that (contains
a type 3 MHOD that) references a full resolution image (in the Photo
Database).

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="String_MHODs"></a>

<h4>

String MHODs

</h4>

The content of string MHODs (probably all types except 2 and 5, although
only 1 and 3 have been observed so far) is structured again with
something like a sub-header:

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>string mhod content format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

string length

</td>
<td>

4

</td>
<td>

length in bytes of the string (e.g. after encoding)

</td>
</tr>
<tr>
<td>

unknown

</td>
<td>

4

</td>
<td>

might be the string encoding: 0,1 == UTF-8; 2 == UTF-16-LE. Observed
values are: 1 in type 1 MHODs and 2 in type 3 MHODs.

</td>
</tr>
<tr>
<td>

unknown

</td>
<td>

4

</td>
<td>

always zero?

</td>
</tr>
<tr>
<td>

content

</td>
<td>

variable

</td>
<td>

the actual, encoded string content

</td>
</tr>
<tr>
<td>

padding

</td>
<td>

0..3

</td>
<td>

zero to three bytes of padding to get the length of the whole MHOD to a
multiple of 4, note that this is not included in the string length but
<b>is</b> included in the total length

</td>
</tr>
</tbody>
</table>

  

<div align="right">

<small>→ <a title="ITunesDB/Misc. Files">Subpage: Misc.
Files</a></small>

</div>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesSD_file"></a>

<h2>

iTunesSD file

</h2>

The iTunesSD file is in a big-endian byte order. It consists of a header
followed by a bunch of entries, one after the other. The format is much
simpler than the iTunesDB. Only the iPod Shuffle is known to use this
file at the moment. The Shuffle uses only this file for playing songs,
but nevertheless a valid iTunesDB must be present on the device. When
connecting to iTunes, only the iTunesDB is read back, not the iTunesSD.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iTunesSD header format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

num songs

</td>
<td>

3

</td>
<td>

Number of song entries in the file.

</td>
</tr>
<tr>
<td>

unknown

</td>
<td>

3

</td>
<td>

0x010600? iTunes 7.2 puts 0x010800 here

</td>
</tr>
<tr>
<td>

header size

</td>
<td>

3

</td>
<td>

size of the header (0x12, 18 byte header)

</td>
</tr>
<tr>
<td>

unknown

</td>
<td>

9

</td>
<td>

possibly zero padding

</td>
</tr>
<tr>
<td colspan="3">

rest of header is NOT zero padded

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iTunesSD song entry format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

size of entry

</td>
<td>

3

</td>
<td>

Always 0x22e (558 bytes)

</td>
</tr>
<tr>
<td>

unk1

</td>
<td>

3

</td>
<td>

unknown (always 0x5aa501 ?)

</td>
</tr>
<tr>
<td>

starttime

</td>
<td>

3

</td>
<td>

Start Time, in 256 millisecond increments - e.g. 60 seconds = 0xea (234
dec). The reason for this is that the iPodShuffle has only a simplistic
"clock". Every millisecond it increments an 8 bit counter. When the
counter overflows, this causes an interrupt or something like that which
causes it to increment this "clock" value. Very simple clock, easy to do
in a an 8-bit register. Basically multiply whatever value you find here
by 0.256 to convert it to seconds. Leaving this as zero means it plays
from the beginning of the file.

</td>
</tr>
<tr>
<td>

unk2

</td>
<td>

3

</td>
<td>

unknown (always 0?)

</td>
</tr>
<tr>
<td>

unk3

</td>
<td>

3

</td>
<td>

Unknown, but seems to be associated with start time (start time of 0xea
resulted in unk3 = 0x1258ee)

</td>
</tr>
<tr>
<td>

stoptime

</td>
<td>

3

</td>
<td>

Stop Time, also in 256 millisecond increments - e.g. 120 seconds = 0x1d4
(468 dec). Leaving this as zero means it'll play to the end of the file.

</td>
</tr>
<tr>
<td>

unk4

</td>
<td>

3

</td>
<td>

unknown

</td>
</tr>
<tr>
<td>

unk5

</td>
<td>

3

</td>
<td>

Unknown, but seems to be associated with stop time (stop time of 0x1d4
resulted in unk5 = 0x24a830)

</td>
</tr>
<tr>
<td>

volume

</td>
<td>

3

</td>
<td>

Volume - ranges from 0x00 (-100%) to 0x64 (0%) to 0xc8 (100%)

</td>
</tr>
<tr>
<td>

file\_type

</td>
<td>

3

</td>
<td>

0x01 = MP3, 0x02 = AAC, 0x04 = WAV

</td>
</tr>
<tr>
<td>

unk6

</td>
<td>

3

</td>
<td>

0x200?

</td>
</tr>
<tr>
<td>

filename

</td>
<td>

522

</td>
<td>

filename of the song, padded at the end with 0's, in UTF-16. Note:
forward slashs are used here, not colons like in the iTunesDB - for
example "/iPod\_Control/Music/F00/Song.mp3".

</td>
</tr>
<tr>
<td>

shuffleflag

</td>
<td>

1

</td>
<td>

If this value is 0x00, the song will be skipped in while the player is
in shuffle mode. Any other value (iTunes uses 0x01) will allow it be
played in both normal and shuffle modes. By default, iTunes 4.7.1 sets
this flag to 0x00 for audiobooks (.m4b and .aa), so they aren't played
in shuffle mode.

</td>
</tr>
<tr>
<td>

bookmarkflag

</td>
<td>

1

</td>
<td>

If this value is 0x00, the song will not be bookmarkable (i.e. its
playback position won't be saved when switching to a different song).
Any other value will make it bookmarkable. Unlike hard drive based
iPods, all songs can be marked as bookmarkable - not just .m4b and .aa.
However, iTunes might not use this bookmark information for songs other
than actual audiobooks. By default, iTunes 4.7.1 sets this flag to 0x01
for audiobooks (.m4b and .aa), and 0x00 for everything else.

</td>
</tr>
<tr>
<td>

unknownflag

</td>
<td>

1

</td>
<td>

This has never been observed to be anything other than 0x00, and setting
it other values seemed to no effect.

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesStats_File"></a>

<h2>

iTunesStats File

</h2>

This seems to be the equivalent of the "Play Counts" file, where the
Shuffle stores data that iTunes reads back in. Strangely, this file
\*is\* byte reversed (little-endian). So a decimal 18 would be
represented as 0x12 0x00 0x00, for example.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iTunesStats header format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

num songs

</td>
<td>

3

</td>
<td>

Number of song entries in the file (same as number of songs on the iPod
Shuffle).

</td>
</tr>
<tr>
<td>

3

</td>
<td>

unknown

</td>
<td>

3

</td>
<td>

zero padding?

</td>
</tr>
</tbody>
</table>
<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iTunesStats song entry format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

size of entry

</td>
<td>

3

</td>
<td>

0x12 (18 bytes)

</td>
</tr>
<tr>
<td>

3

</td>
<td>

bookmarktime

</td>
<td>

3

</td>
<td>

Bookmarked position in the song in 256 millisecond increments (e.g. 0xee
-\> 60928 milliseconds = 60.928 seconds). If a song's playcount is zero,
this field is 0xffffff. Otherwise, it is 0x0 or the bookmarked time.

</td>
</tr>
<tr>
<td>

6

</td>
<td>

unk1

</td>
<td>

3

</td>
<td>

Somehow associated with bookmarktime (bookmarktime of 0x15f results in
unk1 = 0x16db19, 0x1e4 -\> unk1 = 0x1f3215). Probably the same
formatting as seen in iTunesSD unk3 & unk5... Whatever the purpose, it
doesn't seem to be used on the Shuffle - just supplying a valid
bookmarktime works fine.

</td>
</tr>
<tr>
<td>

9

</td>
<td>

unk2

</td>
<td>

3

</td>
<td>

always 0?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

playcount

</td>
<td>

3

</td>
<td>

Number of times this track has played to the end.

</td>
</tr>
<tr>
<td>

15

</td>
<td>

skippedcount

</td>
<td>

3

</td>
<td>

Number of times that the user has skipped past this song before its end.

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesShuffle_File"></a>

<h2>

iTunesShuffle File

</h2>

This file contains a list of the track index numbers, one after another.
Each number is 3 bytes in length, and in little endian byte order
(reverse byte order).

The reason for this file is that it ensures that although the tracks are
shuffled, they remain in the same order until you resync the
iPodShuffle. That way you can turn the device on and off as many times
as you like, and although the songs are shuffled, you can still back up
and so forth.

iTunesShuffle can either be created by the application, or the iPod
Shuffle will automatically create it when necessary. It can also be
recreated on the Shuffle by pressing the play/pause button three times
within one second, thus reshuffling the order of songs on the fly.

It also appears that the songs are automatically reshuffled if, while in
shuffle playback mode, the player reaches the end of the playlist. So
even if an application creates the iTunesShuffle file, the iPod Shuffle
can and will replace it.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesPState_File"></a>

<h2>

iTunesPState File

</h2>

This file keeps track of the current playback state of the iPod shuffle.
It contains seven 24-bit little endian values (i.e. 3 bytes per number,
reverse byte order). The iPod re-creates the file with default if it is
missing.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iTunesPState format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

volume

</td>
<td>

3

</td>
<td>

The current playback volume. 0x1d (29) is the default value. The maximum
value is 0x26 (38), at least for the European model.

</td>
</tr>
<tr>
<td>

shufflepos

</td>
<td>

3

</td>
<td>

Current track number in the shuffle order. Value 0x00 denotes the first
song. In normal playback mode, this value is equal to the entry number
in the iTunesSD file (see next field).

</td>
</tr>
<tr>
<td>

trackno

</td>
<td>

3

</td>
<td>

Current track number in iTunesSD file order. Value 0x00 denotes the
first song.

</td>
</tr>
<tr>
<td>

shuffleflag

</td>
<td>

3

</td>
<td>

0x00 in ordered playback mode, 0x01 in shuffle mode.

</td>
</tr>
<tr>
<td>

trackpos

</td>
<td>

3

</td>
<td>

Current playback position inside the track, in bytes(?)

</td>
</tr>
<tr>
<td>

unk1

</td>
<td>

3

</td>
<td>

high bits for trackpos?

</td>
</tr>
<tr>
<td>

unk2

</td>
<td>

3

</td>
<td>

Always 0x01?

</td>
</tr>
</tbody>
</table>

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesControl_File"></a>

<h2>

iTunesControl File

</h2>

It is quite big (several MB), and has no obvious header identifier.  
It seems to have a single data section written over and over again which
equals 256 bytes each.  
You can see a pattern in the hex data section of growing word values.

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesPrefs_File"></a>

<h2>

iTunesPrefs File

</h2>

It seems to have a fixed length of 236 bytes.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>iTunesPrefs format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

frpd (0x66 0x72 0x70 0x64)

</td>
</tr>
<tr>
<td>

4

</td>
<td>

unk1

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

5

</td>
<td>

unk2

</td>
<td>

1

</td>
<td>

00?

</td>
</tr>
<tr>
<td>

6

</td>
<td>

unk3

</td>
<td>

1

</td>
<td>

03?

</td>
</tr>
<tr>
<td>

7

</td>
<td>

unk4

</td>
<td>

1

</td>
<td>

00?

</td>
</tr>
<tr>
<td>

8

</td>
<td>

iPod set up

</td>
<td>

1

</td>
<td>

0x00: iTunes has not set up iPod, 0x01: iTunes has set up iPod. Checked
by iTunes to determine whether to present iPod set up dialog box.

</td>
</tr>
<tr>
<td>

9

</td>
<td>

Open iTunes when attached

</td>
<td>

1

</td>
<td>

Open iTunes when iPod is Attached, 01 for checked.

</td>
</tr>
<tr>
<td>

10

</td>
<td>

Manual/Automatic Sync flag

</td>
<td>

1

</td>
<td>

00 appears to be "Manually manage my songs", 01 appears to be "Automatic
Sync"

</td>
</tr>
<tr>
<td>

11

</td>
<td>

Sync Type

</td>
<td>

1

</td>
<td>

01 Entire Library, 02 Selected Playlists

</td>
</tr>
<tr>
<td>

12

</td>
<td>

iTunes Music Library Link Identifier

</td>
<td>

8

</td>
<td>

8 byte identifier of the last iTunes library synced to this iPod.
Checked by iTunes to prevent automatic updates when you connect the iPod
to other computers with iTunes/Libraries

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk10

</td>
<td>

4

</td>
<td>

unknown, possibly part of unk11

</td>
</tr>
<tr>
<td>

24

</td>
<td>

unk11

</td>
<td>

4

</td>
<td>

unknown, possibly part of unk10

</td>
</tr>
<tr>
<td>

28

</td>
<td>

unk12

</td>
<td>

1

</td>
<td>

00? Various flags of some kind?

</td>
</tr>
<tr>
<td>

29

</td>
<td>

unk13

</td>
<td>

1

</td>
<td>

00?

</td>
</tr>
<tr>
<td>

30

</td>
<td>

unk14

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

31

</td>
<td>

Enable Disk Use

</td>
<td>

1

</td>
<td>

00 for disabling disk use, 01 to enabling disk use.

</td>
</tr>
<tr>
<td>

32

</td>
<td>

unk16

</td>
<td>

1

</td>
<td>

00?

</td>
</tr>
<tr>
<td>

33

</td>
<td>

unk17

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

34

</td>
<td>

Update Checked

</td>
<td>

1

</td>
<td>

01 for only update checked songs.

</td>
</tr>
<tr>
<td>

35

</td>
<td>

unk19

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

36

</td>
<td>

unk20

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

37

</td>
<td>

padding?

</td>
<td>

12

</td>
<td>

zero padding?

</td>
</tr>
<tr>
<td>

49

</td>
<td>

Show Artwork

</td>
<td>

1

</td>
<td>

1 for Show Artwork in Ipod

</td>
</tr>
<tr>
<td>

50

</td>
<td>

padding?

</td>
<td>

2

</td>
<td>

zero padding?

</td>
</tr>
<tr>
<td>

52

</td>
<td>

Synchronize Photos

</td>
<td>

1

</td>
<td>

Synchronize Photos with iPod

</td>
</tr>
<tr>
<td>

53

</td>
<td>

unk21

</td>
<td>

2

</td>
<td>

0x2010?

</td>
</tr>
<tr>
<td>

55

</td>
<td>

Store Hi-res photos on iPod

</td>
<td>

1

</td>
<td>

1 for yes, 0 for no.

</td>
</tr>
<tr>
<td>

56

</td>
<td>

padding?

</td>
<td>

16

</td>
<td>

zero padding?

</td>
</tr>
<tr>
<td>

72

</td>
<td>

Transcode

</td>
<td>

1

</td>
<td>

01 For Transcode higher bitrate songs to 128 AAC (Shuffle only)

</td>
</tr>
<tr>
<td>

73

</td>
<td>

Keep Ipod in the source list

</td>
<td>

1

</td>
<td>

Keep this ipod in the source list, 1 for true (Shuffle only?)

</td>
</tr>
<tr>
<td>

74

</td>
<td>

unk23

</td>
<td>

15

</td>
<td>

A bunch of other flags?

</td>
</tr>
<tr>
<td>

89

</td>
<td>

Selected Podcast Sync Only

</td>
<td>

1

</td>
<td>

0x01 = Sync All Podcasts, 0x02 = Sync Selected Podcasts Only

</td>
</tr>
<tr>
<td>

90

</td>
<td>

Manual/Automatic Podcast Sync

</td>
<td>

1

</td>
<td>

0x00 = Manually Manage Podcasts (selected flag is meaningless in this
case), 0x01 = Autosync podcasts

</td>
</tr>
<tr>
<td>

91

</td>
<td>

unk24

</td>
<td>

5

</td>
<td>

Five other flags?

</td>
</tr>
<tr>
<td>

96

</td>
<td>

Identifier?

</td>
<td>

8

</td>
<td>

Same 8 byte ID as before

</td>
</tr>
<tr>
<td>

104

</td>
<td>

songs on ipod

</td>
<td>

2

</td>
<td>

Somehow related to the songs / disc space allowed on the shuffle.
Setting songs to 0 or setting filespace to 0 sets this to 0. (0x2000 for
non-shuffles?)

</td>
</tr>
<tr>
<td>

106

</td>
<td>

filespace saved on ipod

</td>
<td>

2

</td>
<td>

Somehow related to the songs / disc space allowed on the shuffle.
Setting filespace to 0 sets this to 0. (0x0000 for non-shuffles?)

</td>
</tr>
<tr>
<td>

108

</td>
<td>

padding?

</td>
<td>

16

</td>
<td>

zero padding?

</td>
</tr>
<tr>
<td>

124

</td>
<td>

Sound Check

</td>
<td>

1

</td>
<td>

Use sound Check (shuffle only setting? Perhaps for transcoding
purposes?)

</td>
</tr>
<tr>
<td>

125

</td>
<td>

padding?

</td>
<td>

111

</td>
<td>

zero padding? undetermined flags?

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="winPrefs_File"></a>

<h2>

winPrefs File

</h2>

It seems to have a fixed length of 16 bytes.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>winPrefs format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

frpw (0x66 0x72 0x70 0x77)

</td>
</tr>
<tr>
<td>

unk1

</td>
<td>

4

</td>
<td>

Unknown.

</td>
</tr>
<tr>
<td>

unk2

</td>
<td>

4

</td>
<td>

Unknown.

</td>
</tr>
<tr>
<td>

unk3

</td>
<td>

4

</td>
<td>

Unknown.

</td>
</tr>
</tbody>
</table>

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="DeviceInfo_File"></a>

<h2>

DeviceInfo File

</h2>

This file has info on the iPod user's computer.  
It seems to have a fixed length of 1536 bytes.  
Each field is zero padded and in UTF-16LE text format.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>DeviceInfo format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

Length of iPod name

</td>
<td>

2

</td>
<td>

The length in symbols (not in bytes) of the string immediately
following.

</td>
</tr>
<tr>
<td>

iPod name

</td>
<td>

510

</td>
<td>

The ipod name as defined in iTunes.

</td>
</tr>
<tr>
<td>

Length of User name

</td>
<td>

2

</td>
<td>

The length in symbols (not in bytes) of the string immediately
following.

</td>
</tr>
<tr>
<td>

User name

</td>
<td>

510

</td>
<td>

The logged on username who last used the iPod with iTunes.

</td>
</tr>
<tr>
<td>

Length of Computer name

</td>
<td>

2

</td>
<td>

The length in symbols (not in bytes) of the string immediately
following.

</td>
</tr>
<tr>
<td>

Computer name

</td>
<td>

510

</td>
<td>

The name of the computer who used the iPod.

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iEKInfo_File"></a>

<h2>

iEKInfo File

</h2>

New file I found recently on a 3G iPod. Unknown purpose. The file looks
to be big endian, and similar to the chapter info records found inside
podcasts with chapters (as well as similar to the chapter info
structures found in the iTunesDB file. One odd characteristic is that
the file appears to contain no 4 character identifier at the beginning.
Similar in structure to standard MPEG 4 metadata layouts.

<a class="external" title="http://www.adg.us/article188.html" rel="nofollow">some
more information on the structure of this
file</a><span class="urlexpansion"> (<i><http://www.adg.us/article188.html></i>)</span>

Info copied from above webpage (adg.us):

    FILE HEADER:
    ============
     &lt;uint32&gt; - Size of file data (excluding this 64-byte header)
     &lt;60-zero-bytes&gt; - Padding?
    FILE DATA BEGINS HERE:
    ======================
     &lt;12-zero-bytes&gt; - Padding?
     &lt;ATOM:"sean"&gt;
       &lt;uint32&gt; - ID/number (should be 1)
       &lt;uint32&gt; - No. of subatoms (should be at least 3)
       &lt;uint32&gt; - Four zero bytes
       &lt;ATOM:"sym "&gt;
         &lt;uint32&gt; - ID/number (should be 1)
         &lt;uint32&gt; - No. of subatoms ("sess" subatoms) - I expect it should be
                    at least 1
         &lt;uint32&gt; - Four zero bytes
         ONE OR MORE "sess" ATOMS:
         =========================
           &lt;ATOM:"sess"&gt;
             &lt;uint32&gt; - Session ID - Used to identify which keys are encrypted
                        with this session
             &lt;uint32&gt; - No. of subatoms - Should be 2 (one "valu" subatom, one
                        "index" subatom)
             &lt;uint32&gt; - Four zero bytes
             &lt;ATOM:"valu"&gt;
               &lt;uint32&gt; - ID/number (should be 1)
               &lt;uint32&gt; - No. of subatoms (should be zero)
               &lt;uint32&gt; - Four zero bytes
               &lt;128 bytes of binary data&gt; - The session key or something???
               &lt;128 zero bytes&gt; - I don't know if all sessions are structured
                                  this way or not...  These last 128 bytes
                                  are ignored for playback purposes.
             --- ("valu")
             &lt;ATOM:"indx"&gt;
               &lt;uint32&gt; - ID/number (should be 1)
               &lt;uint32&gt; - No. of subatoms (should be zero)
               &lt;uint32&gt; - Four zero bytes
               &lt;uint32&gt; - Every index I've seen thus far has the value six
                          ('6') here.  Changing this broke playback on my
                          iPod, so it's important for playback.
             --- ("indx")
           --- ("sess")
         ===
       --- ("sym ")
       &lt;ATOM:"user"&gt;
         &lt;uint32&gt; - User ID (a.k.a. DSID)
         &lt;uint32&gt; - Number of keys, or no. of "key " subatoms
         &lt;uint32&gt; - Four zero bytes
         ONE OR MORE "key " ATOMS:
         =========================
           &lt;ATOM:"key "&gt;
             &lt;uint32&gt; - Key ID or key number
             &lt;uint32&gt; - Number of subatoms (should be 1)
             &lt;uint32&gt; - Four zero bytes
             &lt;ATOM:"valu"&gt;
               &lt;uint32&gt; - Session ID used to encrypt this key's contents
               &lt;uint32&gt; - No. of subatoms (Should be zero)
               &lt;uint32&gt; - Four zero bytes
               &lt;16 bytes of binary data&gt; - The key data, encrypted with the
                                           specified session?
             --- ("valu")
           --- ("key")
         ===
       --- ("user")
       THREE "guid" ATOMS: (Are there ever more?)
       ===================
         &lt;ATOM:"guid"&gt;
           &lt;uint32&gt; - GUID ID or number (should be one)
           &lt;uint32&gt; - Number of subatoms (should be zero)
           &lt;uint32&gt; - Four zero bytes
           &lt;GUID Data Bytes&gt; - 256 bytes for the first 'guid' atom...
             The first 9 bytes, if changed, appear to break playback on my
             iPod.  The next three bytes contained data, but I could change
             it without breaking playback.  Byte number 12 (position 11 if
             the first byte is at position zero) was 1 for a while on my
             iPod, but later was 2.  I didn't check to see if this changed
             when I moved the iPod from one host computer to another.
             On a 4G iPod, the first 8 bytes of this field are the iPod's FireWire ID.
         --- ("guid")
         &lt;ATOM:"guid"&gt;
           &lt;uint32&gt; - GUID ID or number (should be two)
           &lt;uint32&gt; - Number of subatoms (should be zero)
           &lt;uint32&gt; - Four zero bytes
           &lt;GUID Data Bytes&gt; - 256 bytes for the second 'guid' atom...
             This has got to be a global unique identifier based on which
             host computer installation of iTunes the iPod is associated
             with.  It changed when I moved my iPod from one of my host
             computers to another.  But it didn't change when other
             playlist and/or authorization changes changed the 'sess'
             session atom.  The last 128 bytes are all zero-bytes, and
             are ignored during playback.  I was able to randomly change
             the last 128 bytes to anything without affecting playback.
         --- ("guid")
         &lt;ATOM:"guid"&gt;
           &lt;uint32&gt; - GUID ID or number (should be three)
           &lt;uint32&gt; - Number of subatoms (should be zero)
           &lt;uint32&gt; - Four zero bytes
           &lt;GUID Data Bytes&gt; - 4 bytes
             This is the final 'guid' atom, the last one, and yes, it's
             only 4 bytes long, containing probably a &lt;uint32&gt; value.
             It has been value six on my iPod.  Changing it broke my
             iPod's ability to play audio files.  I see that the value
             six shows up here, as well in the above 'indx' atom within
             the 'sess' atom.  I can't help but think these two values
             may be related somehow.     
         --- ("guid")
       ===
     --- ("sean")
    === FILE DATA ENDS HERE

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="WABContactsGroup_File"></a>

<h2>

WABContactsGroup File

</h2>

New file I found recently on a 3G iPod. Unknown purpose, although
probably related to the new Contacts sync, given the name. Assuming that
this may be different for people syncing to Outlook instead of to the
Windows Address Book (WAB). The file is 56 bytes in size.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>WABContactsGroup format</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

header identifier

</td>
<td>

4

</td>
<td>

frpd (0x66 0x72 0x70 0x64) (Yes, this matches the iTunesPrefs file
identifier)

</td>
</tr>
<tr>
<td>

padding?

</td>
<td>

8

</td>
<td>

zero padding

</td>
</tr>
<tr>
<td>

unk1

</td>
<td>

4

</td>
<td>

0x04 02 00 00. Setting flags, perhaps?

</td>
</tr>
<tr>
<td>

padding?

</td>
<td>

40

</td>
<td>

zero padding?

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesLock_File"></a>

<h2>

iTunesLock File

</h2>

This file is created right before iTunes modifies or reads anything on
the iPod, and is deleted when it is finished. Does not contain any data,
and is probably only used to make sure only one program at a time tries
accessing the important files on the iPod at one time.

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="PhotosFolderName"></a>

<h2>

PhotosFolderName

</h2>

Lists the watched folders watched and syncronized with the Ipod. Found
on the iPod video (probably on the Photo as well). It is a very simple
format with the length of the folder name, preceeded by the folder name
in unicode letters.

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="PhotoFolderPrefs"></a>

<h2>

PhotoFolderPrefs

</h2>

Stores Photo Folder preferences.

  

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>PhotoFolderPrefs</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

frpd same as iTunesPrefs

</td>
</tr>
<tr>
<td>

4

</td>
<td>

unk1

</td>
<td>

4

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unk2

</td>
<td>

4

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

unk3

</td>
<td>

4

</td>
<td>

 ?

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk3

</td>
<td>

80

</td>
<td>

zero padding?

</td>
</tr>
<tr>
<td>

96

</td>
<td>

folder name

</td>
<td>

 ??

</td>
<td>

The watch folders names

</td>
</tr>
</tbody>
</table>

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="PhotoFolderAlbums"></a>

<h2>

PhotoFolderAlbums

</h2>

Stores Photo Albums .

  

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>PhotoFolderAlbums</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0

</td>
<td>

header identifier

</td>
<td>

4

</td>
<td>

frpd same as iTunesPrefs

</td>
</tr>
<tr>
<td>

4

</td>
<td>

unk1

</td>
<td>

2

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

6

</td>
<td>

unk2

</td>
<td>

2

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

8

</td>
<td>

unk3

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

9

</td>
<td>

unk4

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

10

</td>
<td>

unk5

</td>
<td>

2

</td>
<td>

00?

</td>
</tr>
<tr>
<td>

12

</td>
<td>

Number of Folders

</td>
<td>

4

</td>
<td>

Number of folders to that are directly below the folder pointed out in
PhotoFolderPrefs. From offset 100 on, the folder name datastructure will
repeat this many times.

</td>
</tr>
<tr>
<td>

16

</td>
<td>

unk7

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

17

</td>
<td>

unk8

</td>
<td>

1

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

18

</td>
<td>

unk9

</td>
<td>

2

</td>
<td>

00?

</td>
</tr>
<tr>
<td>

20

</td>
<td>

unk10

</td>
<td>

4

</td>
<td>

01?

</td>
</tr>
<tr>
<td>

26

</td>
<td>

unk11

</td>
<td>

74

</td>
<td>

zero padding?

</td>
</tr>
<tr>
<td>

100

</td>
<td>

is synched

</td>
<td>

4

</td>
<td>

Whether or not the folder is synched. 1 if is, 0 if it isn't.

</td>
</tr>
<tr>
<td>

104

</td>
<td>

string legnth

</td>
<td>

2

</td>
<td>

Length of the name of the albums to follow

</td>
</tr>
<tr>
<td>

106

</td>
<td>

album name

</td>
<td>

510

</td>
<td>

Name of the album to follow

</td>
</tr>
<tr>
<td colspan="4">

From 100-514 repeats as many times as in Number of folders (see offset
12)

</td>
</tr>
</tbody>
</table>
<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesPlaylists"></a>

<h2>

iTunesPlaylists

</h2>

  
Keeps track of the iTunesPlaylists???

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesPodcasts"></a>

<h2>

iTunesPodcasts

</h2>

  
Keeps track of the iTunesPodcasts???

  

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="iTunesVideoPlaylists"></a>

<h2>

iTunesVideoPlaylists

</h2>

  
Keeps track of the iTunesVideoPlaylists???

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="Preferences"></a>

<h2>

Preferences

</h2>

Located at /iPod\_Control/Device/Preferences.

<table border="1" cellpadding="5" cellspacing="0">
<caption>

<b>Preferences</b>

</caption>
<tbody>
<tr>
<th style="background:#efefef;">

offset

</th>
<th style="background:#efefef;">

field

</th>
<th style="background:#efefef;">

size

</th>
<th style="background:#efefef;">

value

</th>
</tr>
<tr>
<td>

0x6BC (1724)

</td>
<td>

DST-Setting (3g nano)

</td>
<td>

1

</td>
<td>

Set to 0x3C (=60) if DST is set. Otherwise 0x00

</td>
</tr>
<tr>
<td>

2808

</td>
<td>

Language Selection

</td>
<td>

1

</td>
<td>

Defines the menu language selected in your iPod doing
Settings-\>Language. English is 0.

</td>
</tr>
<tr>
<td>

2832

</td>
<td>

Timezone Information

</td>
<td>

1

</td>
<td>

Timezone information.

</td>
</tr>
<tr>
<td>

0xB50 (2896)

</td>
<td>

Volume Limit

</td>
<td>

1

</td>
<td>

Volume limit value on firmware 1.1.1 for 5G and nano. Range: 0-64

</td>
</tr>
<tr>
<td>

0xB70 (2928)

</td>
<td>

Region (3g nano)

</td>
<td>

1

</td>
<td>

Selected region/timezone: 0x4f = London, 0x55 = Zurich, 0x5d = Geneva
... Seen on 3g-Nano. This might also be used to calculate the timezone
offset because changing the timezone from Geneva to London only changes
this offset. (Or is it a bug? The Nano displays London as UTC-1 (!)

</td>
</tr>
</tbody>
</table>

I'm unsure what is the best way to parse the timezone information field,
but the previous explanation that it's the timezone information in 30
minutes steps is wrong, it doesn't hold up during DST. Something that
works is to substract 0x19 (this corresponds to UTC+0) to that value.
The resulting value divided by 2 gives the timezone (in hours), and the
less significant bit (ie the resulting value modulo 2) indicates if DST
is active or not (1 =\> DST active). When the DST bit is set, 1 hour
must be added to the timezone value.

  
<b>Example:</b>Selecting <i>Paris (DST)</i> will set this offset to
0x1c: (0x1c - 0x19) = 0x03 0x03 / 2 = 1 0x03 % 2 = 1 which gives us a
GMT+2 timezone

<div class="editsection" style="float:right;margin-left:5px;">
</div>

<a name="_volumelocked"></a>

<h2>

\_volumelocked

</h2>

Located at /iPod\_Control/Device/

Exists when a 4-digit combination has been set for the Volume Limit on
5Gs and nanos (firmware 1.1.1). File length is 0x20. Appears to be
encrypted? However, deleting the file removes the volume limit :-)

<div class="printfooter">

Retrieved from
"<a href="http://www.ipodlinux.org/"><http://ipodlinux.org/ITunesDB></a>"

</div>
<div id="catlinks">

<a title="Special:Categories">Categories</a>:
<a title="Category:Documentation">Documentation</a> \|
<a title="Category:Development">Development</a>

</div>
<div class="visualClear">
</div>
</div>
</div>
</div>
<div id="column-one">
<div id="p-cactions" class="portlet">
<h5>

Views

</h5>

-   <a href="http://www.ipodlinux.org/">Article</a>

</div>
<div class="portlet" id="p-personal">
<h5>

Personal tools

</h5>
<div class="pBody">

</div>
</div>

<div class="portlet" id="p-nav">
<h5>

Navigation

</h5>
<div class="pBody">

-   <a href="http://www.ipodlinux.org/" title="ipod linux">Ipod
    Linux</a>

<hr>

-   <a href="http://www.ipodlinux.org/Documentation/" title="Documentation">Documentation</a>
-   <a href="http://www.ipodlinux.org/Downloads/" title="Downloads">Downloads</a>
-   <a href="http://www.ipodlinux.org/Screenshots/" title="Screenshots">Screenshots</a>
-   <a href="http://www.ipodlinux.org/Contributors.html" title="Contributors">Contributors</a>
-   <a href="http://www.ipodlinux.org/Links.html" title="Links">Links</a>
-   <a href="http://www.ipodlinux.org/Donations.html" title="Donations">Donations</a>

<hr>

-   <a href="http://www.ipodlinux.org/Special_Recentchanges.html" title="Special:Recentchanges">Recent
    Changes</a>
-   <a href="http://www.ipodlinux.org/Special_Allpages.html" title="Special:Allpages">All
    Pages</a>
-   <a href="http://www.ipodlinux.org/Help_Contents.html" title="Help:Contents">Help</a>

</div>
</div>
<div class="portlet" id="p-tb">
<h5>

Toolbox

</h5>
<div class="pBody">

-   <a href="http://www.ipodlinux.org/Generations/">ipod generations</a>
-   <a href="http://www.ipodlinux.org/Special_Recentchanges.html">Related
    changes</a>
-   <a href="http://www.ipodlinux.org/Special_Specialpages.html">Special
    pages</a>

</div>



</div>
</div>
<div class="visualClear">
</div>
<div id="footer">

-   This page was last modified 18:55, 26 Apr 2017.
-   This page has been accessed 125659 times.
-   Content is available under
    <a href="http://www.gnu.org/copyleft/fdl.html" class="external" title="http://www.gnu.org/copyleft/fdl.html" rel="nofollow">GNU
    Free Documentation License 1.2</a>.
-   <a href="http://www.ipodlinux.org/iPodLinux_About.html" title="iPodLinux:About">About
    wikiPodLinux</a>
-   <a href="http://www.ipodlinux.org/iPodLinux_General_disclaimer.html" title="iPodLinux:General disclaimer">Disclaimers</a>

</div>
</div>

<noscript>
<div>

</div>
</noscript>
<div style="position: static !important;">
</div>
</body>
</html>
