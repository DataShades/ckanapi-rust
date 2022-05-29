use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub struct CKAN {
    url: String,
    client: Client,
}

impl CKAN {
    pub fn new(url: &str) -> Self {
        CKAN {
            url: url.trim_matches('/').to_owned(),
            client: Client::new(),
        }
    }
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn invoke<T: for<'de> Deserialize<'de>>(&self, action: &Action) -> Response<T> {
        let url = format!("{}/api/action/{}", self.url, &action.name);
        match self.client.post(url).send() {
            Ok(resp) => match resp.json::<Response<T>>() {
                Ok(result) => result,
                Err(err) => Response::DecodeError(err.to_string()),
            },
            Err(err) => Response::ReqwestError(err.to_string()),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Result(Success<T>),
    Error(Fail),
    StringError(String),
    ReqwestError(String),
    DecodeError(String),
}

#[derive(Deserialize, Debug)]
pub struct Success<T> {
    pub help: String,
    pub result: T,
}

#[derive(Deserialize, Debug)]
pub struct Fail {
    pub help: String,
    pub error: Value,
}

#[derive(Debug)]
pub struct Action {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let ckan = CKAN::new("http://localhost:5000");

        let action = Action {
            name: "status_show".into(),
        };

        let result: Response<Value> = ckan.invoke(&action);

        dbg!(result);
    }
}
