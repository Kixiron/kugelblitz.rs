use super::string;
use crate::inserts::HttpClientKey;
use log::error;
use serde::{Deserialize, Serialize};
use serenity::prelude::ShareMap;
use std::fmt;

/// Evaluate Rust code
pub fn eval_rust(data: &ShareMap, code: &str) -> Result<String, &'static str> {
    use regex::Regex;

    lazy_static::lazy_static! {
        /// A regex to detect if a main function is in the code
        static ref FUNCTION_REGEX: Regex = Regex::new(r#"fn\s*main\s*\(\s*\)"#).expect("Main function regex failed to compile");
    }

    // If the code has a main function, don't add one
    let code = if FUNCTION_REGEX.is_match(code) {
        code.to_owned()

    // If there is no main function, wrap the user's code inside of one while also printing the output of their code
    } else {
        #[cfg_attr(rustfmt, rustfmt::skip)]
        format!(
            "fn main() -> Result<(), Box<dyn std::any::Any>> {{\n    \
                println!(\"{{:?}}\", {{\n        \
                    {code}\n    \
                }});\n    \
                Ok(())\n\
            }}",
            code = code,
        )
    };

    // Form the rust request
    // TODO: Add options from the user
    let request = Request {
        channel: Channel::Stable,
        mode: Mode::Debug,
        edition: Edition::_2018,
        crate_type: CrateType::Bin,
        tests: false,
        code: &code,
        backtrace: false,
    };

    // Make the request and parse the output
    parse_rust(data, &request, rust_post(data, &request)?)
}

/// Make a post request to play.rust-lang.org to evaluate Rust code
#[inline]
fn rust_post<'a>(data: &ShareMap, request: &Request<'a>) -> Result<Response, &'static str> {
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
                error!("Mutex Error: {:?}", err);
                return Err("I had trouble doing that, try again later");
            }
        };

        // Post the request with the necessary data as a json payload
        (*client)
            .post("https://play.rust-lang.org/execute")
            .json(&request)
            .send()
    };

    match response {
        Ok(mut res) => match res.json() {
            Ok(json) => Ok(json),
            Err(err) => {
                error!("Deserialize Error: {:?}", err);
                Err("The server failed to respond")
            }
        },
        Err(err) => {
            error!("Error sending post request (Rust): {:?}", err);
            Err("The server failed to respond")
        }
    }
}

/// Parse a Rust response into a human-readable Discord message
#[inline]
fn parse_rust<'a>(
    data: &ShareMap,
    request: &Request<'a>,
    rust: Response,
) -> Result<String, &'static str> {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref ERROR_REGEX: Regex = Regex::new(r#"(error(\[.*\])*:)|(error:)"#).expect("Error regex failed to compile");
    }

    let output = if !ERROR_REGEX.is_match(&rust.stderr)
        && rust.success
        && (rust.stderr.len() >= 4 && !rust.stderr.contains("panicked"))
    {
        rust.stdout
    } else {
        rust.stderr.split('\n').collect::<Vec<&str>>()[1..].join("\n") + "\n\n" + &rust.stdout
    };

    // If no output, send an empty codeblock
    Ok(if output.is_empty() {
        String::from("``` ```")

    // If output is under the Discord size limit, send it inside of a codeblock
    } else if output.len() < 1990 {
        format!("```\n{}\n```", output.replace("```", " ` ` `"))

    // If output is over the Discord size limit and under 5000 char, send it as a playground link
    } else if output.len() >= 1990 && output.len() <= 5000 {
        get_gist(data, request)?

    // Inform the user that the output is too large
    } else {
        String::from("Output too big!")
    })
}

fn get_gist<'a>(data: &ShareMap, request: &Request<'a>) -> Result<String, &'static str> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Gist<'b> {
        code: &'b str,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        id: String,
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
                error!("Mutex Error: {:?}", err);
                return Err("I had trouble doing that, try again later");
            }
        };

        // Post the request with the necessary data as a json payload
        (*client)
            .post("https://play.rust-lang.org/meta/gist/")
            .json(&Gist { code: request.code })
            .send()
    };

    let response = match response {
        Ok(mut res) => match res.json::<Response>() {
            Ok(json) => json,
            Err(err) => {
                error!("Deserialize Error: {:?}", err);
                return Err("The server failed to respond");
            }
        },
        Err(err) => {
            error!("Error sending post request (Rust): {:?}", err);
            return Err("The server failed to respond");
        }
    };

    Ok(format!(
        "https://play.rust-lang.org/?version={}&mode={}&edition={}&gist={}",
        request.channel, request.mode, request.edition, response.id
    ))
}

/// A valid request to the Rust playground
#[derive(Debug, Serialize)]
struct Request<'a> {
    /// Enable a backtrace
    backtrace: bool,

    /// Set the channel to build on
    #[serde(with = "string")]
    channel: Channel,

    /// The code to be evaluated
    code: &'a str,

    /// The crate type to build as
    #[serde(rename = "crateType", with = "string")]
    crate_type: CrateType,

    /// The edition of Rust to build on
    #[serde(with = "string")]
    edition: Edition,

    /// The mode to build in
    #[serde(with = "string")]
    mode: Mode,

    /// Enable or disable tests
    tests: bool,
}

/// The channel to build Rust on
#[allow(dead_code)]
#[derive(Debug, Serialize)]
enum Channel {
    Stable,
    Beta,
    Nightly,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Channel::*;
        write!(
            f,
            "{}",
            match self {
                Stable => "stable",
                Beta => "beta",
                Nightly => "nightly",
            }
        )
    }
}

/// The build mode
#[allow(dead_code)]
#[derive(Debug, Serialize)]
enum Mode {
    Debug,
    Release,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Mode::*;
        write!(
            f,
            "{}",
            match self {
                Debug => "debug",
                Release => "release",
            }
        )
    }
}

/// The edition of Rust to build on
#[derive(Debug, Serialize)]
#[allow(dead_code)]
enum Edition {
    _2015,
    _2018,
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Edition::*;
        write!(
            f,
            "{}",
            match self {
                _2015 => "2015",
                _2018 => "2018",
            }
        )
    }
}

/// The crate type to build as
#[derive(Debug, Serialize)]
#[allow(dead_code)]
enum CrateType {
    Bin,
    Lib,
}

impl fmt::Display for CrateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CrateType::*;
        write!(
            f,
            "{}",
            match self {
                Bin => "bin",
                Lib => "lib",
            }
        )
    }
}

/// The response recived from the playground
#[derive(Debug, Deserialize)]
struct Response {
    success: bool,
    stdout: String,
    stderr: String,
}
