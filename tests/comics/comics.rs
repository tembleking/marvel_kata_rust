use mockall::mock;
use rspec::{describe, run};
use spectral::assert_that;

use marvel_kata_rust::comics::{Comic, ComicRepository, ComicService};

// We mock the trait here, as we don't want to generate code in the main service that won't be use in production.
mock! {
    pub Repository {
    }
    impl ComicRepository for Repository {
        fn retrieve_comics(&self) -> Option<Vec<Comic>>;
    }
}

#[test]
fn service() {
    run(&describe("Marvel Kata Service", false, |ctx| {
        ctx.it("retrieves 5 comics", |_ctx| {
            let mut repository = Box::new(MockRepository::new());
            repository
                .expect_retrieve_comics()
                .returning(|| Some(comics()));

            let comic_service = ComicService::new(repository);

            assert_that(&comic_service.get_next_week_comics().len()).is_equal_to(5);
        })
    }))
}

fn comics() -> Vec<Comic> {
    vec![
        Comic::new("Comic 1".to_owned(), "Thumbnail 1".to_owned(), 1.0),
        Comic::new("Comic 2".to_owned(), "Thumbnail 2".to_owned(), 2.0),
        Comic::new("Comic 3".to_owned(), "Thumbnail 3".to_owned(), 3.0),
        Comic::new("Comic 4".to_owned(), "Thumbnail 4".to_owned(), 4.0),
        Comic::new("Comic 5".to_owned(), "Thumbnail 5".to_owned(), 5.0),
    ]
}
