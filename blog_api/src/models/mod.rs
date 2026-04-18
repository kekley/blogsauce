use std::fmt::Write;

use pulldown_cmark_escape::StrWrite;

pub mod comment;
pub mod ip;
pub mod joins;
pub mod post;
pub mod shout;
pub mod star;
pub mod user;

#[derive(Debug, Default)]
pub struct StackString {
    data: heapless::String<255, u8>,
}

impl StrWrite for StackString {
    type Error = std::fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.data.write_str(s)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments) -> Result<(), Self::Error> {
        self.data.write_fmt(args)
    }
}

impl StackString {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
    pub fn as_str(&self) -> &str {
        &self.data
    }
}

#[derive(Debug, Default)]
pub struct TokenString {
    pub data: heapless::String<32, u8>,
}

impl TokenString {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
    pub fn as_str(&self) -> &str {
        &self.data
    }
}
