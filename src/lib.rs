#![warn(clippy::pedantic)]
#![deny(warnings, missing_debug_implementations)]

use reqwest::RequestBuilder;
use serde::Serialize;
use serde_json::to_vec_pretty;

pub trait PrettyJson<T>: Sized
where
    T: Serialize + ?Sized,
{
    fn pretty_json(self, json: &T) -> Self;
}

impl<T> PrettyJson<T> for RequestBuilder
where
    T: Serialize + ?Sized,
{
    fn pretty_json(self, json: &T) -> Self {
        let builder = self.json(json);
        match to_vec_pretty(json) {
            Ok(body) => builder.body(body),
            Err(_) => builder,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use reqwest::{Client, StatusCode};
    use serde_json::{to_string, to_string_pretty, to_value, Value};

    use super::*;

    #[test]
    fn add_pretty_json() {
        let mut json_data = HashMap::new();
        json_data.insert("foo", vec![1, 2, 3]);

        let body_should_be = to_string_pretty(&json_data).unwrap();
        let body_shouldnt_be = to_string(&json_data).unwrap();
        let value = to_value(&json_data).unwrap();

        let client = Client::new();
        let mut response = client
            .post("http://httpbin.org/post")
            .pretty_json(&json_data)
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let result: Value = response.json().unwrap();

        assert_eq!(result["data"], body_should_be);
        assert_ne!(result["data"], body_shouldnt_be);
        assert_eq!(result["headers"]["Content-Type"], "application/json");
        assert_eq!(result["json"], value);
    }
}
