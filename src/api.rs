pub struct Api
{
   client: reqwest::Client,
}


impl Api {
   pub fn new() -> Self {
      Api { client: reqwest::Client::new() }
  }
}


mod login;
