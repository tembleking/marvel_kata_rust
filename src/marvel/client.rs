use crate::comics::{Comic, ComicRepository};
use ureq::SerdeValue;

pub struct MarvelClient {}

impl MarvelClient {
    pub fn new() -> Self {
        MarvelClient {}
    }
}

//const MARVEL_COMIC_URL: &str = "http://gateway.marvel.com:80/v1/public/comics?dateDescriptor=nextWeek&ts=987&apikey=97f295907072a970c5df30d73d1f3816&hash=abfa1c1d42a73a5eab042242335d805d";
const MARVEL_COMIC_URL: &str = "https://gist.githubusercontent.com/nikeyes/605b05239fa737661a76/raw/ef1c10ae00f667782da38656cea53d3e08004506/ComicsNextWeek.js";

impl ComicRepository for MarvelClient {
    fn retrieve_comics(&self) -> Option<Vec<Comic>> {
        ureq::get(MARVEL_COMIC_URL).call().ok().map(|response| {
            let mut res: Vec<Comic> = vec![];

            let data: SerdeValue = response.into_json().unwrap();

            let comics_data = data["data"]["results"].as_array().unwrap();
            for comic_data in comics_data {
                res.push(Comic::new(
                    comic_data["title"].to_string(),
                    format!(
                        "{}.{}",
                        comic_data["thumbnail"]["path"].as_str().unwrap(),
                        comic_data["thumbnail"]["extension"].as_str().unwrap()
                    ),
                    comic_data["prices"][0]["price"].as_f64().unwrap_or(0.0) as f32,
                ));
            }

            return res;
        })
    }
}
