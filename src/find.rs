//! https://developer.themoviedb.org/reference/find-by-id

use std::borrow::Cow;

use crate::{
    client::Executor,
    movie::MovieShort,
    people::PersonShort,
    tvshow::{EpisodeShort, SeasonShort, TVShowShort},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FindResults {
    pub movie_results: Vec<MovieShort>,
    // FIXME: The API endpoint returns a few `Person` fields (`adult`, `known_for_department` and
    // `popularity`), however it does not have a `also_known_as` field, which is required for a
    // `Person` struct. Do we want to use a `Vec<PersonShort>` and drop the additional info? Make a
    // breaking change and make also_known_as an `Option` in `Person`?
    pub person_results: Vec<PersonShort>,
    pub tv_results: Vec<TVShowShort>,
    pub tv_season_results: Vec<SeasonShort>,
    pub tv_episode_results: Vec<EpisodeShort>,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum ExternalIdSource {
    #[serde(rename = "imdb_id")]
    Imdb,
    #[serde(rename = "facebook_id")]
    Facebook,
    #[serde(rename = "instagram_id")]
    Instagram,
    #[serde(rename = "tvdb_id")]
    Tvdb,
    #[serde(rename = "tiktok_id")]
    Tiktok,
    #[serde(rename = "twitter_id")]
    Twitter,
    #[serde(rename = "wikidata_id")]
    Wikidata,
    #[serde(rename = "youtube_id")]
    Youtube,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Params<'a> {
    pub external_source: ExternalIdSource,
    /// ISO 639-1 value to display translated data for the fields that support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Cow<'a, str>>,
}

impl<'a> Params<'a> {
    pub fn from_external_source(external_source: ExternalIdSource) -> Self {
        external_source.into()
    }

    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }
}

impl From<ExternalIdSource> for Params<'_> {
    fn from(value: ExternalIdSource) -> Self {
        Self {
            external_source: value,
            language: None,
        }
    }
}

