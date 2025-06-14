use mockito::{Matcher, Server, ServerGuard};
use serde::Serialize;

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            RequestMethod::Get => "GET".to_string(),
            RequestMethod::Post => "POST".to_string(),
        }
    }
}

fn build_file_path_for_response<T: Serialize>(
    method: &RequestMethod,
    endpoint: &str,
    status: usize,
    body_or_query: &Option<&T>,
) -> String {
    match method {
        RequestMethod::Get => {
            if let Some(query) = body_or_query {
                format!(
                    "mock_server/responses/{}/{}/{}.json",
                    endpoint,
                    serde_urlencoded::to_string(query)
                        .expect("Failed to serialize query parameters"),
                    status
                )
            } else {
                format!("mock_server/responses/{}/{}.json", endpoint, status)
            }
        }
        RequestMethod::Post => {
            format!("mock_server/responses/{}/{}.json", endpoint, status)
        }
    }
}

pub struct MockServerBuilder {
    server: ServerGuard,
}

impl MockServerBuilder {
    pub async fn new() -> Self {
        let server = Server::new_async().await;
        Self { server }
    }

    pub async fn mock_response<T: Serialize>(
        mut self,
        method: RequestMethod,
        endpoint: &str,
        status: usize,
        bearer_token: Option<String>,
        body_or_query: Option<&T>,
    ) -> Self {
        let body_response = build_file_path_for_response(&method, endpoint, status, &body_or_query);

        let mut mock = self
            .server
            .mock(
                method.to_string().as_str(),
                format!("/{}", endpoint).as_str(),
            )
            .with_header("Content-Type", "application/json")
            .with_status(status)
            .with_body_from_file(&body_response);

        if let Some(bearer_token) = bearer_token {
            mock = mock.match_header("Authorization", format!("Bearer {}", bearer_token).as_str());
        }

        match method {
            RequestMethod::Get => {
                if let Some(query) = body_or_query {
                    let query_string = serde_urlencoded::to_string(query)
                        .expect("Failed to serialize query parameters");
                    mock = mock.match_query(query_string.as_str());
                }
            }
            RequestMethod::Post => {
                if let Some(body) = body_or_query {
                    let json = serde_json::to_value(body).expect("Failed to serialize body");
                    mock = mock.match_body(Matcher::Json(json));
                }
            }
        }

        mock.create_async().await;
        self
    }

    pub fn build(self) -> ServerGuard {
        self.server
    }

    pub async fn mock_once<T: Serialize>(
        method: RequestMethod,
        endpoint: &str,
        status: usize,
        bearer_token: Option<String>,
        body_or_query: Option<&T>,
    ) -> ServerGuard {
        Self::new()
            .await
            .mock_response(method, endpoint, status, bearer_token, body_or_query)
            .await
            .build()
    }
}
