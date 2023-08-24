use crate::models::Genre;

pub async fn fetch_genres() -> reqwest::Result<Vec<Genre>> {
    reqwest::get("https://localhost:44316/api/movie-genre")
        .await?
        .json::<Vec<Genre>>()
        .await
}
