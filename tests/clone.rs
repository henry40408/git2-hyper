use hyper_tls::HttpsConnector;
use tempfile::TempDir;

const REMOTE_URL: &'static str = "https://github.com/henry40408/git2-hyper";

fn main() {
    unsafe {
        git2_hyper::register(
            hyper::Client::builder()
                .http1_title_case_headers(true)
                .build(HttpsConnector::new()),
        );
    }

    let td = TempDir::new().unwrap();
    git2::Repository::clone(REMOTE_URL, td.as_ref()).unwrap();
}
