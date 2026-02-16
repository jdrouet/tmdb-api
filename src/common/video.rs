#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Video {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub site: String,
    pub key: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub size: u64,
    pub iso_639_1: String,
    pub iso_3166_1: String,
    pub official: bool,
}

impl Video {
    pub fn url(&self) -> Option<String> {
        match self.site.as_str() {
            "YouTube" => Some(format!("https://www.youtube.com/watch?v={}", self.key)),
            "Vimeo" => Some(format!("https://vimeo.com/{}", self.key)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Video;

    fn make_video(site: &str, key: &str) -> Video {
        Video {
            id: "abc123".into(),
            name: "Test Video".into(),
            kind: "Trailer".into(),
            site: site.into(),
            key: key.into(),
            published_at: chrono::DateTime::default(),
            size: 1080,
            iso_639_1: "en".into(),
            iso_3166_1: "US".into(),
            official: true,
        }
    }

    #[test]
    fn url_youtube() {
        let video = make_video("YouTube", "dfeUzm6KF4g");
        assert_eq!(
            video.url(),
            Some("https://www.youtube.com/watch?v=dfeUzm6KF4g".into())
        );
    }

    #[test]
    fn url_vimeo() {
        let video = make_video("Vimeo", "123456");
        assert_eq!(video.url(), Some("https://vimeo.com/123456".into()));
    }

    #[test]
    fn url_unknown_site() {
        let video = make_video("Dailymotion", "xyz");
        assert_eq!(video.url(), None);
    }
}
