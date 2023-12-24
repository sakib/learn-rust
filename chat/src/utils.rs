// Miscellaneous useful shorthands (Ok, Err, etc.)
use async_std::prelude::*;

// Library for serializing and deserializing JSON data to and from byte-streams.
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

// A type for representing chat errors.
// `type` defines a type alias - give an existing type (RHS) a new name
// Box is a Rust smart pointer that provides Heap allocation and Ref counting. 
//  ...Generally used to avoid large amounts of copying for large data structures.
// dyn Error is a trait that represents an Error type that can be used in the Result type.
// `Send + Sync` are marker traits that mean the type is OK to send + share-refs-to between threads. 
// `static means that this Error's lifetime must be >= lifetime of the entire program. 
pub type ChatError = Box<dyn Error + Send + Sync + 'static>;

// Represents Chat Results that utilize our custom ChatError.
pub type ChatResult<T> = Result<T, ChatError>;

// The send_json function inputs a mutable reference to a `leaving` async "file"-write handler and a 
// shared reference to a `packet` which must be Serializable, and outputs a (custom) ChatResult.
pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    // The type of `leaving` must allow us to write bytes asynchronously while being safely unpinned.
    O: async_std::io::Write + Unpin,
    // The type of `packet` must be serializable.
    P: Serialize,
{
    // Turn `packet` into a JSON string, return the Result; propagate Errors
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    // Write the JSON string to the `leaving` argument asynchronously; propagate Errors
    leaving.write_all(json.as_bytes()).await?;
    Ok(())
}

// The receive function inputs an `incoming` byte stream and outputs a type that implements the
// Stream trait where each Item in the Stream is a (custom) ChatResult whose result is Deserializable.
pub async fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult<T>>
where
    // The type of `incoming` must allow us to read bytes asynchronously while being safely unpinned.
    I: async_std::io::BufRead + Unpin,
    // The type of ChatResult's result must be able to be deserialized without borrowing 
    // any data structure from the Deserializer. Useful for trait bounds on funcs like from_string.
    T: DeserializeOwned,
{
    // lines() returns a Stream over the lines of this `incoming` byte stream. 
    // map() each line to a ChatResult (whose result must be deserializable)
    // Do this by unpacking the line string slice into a JSON object.
    incoming.lines().map(|line| -> ChatResult<T> {
        let li = line?;
        let msg = serde_json::from_str::<T>(&li)?;
        Ok(msg)
    })
}