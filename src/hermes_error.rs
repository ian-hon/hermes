use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum HermesError {
    Success,

    // expected "username" to be in query args
    InvalidArguments,

    // expected "name" not to have any special characters
    InvalidFormat, 
}

pub enum HermesFormat {
    Unspecified, // anything goes

    Number,     // i32; only numbers 0-9
    BigNumber,  // i64; only numbers 0-9
    // Hex,        // i64; only alphanumerics
    Key,      // all lowercase, no spaces or special characters
}

pub fn check(c: &HashMap<String, String>, t: Vec<(&str, HermesFormat)>) -> HermesError {
    let t = t.into_iter().map(|x| (x.0.to_string(), x.1)).collect::<Vec<(String, HermesFormat)>>();
    for i in t {
        match c.get(&i.0) {
            Some(v) => {
                match i.1 {
                    HermesFormat::Number => {
                        if v.parse::<i32>().is_err() {
                            return HermesError::InvalidFormat;
                        }
                    },
                    HermesFormat::BigNumber => {
                        if v.parse::<i64>().is_err() {
                            return HermesError::InvalidFormat;
                        }
                    },
                    HermesFormat::Key => {
                        // "a-z, 0-9, _"
                        if !v.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || (b == b'_')) {
                            return HermesError::InvalidFormat;
                        }
                    }
                    _ => {}
                }
            },
            None => {
                return HermesError::InvalidArguments;
            }
        }
    }

    HermesError::Success
}
