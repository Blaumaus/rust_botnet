pub fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let mut resp = reqwest::get(url)
        .expect("Failed to load data");

    resp.text()
}