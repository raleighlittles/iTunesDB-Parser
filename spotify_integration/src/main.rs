use rspotify::clients::BaseClient;
use rspotify::model::SearchType;

fn main() {
    println!("Loading credentials from .env file ..");
    let creds_from_env_file = rspotify::Credentials::from_env().unwrap();
    println!("Credentials loaded! Creating client obj.. ");
    let spotify_cli_obj = rspotify::ClientCredsSpotify::new(creds_from_env_file);

    // Obtaining the access token - must be done before any query is made
    spotify_cli_obj.request_token().unwrap();

    let album_query = "album:arrival artist:abba";
    let result = spotify_cli_obj.search(album_query, SearchType::Album, None, None, Some(10), None);
    match result {
        Ok(album) => println!("Searched album: {album:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }
}
