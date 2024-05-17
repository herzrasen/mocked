use crate::request::Request;

mod body;
mod condition;
pub mod config;
mod matcher;
mod matchers;
mod method;
mod response;
mod route;
mod value;
pub mod options;

trait Matching {
    fn matches(&self, req: &Request) -> bool;
}
