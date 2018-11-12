#![warn(clippy::pedantic)]
#![deny(warnings, missing_debug_implementations)]

//! [`reqwest`] provides an easy way of sending JSON-formatted body in the HTTP request and
//! it always emits terse on-line JSON representation.
//!
//! Most of the time it is exactly what you need. However, in some cases you may prefer
//! to emit "pretty" JSON representation of your data structures. Key-Value data stores are one
//! such use case and there may be others as well.
//!
//! In this case you won't be able to use [`reqwest::RequestBuilder.json`] method and will have to
//! manually serialize your data and set both the body of the request and Content-Type HTTP header.
//!
//! This crate provides convenient method to do just that.
//! It exports trait [`PrettyJson`] that extends [`reqwest::RequestBuilder`] with
//! [`PrettyJson::pretty_json`] method (in addition to the original
//! [`reqwest::RequestBuilder::json`]).
//!
//! This method serializes your data structures as "pretty" JSON
//! (using [`serde_json::to_vec_pretty`]) and lets [`reqwest::RequestBuilder::json`] do the rest.
//!
//!
//! ```rust
//! use reqwest::Client;
//! use reqwest_pretty_json::PrettyJson;
//!
//! let data = vec![1, 2, 3];
//! let client = Client::new();
//! let request = client
//!     .post("http://httpbin.org/post")
//!     .pretty_json(&data)
//!     .build();
//! ```

use reqwest::RequestBuilder;
use serde::Serialize;
use serde_json::to_vec_pretty;

/// A trait to set HTTP request body to a "prettified" JSON-formatted representation of the data.
pub trait PrettyJson<T>: Sized
where
    T: Serialize + ?Sized,
{
    /// Send a "pretty" JSON body.
    ///
    /// Set the body to the "pretty" (human-friendly) JSON serialization of the passed value,
    /// and also set the `Content-Type: application/json` header.
    ///
    /// ```rust
    /// # use reqwest::Error;
    /// # use std::collections::HashMap;
    /// use reqwest_pretty_json::PrettyJson;
    ///
    /// # fn run() -> Result<(), Error> {
    /// let mut map = HashMap::new();
    /// map.insert("lang", "rust");
    ///
    /// let client = reqwest::Client::new();
    /// let res = client.post("http://httpbin.org")
    ///     .pretty_json(&map)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Same as `json()`. See `reqwest` for more details.
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
