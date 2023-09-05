use crate::models::Genre;

use crate::constants::CONFIG;

pub async fn fetch_genres() -> reqwest::Result<Vec<Genre>> {
    reqwest::get(format!("{}/movie-genre", CONFIG.api.path))
        .await?
        .json::<Vec<Genre>>()
        .await
}
