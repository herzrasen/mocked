use serde::{Deserialize, Serialize};

use crate::matcher::Matcher;
use crate::rule::Request;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "op", content = "matchers", rename_all = "camelCase")]
pub enum Matchers {
    And(Vec<Matcher>),
    Or(Vec<Matcher>),
}

impl Matchers {
    pub fn empty() -> Matchers {
        Matchers::Or(Vec::new())
    }

    pub fn matches(&self, req: &Request) -> bool {
        match self {
            Matchers::And(matchers) => matchers.iter().all(|matcher| matcher.matches(req)),
            Matchers::Or(matchers) => matchers.iter().any(|matcher| matcher.matches(req)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::{Matcher, PathParamMatch};
    use crate::matchers::Matchers;
    use crate::r#match::Match;

    #[test]
    fn test_serialize() {
        let c = Matchers::And(vec![Matcher::PathParamMatcher(PathParamMatch {
            name: "foo".to_string(),
            matches: vec![
                Match::String("bar".to_string()),
                Match::String("baz".to_string()),
            ],
        })]);
        let yaml = serde_yaml::to_string(&c).unwrap();
        println!("{yaml}")
    }
}
