# About

Provides API integration with [Spotify](https://en.wikipedia.org/wiki/Spotify)

After running the iTunesDB parser application on an iTunesDB file, you end up with a music CSV file that contains info about your songs. This tool lets you then import that CSV file into a Spotify playlist.

If you want to use this tool with your _own_ CSV files, see `test.csv` for an example of the format.

## Usage info

```
usage: spotify_integration.py [-h] -f CSV_FILE [-t TRACK_COLUMN] -a API_CREDENTIALS_FILE

Read CSV files containing songs and search for them in Spotify

options:
  -h, --help            show this help message and exit
  -f CSV_FILE, --csv-file CSV_FILE
                        Path to the CSV file containing the songs
  -t TRACK_COLUMN, --track-column TRACK_COLUMN
                        Name of the column containing the track names
  -a API_CREDENTIALS_FILE, --api-credentials-file API_CREDENTIALS_FILE
                        Path to JSON file containing API credentials, see
                        spotify_api_credentials.json for the format
```

# API instructions

See `spotify_api_credentials.json` for example of format
Both are 32-character alphanumeric values
See: https://developer.spotify.com/documentation/web-api/concepts/apps
for instructions on how to generate!

After running you will be sent to a authorization prompt in your browser and then redirected to a URL

The application will ask you to paste that URL to retrieve the code

