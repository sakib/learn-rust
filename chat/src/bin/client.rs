use async_std::io::BufReader;
use async_std::path::Iter;
use async_std::prelude::*;
use async_std::{task,io,net};
use std::iter;
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

// Input: a mutable TCP Stream to send messages to
// Output: a ChatResult
async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Options: \njoin CHAT\npost CHAT MESSAGE");

    // Create a Buffered Reader to read lines from the command line's STDIN.
    // Call it `options` to represent "optional values" b/c the lines could be empty.
    let mut options = io::BufReader::new(io::stdin()).lines();

    // Loop as long as more lines can be asynchronously pulled from STDIN via the 
    // `options` Stream/Iterator and deconstructed into a Result that is not None. 
    while let Some(option_result) = options.next().await {
        // Parse the input line into a `req` variable if there are no errors parsing.
        let opt = option_result?;
        let req = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };
        // Send our user input as a JSON message to the server.
        utils::send_json(&mut send, &req).await?;
        // Flush the TCP Stream buffer to make sure all the data has been written.
        send.flush().await?;
    }
    Ok(())
}

// Input: an immutable TCP Stream to read messages from
// Output: a ChatResult
async fn messages(server: net::TcpStream) -> ChatResult<()> {
    // Create a Buffered Reader to read lines from the server TCP Stream.
    let buf = io::BufReader::new(server);
    // Deserialize data from the buffer into a Stream of ChatResults.
    let mut stream = utils::receive(buf);

    // Loop as long as more lines can be asynchronously pulled from the server TCP Stream
    // and deconstructed into a Result that is not None.
    while let Some(msg) = stream.next().await {
        match msg? {
            // If the line is serialized into a Server::Message type, format/print it to STDOUT.
            Server::Message { chat_name, message } => {
                println!("Chat Name: {}\nMessage: {}", chat_name, message);
            }
            // If the line is an error message, format/print it to STDERR.
            Server::Error(message) => {
                println!("Error received: {}", message);
            }
        }
    }
    Ok(())
}

// Helper function to extract the first value from input STDIN line separated on whitespace.
fn get_value(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    // If a whitespace char is found, split the input into two parts and return them as a tuple.
    match input.find(char::is_whitespace) {
        Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),
        None => Some((input, "")),
    }
}

// Helper function to turn a raw string from STDIN into a Client->Server request (custom Enum).
fn parse_input(line: &str) -> Option<Client> {
    let (input, remainder) = get_value(line)?;
    if input == "join" {
        let (chat, remainder) = get_value(remainder)?;
        if !remainder.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Join{chat_name: Arc::new(chat.to_string())});
    } else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        let message = remainder.trim_start().to_string();
        return Some(Client::Post{chat_name: Arc::new(chat.to_string()), message: Arc::new(message)});
    }
    println!("Unrecognized input: {:?}", line);
    return None;
}

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address:Port");
    task::block_on(async {
        let socket = net::TcpStream::connect(addr).await?;
        socket.set_nodelay(true)?;
        let send = send(socket.clone());
        let replies = messages(socket);
        replies.await?;
        Ok(())
        // I give up, fuck this guy
    });
    Ok(())
}
