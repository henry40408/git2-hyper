use git2::FetchOptions;

#[cfg(feature = "native")]
use hyper_tls::HttpsConnector;

#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;

use tempfile::TempDir;

const REMOTE_URL: &'static str = "https://github.com/henry40408/git2-hyper";

fn main() {
    #[cfg(feature = "native")]
    unsafe {
        git2_hyper::register(
            hyper::Client::builder()
                .http1_title_case_headers(true)
                .build(HttpsConnector::new()),
        );
    }

    #[cfg(feature = "rustls")]
    unsafe {
        git2_hyper::register(
            hyper::Client::builder()
                .http1_title_case_headers(true)
                .build(HttpsConnector::with_webpki_roots()),
        );
    }

    let td = TempDir::new().unwrap();
    let repo = git2::Repository::clone(REMOTE_URL, td.as_ref()).unwrap();
    let mut remote = repo.remote_anonymous("origin").unwrap();

    let mut fo = FetchOptions::new();
    remote.download(&[] as &[&str], Some(&mut fo)).unwrap();
}
