use assertor::{assert_that, EqualityAssertion, StringAssertion};

use crate::download;

const TEST_TXT_FILE_URL: &str =
    "https://gist.githubusercontent.com/thewheat/bb67f632950c7feaf4b8a2f3febbd98a/raw/02feb16f6fac5edf8e6df7e287dbb08b53cc38c1/Test.txt";

#[test]
fn get_data() {
    // GIVEN a url pointing to a text file

    // WHEN downloading data of the file
    let data = download::get_data(TEST_TXT_FILE_URL);

    // THEN the data should contain the text file content
    assert_that!(data.as_slice()).is_equal_to("test text file".as_bytes());
}

#[test]
fn get_string() {
    // GIVEN a url pointing to a text file

    // WHEN downloading data of the file
    let string_data = download::get_string(TEST_TXT_FILE_URL);

    // THEN the data should contain the text file content
    assert_that!(string_data).is_same_string_to("test text file");
}
