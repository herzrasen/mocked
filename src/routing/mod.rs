use crate::request::Request;

pub mod body;
pub mod condition;
pub mod config;
pub mod matcher;
pub mod matchers;
pub mod method;
pub mod response;
pub mod route;
pub mod value;
pub mod options;

trait Matching {
    fn matches(&self, req: &Request) -> bool;
}
