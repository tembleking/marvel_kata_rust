use marvel_kata_rust::comics::ComicRepository;
use marvel_kata_rust::marvel::MarvelClient;
use rspec::{describe, run};
use spectral::assert_that;

#[test]
fn marvel() {
    run(&describe("Marvel Suite", false, |ctx| {
        ctx.it("is able to retrieve the comics from the API", |_ctx| {
            let client = MarvelClient::new();
            let comics = client.retrieve_comics().unwrap();

            assert_that(&comics.len()).is_equal_to(20);
        })
    }))
}