impl<E: Executor> crate::Client<E> {
    /// Search for movies, persons, or TV shows/seasons/episodes by an external id.
    /// See [ExternalIdSource] for a list of external id sources.
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    /// use tmdb_api::find::ExternalIdSource;
    /// use tmdb_api::find::Params;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.find_by_id("tt31193180", &ExternalIdSource::Imdb.into()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn find_by_id(
        &self,
        external_id: &str,
        params: &Params<'_>,
    ) -> crate::Result<FindResults> {
        let url = format!("/find/{}", external_id);
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDate;
    use mockito::Matcher;

    use crate::{Client, client::reqwest::Client as ReqwestClient, find::ExternalIdSource};

    #[tokio::test]
    async fn it_works_movie() {
        let mut server = mockito::Server::new_async().await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", "/find/tt31193180")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("external_source".into(), "imdb_id".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/find-by-id-movie.json"))
            .create_async()
            .await;

        let result = client
            .find_by_id("tt31193180", &ExternalIdSource::Imdb.into())
            .await
            .unwrap();

        assert_eq!(result.movie_results.len(), 1);
        assert!(result.person_results.is_empty());
        assert!(result.tv_results.is_empty());
        assert!(result.tv_episode_results.is_empty());
        assert!(result.tv_season_results.is_empty());

        let movie = &result.movie_results[0];
        let inner = &movie.inner;

        assert_eq!(
            inner.backdrop_path,
            Some("/nAxGnGHOsfzufThz20zgmRwKur3.jpg".into())
        );
        assert_eq!(inner.id, 1233413);
        assert_eq!(inner.title, "Sinners");
        assert_eq!(inner.original_title, "Sinners");
        assert_eq!(
            inner.overview,
            "Trying to leave their troubled lives behind, twin brothers return to their hometown to start again, only to discover that an even greater evil is waiting to welcome them back."
        );
        assert_eq!(
            inner.poster_path,
            Some("/jYfMTSiFFK7ffbY2lay4zyvTkEk.jpg".into())
        );
        assert!(!inner.adult);
        assert_eq!(inner.original_language, "en");
        assert_eq!(movie.genre_ids, [27, 53]);
        assert_eq!(inner.popularity, 119.2537);
        assert_eq!(
            inner.release_date,
            Some(NaiveDate::parse_from_str("2025-04-16", "%Y-%m-%d").unwrap())
        );
        assert!(!inner.video);
        assert_eq!(inner.vote_average, 7.6);
        assert_eq!(inner.vote_count, 714);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn it_works_person() {
        let mut server = mockito::Server::new_async().await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", "/find/nm0000354")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("external_source".into(), "imdb_id".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/find-by-id-person.json"))
            .create_async()
            .await;

        let result = client
            .find_by_id("nm0000354", &ExternalIdSource::Imdb.into())
            .await
            .unwrap();

        assert!(result.movie_results.is_empty());
        assert_eq!(result.person_results.len(), 1);
        assert!(result.tv_results.is_empty());
        assert!(result.tv_episode_results.is_empty());
        assert!(result.tv_season_results.is_empty());

        let person = &result.person_results[0];
        assert_eq!(person.id, 1892);
        assert_eq!(person.name, "Matt Damon");
        assert_eq!(person.gender, Some(2));

        m.assert_async().await;
    }

    #[tokio::test]
    async fn it_works_tv_show() {
        let mut server = mockito::Server::new_async().await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", "/find/72023")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("external_source".into(), "tvdb_id".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/find-by-id-show.json"))
            .create_async()
            .await;

        let result = client
            .find_by_id("72023", &ExternalIdSource::Tvdb.into())
            .await
            .unwrap();

        // Searching for a show, but ID 72023 just so happens to also be an episode ID
        assert!(result.movie_results.is_empty());
        assert!(result.person_results.is_empty());
        assert_eq!(result.tv_results.len(), 1);
        assert_eq!(result.tv_episode_results.len(), 1);
        assert!(result.tv_season_results.is_empty());

        let show = &result.tv_results[0];
        let inner = &show.inner;

        assert_eq!(
            inner.backdrop_path,
            Some("/dGzPJnh8YcUS4J10sNunohaXMVn.jpg".into())
        );
        assert_eq!(inner.id, 1406);
        assert_eq!(inner.name, "Deadwood");
        assert_eq!(inner.original_name, "Deadwood");
        assert_eq!(inner.overview, Some("The story of the early days of Deadwood, South Dakota; woven around actual historic events with most of the main characters based on real people. Deadwood starts as a gold mining camp and gradually turns from a lawless wild-west community into an organized wild-west civilized town. The story focuses on the real-life characters Seth Bullock and Al Swearengen.".into()));
        assert_eq!(
            inner.poster_path,
            Some("/4Yp35DVbVOAWkfQUIQ7pbh3u0aN.jpg".into())
        );
        assert!(!inner.adult);
        assert_eq!(inner.original_language, "en");
        assert_eq!(show.genre_ids, [37, 80, 18]);
        assert_eq!(inner.popularity, 19.7755);
        assert_eq!(
            inner.first_air_date,
            Some(NaiveDate::parse_from_str("2004-03-21", "%Y-%m-%d").unwrap())
        );
        assert_eq!(inner.vote_average, 8.122);
        assert_eq!(inner.vote_count, 840);
        assert_eq!(inner.origin_country, ["US"]);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn it_works_tv_episode() {
        let mut server = mockito::Server::new_async().await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", "/find/72023")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("external_source".into(), "tvdb_id".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/find-by-id-show.json"))
            .create_async()
            .await;

        let result = client
            .find_by_id("72023", &ExternalIdSource::Tvdb.into())
            .await
            .unwrap();

        // Searching for an episode, but ID 72023 just so happens to also be a show ID
        assert!(result.movie_results.is_empty());
        assert!(result.person_results.is_empty());
        assert_eq!(result.tv_results.len(), 1);
        assert_eq!(result.tv_episode_results.len(), 1);
        assert!(result.tv_season_results.is_empty());

        let episode = &result.tv_episode_results[0];

        assert_eq!(episode.id, 1005780);
        assert_eq!(episode.name, "Indiscretion");
        assert_eq!(episode.overview, Some("Stardate: Unknown. Kira and Gul Dukat go after the lost prison ship Ravinok. Dukat has a secret. Meanwhile Kasidy Yates tries to find work near Bajor, leaving Sisko uncomfortable.".into()));
        assert_eq!(episode.vote_average, 7.375);
        assert_eq!(episode.vote_count, 24);
        assert_eq!(
            episode.air_date,
            Some(NaiveDate::parse_from_str("1995-10-23", "%Y-%m-%d").unwrap())
        );
        assert_eq!(episode.episode_number, 5);
        assert_eq!(episode.production_code, "40510-477");
        assert_eq!(episode.season_number, 4);
        assert_eq!(
            episode.still_path,
            Some("/lwckcpWV44isBX4dxYbPivv0crq.jpg".into())
        );

        m.assert_async().await;
    }

    #[tokio::test]
    async fn it_works_tv_season() {
        let mut server = mockito::Server::new_async().await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", "/find/1940416")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("external_source".into(), "tvdb_id".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/find-by-id-season.json"))
            .create_async()
            .await;

        let result = client
            .find_by_id("1940416", &ExternalIdSource::Tvdb.into())
            .await
            .unwrap();

        // Searching for an episode, but ID 72023 just so happens to also be a show ID
        assert!(result.movie_results.is_empty());
        assert!(result.person_results.is_empty());
        assert!(result.tv_results.is_empty());
        assert!(result.tv_episode_results.is_empty());
        assert_eq!(result.tv_season_results.len(), 1);

        let season = &result.tv_season_results[0];
        let inner = &season.inner;

        assert_eq!(inner.id, 112257);
        assert_eq!(inner.name, "Season 1");
        assert_eq!(inner.overview, None);
        assert_eq!(
            inner.poster_path,
            Some("/59SVNwLfoMnZPPB6ukW6dlPxAdI.jpg".into())
        );
        assert_eq!(
            inner.air_date,
            Some(NaiveDate::parse_from_str("2022-09-21", "%Y-%m-%d").unwrap())
        );
        assert_eq!(inner.season_number, 1);
        assert_eq!(season.episode_count, 12);

        m.assert_async().await;
    }
}
