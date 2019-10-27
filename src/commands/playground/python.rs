use crate::inserts::HttpClientKey;
use log::error;
use serde::Deserialize;
use serenity::prelude::ShareMap;

pub fn eval_python(data: &ShareMap, code: &str) -> Result<String, &'static str> {
    // TODO: Add output processing
    let response = match python_post(data, code) {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    Ok(if response.output.is_empty() {
        String::from("``` ```")

    // If output is under the Discord size limit, send it inside of a codeblock
    } else if response.output.len() < 1990 {
        format!("```\n{}\n```", response.output.replace("```", " ` ` `"))

    // If output is over the Discord size limit and under 5000 char, send it as a hastebin
    } else if response.output.len() >= 1990 && response.output.len() <= 5000 {
        super::get_hastebin(data, response.output)?

    // Inform the user that the output is too large
    } else {
        String::from("Output too big!")
    })
}

#[inline]
fn python_post(data: &ShareMap, request_data: &str) -> Result<Response, &'static str> {
    let response = {
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

        (*client)
            // TODO: Find a better API, Possibly https://www.jdoodle.com/python3-programming-online (Offers other languages as well)
            .post("https://pythonprinciples.com/validate.php?lesson=Interpreter&slide=0")
            .body(base64::encode(&request_data))
            .send()
    };

    // Lots of shadowing here, just helps break up all the error handling

    // See if request was successful
    let mut response = match response {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending post request (Python): {:?}", err);
            return Err("The server failed to respond");
        }
    };

    // Get the text of the response
    let response = match response.text() {
        Ok(text) => text,
        Err(err) => {
            error!("Deserialize Error: {:?}", err);
            return Err("The server failed to respond");
        }
    };

    // Decode the response
    let response = match base64::decode(&response) {
        Ok(decoded) => decoded,
        Err(err) => {
            error!("Base64 Decode Error: {:?}", err);
            return Err("The server returned an invalid response, please try again later");
        }
    };

    // Convert the response from bytes to a string
    let response = match String::from_utf8(response) {
        Ok(data) => data,
        Err(err) => {
            error!("Decoded Data to String Error: {:?}", err);
            return Err("The server returned an invalid response, please try again later");
        }
    };

    // Deserialize the data
    match serde_json::from_str::<Response>(&response) {
        Ok(data) => Ok(data),
        Err(err) => {
            error!("Deserialization Error: {:?}", err);
            Err("The server returned an invalid response, please try again later")
        }
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    output: String,
}
