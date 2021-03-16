use crate::comics::ComicService;
use crate::marvel::MarvelClient;

mod comics;
mod marvel;

fn main() {
    let marvel_client = Box::new(MarvelClient::new());
    let service = ComicService::new(marvel_client);

    for comic in service.get_next_week_comics() {
        println!(
            "{} | {} ({}) ",
            comic.price(),
            comic.title(),
            comic.thumbnail()
        );
    }
}
