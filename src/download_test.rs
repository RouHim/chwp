use assertor::{assert_that, EqualityAssertion};
use crate::download;

#[test]
fn get_data() {
    // GIVEN
    let url = "https://gist.githubusercontent.com/thewheat/bb67f632950c7feaf4b8a2f3febbd98a/raw/02feb16f6fac5edf8e6df7e287dbb08b53cc38c1/Test.txt";

    // WHEN
    let data = download::get_data(url);

    // THEN
    assert_that!(data.as_slice()).is_equal_to("test text file".as_bytes());
}
