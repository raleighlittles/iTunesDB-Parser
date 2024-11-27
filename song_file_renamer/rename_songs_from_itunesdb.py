import argparse
import os
import csv

if __name__ == "__main__":

    argparse_parser = argparse.ArgumentParser(
        "Apply recovered song data to music files on an iPod")
    argparse_parser.add_argument(
        "-i", "--song-directory", help="Directory to look for songs", required=True, type=str)
    argparse_parser.add_argument(
        "-c", "--itunes-csv", help="Path to iTunesDB CSV file", required=True)
    argparse_parser.add_argument(
        "-o", "--output-directory", help="Directory where to put renamed songs", required=True)

    argparse_args = argparse_parser.parse_args()

    if not os.path.exists(argparse_args.song_directory):
        raise FileNotFoundError(f"Error! Can't find song directory '{
                                argparse_args.song_directory}'")

    if not os.path.exists(argparse_args.itunes_csv):
        raise FileNotFoundError(f"Error! Can't find iTunesDB csv file '{
                                argparse_args.itunes_csv}', make sure you ran the parser")

    song_filenames_and_titles = dict()

    with open(argparse_args.itunes_csv, mode='r') as csv_file_obj:
        csv_reader = csv.DictReader(csv_file_obj)

        for csv_row in csv_reader:
            song_title = csv_row["Song Title"]
            if song_title is None or song_title == "":
                song_title = "UKNOWN_SONG_TITLE"
            song_artist = csv_row["Artist"]
            if song_artist is None or song_artist == "":
                song_artist = "UNKNOWN_ARTIST"
            song_filename = csv_row["Filename"]

            # Only care about the filename matching, not the whole directory path
            song_filenames_and_titles[song_filename.split(
                "/")[-1]] = f"{song_title} - {song_artist}.{song_filename.split(".")[-1]}"

    song_idx = 0

    for candidate_file in os.listdir(argparse_args.song_directory):
        candidate_filename = os.fsdecode(candidate_file)

        if candidate_filename.endswith(".m4a") or candidate_filename.endswith(".mp3"):
            if candidate_filename in song_filenames_and_titles:
                songs_new_filename = song_filenames_and_titles[candidate_filename]
                # print(f"Renaming {candidate_filename} to {songs_new_filename}")
                os.rename(os.path.join(argparse_args.song_directory, candidate_filename), os.path.join(
                    argparse_args.output_directory, songs_new_filename))
                song_idx += 1

    print(f"[DEBUG] Finished renaming {song_idx} songs")
