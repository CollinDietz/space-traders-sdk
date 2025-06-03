use mockito::ServerGuard;
use serde::Serialize;

pub enum RequestMethod {
    Get,
    Post,
}

impl std::string::ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            RequestMethod::Get => "GET".to_string(),
            RequestMethod::Post => "POST".to_string(),
        }
    }
}

pub async fn mock_response<T: Serialize>(
    method: RequestMethod,
    endpoint: &str,
    status: usize,
    bearer_token: String,
    body: Option<&T>,
) -> ServerGuard {
    let mut s = mockito::Server::new_async().await;
    let mut mock = s
        .mock(
            method.to_string().as_str(),
            format!("/{}", endpoint).as_str(),
        )
        .match_header("Authorization", format!("Bearer {}", bearer_token).as_str())
        .with_header("Content-Type", "application/json")
        .with_status(status)
        .with_body_from_file(
            format!(
                "mock_server/responses/{}_{}.json",
                endpoint.replace("/", "_"),
                status
            )
            .as_str(),
        );

    if let Some(body) = body {
        let json = serde_json::to_value(body).expect("Failed to serialize expected body");
        mock = mock.match_body(mockito::Matcher::Json(json));
    };

    mock.create_async().await;

    s
}
