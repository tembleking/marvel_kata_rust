use super::{Comic, ComicRepository};

pub struct ComicService {
    comic_repository: Box<dyn ComicRepository>,
}

impl ComicService {
    pub fn new(comic_repository: Box<dyn ComicRepository>) -> Self {
        ComicService { comic_repository }
    }

    pub fn get_next_week_comics(&self) -> Vec<Comic> {
        self.comic_repository.retrieve_comics().unwrap_or(vec![])
    }
}
