use std::io::Read;

/// Download a byte vector from an URL
pub fn get_data(request_url: &str) -> Vec<u8> {
    let response = ureq::get(request_url).call().unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    response.into_reader().read_to_end(&mut bytes).unwrap();
    bytes
}

/// Downloads a string from an URL
/// # Arguments
/// * `url` - The url to download
/// # Returns
/// The downloaded string
pub fn get_string(request_url: &str) -> String {
    let response = ureq::get(request_url).call().unwrap();
    response.into_string().unwrap()
}
