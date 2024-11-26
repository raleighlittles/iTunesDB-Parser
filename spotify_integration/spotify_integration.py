import csv
import spotipy
import argparse
import socket
import json
import datetime

from spotipy.oauth2 import SpotifyClientCredentials
from spotipy.oauth2 import SpotifyOAuth

UNITED_STATES_ISO_3166_1_CODE = "US"
DEFAULT_QUERY_LIMIT = 3
DEFAULT_SPOTIFY_API_SCOPE = "playlist-modify-private"

def get_track_ids_from_csv(csv_file, spotify_api_obj) -> list:

    spotify_tracks = list()
    num_songs_found = 0

    with open(csv_file, mode='r') as csv_file:
        csv_reader = csv.DictReader(csv_file)

        sp_obj = spotipy.Spotify(auth_manager=SpotifyClientCredentials(
            client_id=spotify_api_obj["client_id"], client_secret=spotify_api_obj["client_secret"]))

        for csv_row in csv_reader:
            song_title = csv_row[argparse_args.track_column]
            song_artist = csv_row["Artist"]

            if not song_title or not song_artist:
                print(f"Skipping row... missing song title or artist")
                continue

            search_results = sp_obj.search(q=f"track:{song_title} artist:{
                                           song_artist}", type="track", limit=DEFAULT_QUERY_LIMIT, market=UNITED_STATES_ISO_3166_1_CODE)

            if search_results["tracks"]["total"] == 0:
                print(f"Song '{song_title}' by '{
                      song_artist}' not found on Spotify!")
                continue

            else:
                num_songs_found += 1
                track_id = search_results["tracks"]["items"][0]["id"]
                spotify_tracks.append(
                    dict(song_title=song_title, song_artist=song_artist, track_id=track_id))

    print(f"[DEBUG] Found {num_songs_found} songs on Spotify")
    return spotify_tracks


def create_playlist_from_tracks(spotify_tracks: list, playlist_name: str, playlist_description: str, spotify_api_obj) -> str:

    sp_obj = spotipy.Spotify(auth_manager=SpotifyOAuth(client_id=spotify_api_obj["client_id"], client_secret=spotify_api_obj["client_secret"], redirect_uri=spotify_api_obj["redirect_uri"], scope=DEFAULT_SPOTIFY_API_SCOPE))

    user_id = sp_obj.current_user()["id"]
    playlist_id = sp_obj.user_playlist_create(
        user_id, playlist_name, public=False, description=playlist_description)["id"]

    track_ids = [track["track_id"] for track in spotify_tracks]
    sp_obj.playlist_add_items(playlist_id, track_ids)

    return playlist_id


if __name__ == '__main__':

    argparse_parser = argparse.ArgumentParser(
        description="Read CSV files containing songs and search for them in Spotify")
    argparse_parser.add_argument(
        "-f", "--csv-file", help="Path to the CSV file containing the songs", required=True)
    argparse_parser.add_argument(
        "-t", "--track-column", help="Name of the column containing the track names", required=False, default="Song Title")
    argparse_parser.add_argument("-a", "--api-credentials-file",
                                 help="Path to JSON file containing API credentials, see spotify_api_credentials.json for the format", required=True)
    argparse_args = argparse_parser.parse_args()

    if not argparse_args.csv_file:
        raise FileNotFoundError(
            f"Error: CSV file '{argparse_args.csv_file}' not found")

    if not argparse_args.api_credentials_file:
        raise FileNotFoundError(
            f"Error: API credentials file '{argparse_args.api_credentials_file}' not found! Go to https://developer.spotify.com/documentation/web-api to create one")

    api_obj = json.load(open(argparse_args.api_credentials_file))

    spotify_tracks_and_track_ids = get_track_ids_from_csv(
        argparse_args.csv_file, api_obj)

    playlist_description = f"Playlist created on {
        datetime.datetime.now().isoformat()} \n Created by {socket.gethostname()}"

    new_playlist_id = create_playlist_from_tracks(
        spotify_tracks_and_track_ids, "Test Playlist", playlist_description, api_obj)

    print(f"Created new playlist with ID {new_playlist_id}")
