use mockito::ServerGuard;

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

pub fn mock_response(method: RequestMethod, endpoint: &str, status: usize) -> ServerGuard {
    let mut s = mockito::Server::new();
    s.mock(
        method.to_string().as_str(),
        format!("/{}", endpoint).as_str(),
    )
    .with_header("Content-Type", "application/json")
    .with_status(status)
    .with_body_from_file(format!("mock_server/responses/{}_{}.json", endpoint, status).as_str())
    .create();

    s
}
