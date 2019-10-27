mod python;
mod rust;

pub use python::eval_python;
pub use rust::eval_rust;

use log::error;
use serenity::prelude::ShareMap;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalType {
    Rust,
    Python,
}

// TODO: Just use a Github API token and use gists
/// Get a hastebin containing the output
#[inline]
fn get_hastebin<'a>(data: &ShareMap, output: String) -> Result<String, &'static str> {
    use crate::inserts::HttpClientKey;

    /// The response given by hastebin
    #[derive(Debug, serde::Deserialize)]
    struct HastebinResponse {
        /// The code to access your hastebin. Concat with "https://hastebin.com/" for a valid url
        key: String,
    }

    let response = {
        // Get the Http Client
        let client = if let Some(client) = data.get::<HttpClientKey>() {
            client
        } else {
            error!("Failed to get Http Client");
            return Err("Failed to run code, try again later");
        };

        // Lock the mutex
        let client = match client.lock() {
            Ok(client) => client,
            Err(err) => {
                error!("Mutex error: {:?}", err);
                return Err("I had trouble doing that, try again later");
            }
        };

        // Send the post with the output as the request body
        (*client)
            .post("https://hastebin.com/documents")
            .body(output)
            .send()
    };

    // Deserialize the response
    let response = match response {
        Ok(mut res) => match res.json::<HastebinResponse>() {
            Ok(json) => json,
            Err(err) => {
                error!("Response Text Error: {:?}", err);
                return Err("Output too big!");
            }
        },
        Err(err) => {
            error!("Response Error: {:?}", err);
            return Err("Failed to get server response");
        }
    };

    // Form the key into a hastebin url
    Ok(String::from("https://hastebin.com/") + &response.key)
}

pub mod string {
    use serde::Serializer;
    use std::fmt::Display;
    use std::io::Write;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        use serde::ser::Error;

        let mut buf: smallvec::SmallVec<[u8; 64]> = smallvec::SmallVec::new();
        match write!(buf, "{}", value) {
            Ok(_) => {}
            Err(err) => {
                return Err(Error::custom(format!(
                    "Serialize Error (String Module): {:?}",
                    err
                )));
            }
        };

        match std::str::from_utf8(&buf) {
            Ok(s) => serializer.serialize_str(s),
            Err(err) => Err(Error::custom(format!(
                "Serialize Error (String Module): {:?}",
                err
            ))),
        }
    }
}
