import csv
import spotipy
import argparse
import socket
import json
import typing
import time
import lxml.etree
import datetime
import sys

from spotipy.oauth2 import SpotifyClientCredentials
from spotipy import SpotifyOAuth

UNITED_STATES_ISO_3166_1_CODE = "US"
DEFAULT_QUERY_LIMIT = 3
DEFAULT_SPOTIFY_API_SCOPE = "playlist-modify-private"


def get_track_ids_from_songs(song_and_artist_info: typing.List, spotify_api_obj):
    """
    Shared between the XML and CSV formats
    """

    spotify_track_ids: typing.List = []
    num_songs_found = 0

    sp_obj = spotipy.Spotify(auth_manager=SpotifyClientCredentials(
        client_id=spotify_api_obj["client_id"], client_secret=spotify_api_obj["client_secret"]))

    for song_idx, song_obj in enumerate(song_and_artist_info):

        song_title = song_obj.get("title")
        song_artist = song_obj.get("artist")

        spotify_search_results = sp_obj.search(q=f"track:{song_title} artist:{
            song_artist}", type="track", limit=DEFAULT_QUERY_LIMIT, market=UNITED_STATES_ISO_3166_1_CODE)

        if spotify_search_results["tracks"]["total"] == 0:
            # print(f"Song '{song_title}' by '{song_artist}' (idx {song_idx}) not found on Spotify!")
            continue

        else:
            num_songs_found += 1
            track_id = spotify_search_results["tracks"]["items"][0]["id"]
            spotify_track_ids.append(track_id)

    print(f"[DEBUG] Found {num_songs_found} on Spotify, out of {
          len(song_and_artist_info)}")
    return spotify_track_ids


def get_songs_from_csv(csv_file, track_title_column_name):

    songs_and_artists: typing.List = []

    with open(csv_file, mode='r') as csv_file:

        csv_reader = csv.DictReader(csv_file)

        for csv_row in csv_reader:

            song_title = csv_row[track_title_column_name]
            song_artist = csv_row["Artist"]

            if not song_title or not song_artist:
                print("Skipping row! Missing song title or song artist")
                continue

            # Must use the same key names as the XML version for compatibility
            songs_and_artists.append(
                dict({"title": song_title, "artist": song_artist}))

    print(f"[DEBUG] Finished extracting {
          len(songs_and_artists)} songs from CSV file")


def create_playlist_from_tracks(spotify_tracks: list, playlist_name: str, playlist_description: str, spotify_api_obj) -> str:

    sp_obj = spotipy.Spotify(auth_manager=SpotifyOAuth(
        client_id=spotify_api_obj["client_id"],
        client_secret=spotify_api_obj["client_secret"],
        redirect_uri=spotify_api_obj["redirect_uri"],
        scope=DEFAULT_SPOTIFY_API_SCOPE
    ))

    user_id = sp_obj.current_user()["id"]
    playlist_id = sp_obj.user_playlist_create(
        user_id, playlist_name, public=False, description=playlist_description)["id"]

    # Extract track IDs
    track_ids = [track["track_id"] for track in spotify_tracks]

    # Spotify allows adding up to 100 tracks per request
    batch_size = 100
    for i in range(0, len(track_ids), batch_size):
        batch = track_ids[i:i + batch_size]
        sp_obj.playlist_add_items(playlist_id, batch)

    return playlist_id


def get_itunes_library_metadata(lxml_root_obj):

    date_exported = lxml_root_obj[0][5].text
    itunes_application_ver = lxml_root_obj[0][7].text
    itunes_music_folder = lxml_root_obj[0][13].text
    itunes_library_id = lxml_root_obj[0][15].text

    return dict({"date_library_exported": date_exported, "itunes_application_version": itunes_application_ver, "itunes_music_folder": itunes_music_folder, "itunes_library_id": itunes_library_id})


def get_song_and_artist_from_xml(lxml_root_obj):

    song_and_artist_list: typing.List = []

    t0 = time.time()

    track_dicts_list = lxml_root_obj[0][17].findall("dict")

    for track_idx, track_dict_obj in enumerate(track_dicts_list):

        song_title, song_artist = track_dict_obj[3].text, track_dict_obj[5].text

        song_info = dict(
            {"title": song_title, "artist": song_artist, "track_library_idx": track_idx})
        song_and_artist_list.append(song_info)

    t1 = time.time()

    print(f"[DEBUG] Extracted {len(song_and_artist_list)} songs in {
          round(t1 - t0, 3)} seconds from XML file")

    return song_and_artist_list


if __name__ == '__main__':

    argparse_parser = argparse.ArgumentParser(
        description="Read CSV or iTunesXML files containing songs and search for them in Spotify")
    argparse_parser.add_argument(
        "-i", "--input-file", help="Path to either 1) the CSV file exported by iTunesDB parser or 2) an XML file exported from iTunes file. The extension is used to determine the procedure", required=True)
    argparse_parser.add_argument(
        "-t", "--track-column", help="Name of the column containing the track names, only required if CSV files are used", required=False, default="Song Title")
    argparse_parser.add_argument("-a", "--api-credentials-file",
                                 help="Path to JSON file containing API credentials, see spotify_api_credentials.json for the format", required=True)
    argparse_args = argparse_parser.parse_args()

    if not argparse_args.input_file:
        raise FileNotFoundError(
            f"Error: Input file '{argparse_args.input_file}' not found")

    if not argparse_args.api_credentials_file:
        raise FileNotFoundError(
            f"Error: API credentials file '{argparse_args.api_credentials_file}' not found! Go to https://developer.spotify.com/documentation/web-api to create one")

    api_obj = json.load(open(argparse_args.api_credentials_file))

    if argparse_args.input_file.endswith("csv"):

        songs_in_csv = get_songs_from_csv(
            argparse_args.csv_file, argparse_args.track_column)

        spotify_track_ids = get_track_ids_from_songs(songs_in_csv, api_obj)

        # Cannot have newline in playlist description
        playlist_description = f"Playlist created on {
            datetime.datetime.now().isoformat()} Created by {socket.gethostname()}"

        new_playlist_id = create_playlist_from_tracks(
            spotify_track_ids, "Test Playlist", playlist_description, api_obj)

        print(f"Created new playlist with ID {new_playlist_id}")

    elif argparse_args.input_file.endswith("xml"):

        itunes_xml_obj = lxml.etree.parse(argparse_args.input_file)

        itunes_xml_root = itunes_xml_obj.getroot()

        if itunes_xml_root.get("version") != "1.0":
            print(f"WARNING - XML file schema does not match the version used for development of this application. May produce invalid results.")
            sys.exit(1)

        itunes_library_metadata = get_itunes_library_metadata(itunes_xml_root)

        songs_info_from_library = get_song_and_artist_from_xml(itunes_xml_root)

        spotify_track_ids = get_track_ids_from_songs(
            songs_info_from_library, api_obj)

        playlist_description = str(itunes_library_metadata)

        new_playlist_id = create_playlist_from_tracks(
            spotify_track_ids, "XML Playlist", playlist_description, api_obj)
