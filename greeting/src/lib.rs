
#![allow(unused)]

use std::f32::consts::E;

use emacs::Error;
fn main() {
use emacs::{defun, Env, Result, Value};
use std::fs;
use serde_json::from_str;
// Emacs won't load the module without this.
emacs::plugin_is_GPL_compatible!();

// Register the initialization hook that Emacs will call when it loads the module.
#[emacs::module]
fn init(env: &Env) -> Result<Value<'_>> {
    env.message("Done loading!")
}

// Define a function callable by Lisp code.
#[defun]
fn say_hello(env: &Env, name: String) -> Result<Value<'_>> {
    env.message(&format!("Hello, {}!", name))
}

// Process Json
#[defun]
fn process_json(env: &Env, filepath: String) -> Result<String> {
    let file_content = fs::read_to_string(filepath)?;
    let json_data: serde_json::Value= from_str(file_content.as_str())?;
    json_data.as_object().ok_or(
        Error{inner: "Expected a JSON object"}
    )?.iter().for_each(|(key, value)| {
        env.message(&format!("{}: {}", key, value));
    });

}
}

