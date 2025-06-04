use reqwest::{Error, Response};
use serde::Serialize;

// pub(crate) async fn get(
//     client: &reqwest::Client,
//     url: &str,
//     endpoint: &str,
//     token: &str,
// ) -> Result<Response, Error> {
//     client
//         .get(&format!("{}/{}", url, endpoint))
//         .bearer_auth(token)
//         .send()
//         .await
// }

pub(crate) async fn post<T: Serialize + ?Sized>(
    client: &reqwest::Client,
    url: &str,
    endpoint: &str,
    token: &str,
    body: Option<&T>,
) -> Result<Response, Error> {
    let mut request = client
        .post(&format!("{}/{}", url, endpoint))
        .bearer_auth(token)
        .header("Accept", "application/json");

    if let Some(body) = body {
        request = request.json(body);
    };

    request.send().await
}
