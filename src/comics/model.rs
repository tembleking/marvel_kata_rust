pub struct Comic {
    title: String,
    thumbnail: String,
    price: f32,
}

impl Comic {
    pub fn new(title: String, thumbnail: String, price: f32) -> Self {
        Comic {
            title,
            thumbnail,
            price,
        }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn thumbnail(&self) -> &str {
        &self.thumbnail
    }
    pub fn price(&self) -> f32 {
        self.price
    }
}
