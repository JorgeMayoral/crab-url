use crate::url_repository::UrlRepository;

pub struct AppState {
    url_repository: UrlRepository,
}

impl AppState {
    pub fn new() -> Self {
        let url_repository = UrlRepository::new();
        Self { url_repository }
    }

    pub fn get_url_repo(&self) -> &UrlRepository {
        &self.url_repository
    }
}
