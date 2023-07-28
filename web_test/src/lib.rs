//封装http请求

use reqwest::blocking::Client;

pub struct Request {
    pub url: String,
    pub body: String,
}

impl RequestData for Request {
    fn to_string(&self) -> String {
        self.body.clone()
    }
    fn url(&self) -> String {
        self.url.clone()
    }
}

pub trait RequestData {
    fn to_string(&self) -> String;
    fn url(&self) -> String;
    fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        get(&self.url())
    }
    fn post(&self) -> Result<String, Box<dyn std::error::Error>> {
        post(&self.url(), self)
    }
}

pub fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.get(url).send()?;
    let text = resp.text()?;
    Ok(text)
}

pub fn post(url: &str, body: &RequestData) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.post(url).json(body.to_string()).send()?;
    let text = resp.text()?;
    Ok(text)
}
