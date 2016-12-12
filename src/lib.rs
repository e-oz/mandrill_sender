extern crate rustc_serialize;
extern crate reqwest;

pub mod email;

pub use email::{Recipient, Attachment, Email, Sender, TemplateVar};
