use super::Comic;

pub trait ComicRepository {
    fn retrieve_comics(&self) -> Option<Vec<Comic>>;
}
