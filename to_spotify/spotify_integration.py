import csv
import spotipy
import argparse
from spotipy.oauth2 import SpotifyClientCredentials

UNITED_STATES_ISO_3166_1_CODE = "US"
DEFAULT_QUERY_LIMIT = 3

if __name__ == '__main__':

    argparse_parser = argparse.ArgumentParser(description="Read CSV files containing songs and search for them in Spotify")
    argparse_parser.add_argument("-f", "--csv-file", help="Path to the CSV file containing the songs", required=True)
    argparse_parser.add_argument("-t", "--track-column", help="Name of the column containing the track names", required=False, default="Song Title")
    argparse_args = argparse_parser.parse_args()

    num_songs_found = 0
    spotify_track_ids = list()

    if not argparse_args.csv_file:
        raise FileNotFoundError(f"Error: File {argparse_args.csv_file} not found")
    
    with open(argparse_args.csv_file, mode='r') as csv_file:
        csv_reader = csv.DictReader(csv_file)

        sp_obj = spotipy.Spotify(auth_manager=SpotifyClientCredentials(client_id="",
                                                           client_secret=""))
        
        for csv_row in csv_reader:
            song_title = csv_row[argparse_args.track_column]
            song_artist = csv_row["Artist"]

            if not song_title or not song_artist:
                print(f"Skipping row... missing song title or artist")
                continue

            search_results = sp_obj.search(q=f"track:{song_title} artist:{song_artist}", type="track", limit=DEFAULT_QUERY_LIMIT, market=UNITED_STATES_ISO_3166_1_CODE)

            if search_results["tracks"]["total"] == 0:
                print(f"Song '{song_title}' by '{song_artist}' not found on Spotify!")
                continue
            
            else:
                num_songs_found += 1
                track_id = search_results["tracks"]["items"][0]["id"]
                spotify_track_ids.append(track_id)
                

            

    







