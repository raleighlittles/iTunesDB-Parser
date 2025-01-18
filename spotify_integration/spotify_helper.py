import spotipy
from spotipy.oauth2 import SpotifyOAuth

def create_playlist_from_tracks(spotify_tracks: list, playlist_name: str, playlist_description: str, spotify_api_obj) -> str:
    
    from spotipy import SpotifyOAuth
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