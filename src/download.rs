use std::io::Read;
use std::sync::Arc;

use rustls::ClientConfig;
use ureq::Agent;

/// Download a byte vector from an URL
pub fn get_data(request_url: &str) -> Vec<u8> {
    let http_client = get_http_client();
    let response = http_client.get(request_url).call().unwrap();
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
    let http_client = get_http_client();
    let response = http_client.get(request_url).call().unwrap();
    response.into_string().unwrap()
}

/// Get a new HTTP client with the platform's root certificates
/// # Returns
/// A new HTTP client
fn get_http_client() -> Agent {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        roots.add(cert).unwrap();
    }

    let tls_config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    ureq::builder().tls_config(Arc::new(tls_config)).build()
}
