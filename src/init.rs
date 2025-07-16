use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::vec;

use axum::http::StatusCode;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::routing::body::{Body, Include};
use crate::routing::condition::Condition;
use crate::routing::matcher::{
    BodyContainsMatcher, HeaderContainsMatcher, Matcher, PathParamMatcher,
};
use crate::routing::matchers::Matchers;
use crate::routing::method::Method;
use crate::routing::response::Response;
use crate::routing::route::Route;
use crate::routing::value::Value;
use crate::routing::{config::Config, options::Options};

pub async fn init(
    port: u16,
    address: String,
    enable_cors: bool,
    min_response_delay_ms: u64,
    max_response_delay_ms: u64,
    without_example_routes: bool,
    path: String,
) {
    let routes = if without_example_routes {
        vec![]
    } else {
        let parent_dir = get_directory(path.clone());
        create_example_routes(parent_dir)
    };
    let config = Config {
        options: Options {
            address,
            port,
            enable_cors: Some(enable_cors),
            min_response_delay_ms: Some(min_response_delay_ms),
            max_response_delay_ms: Some(max_response_delay_ms),
        },
        routes,
    };
    let config_str = serde_yaml::to_string(&config).unwrap();
    match File::create(&path).await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(config_str.as_bytes()).await {
                eprintln!("Failed to write to file {}: {}", path, e);
            } else {
                println!("Configuration written to {}", path);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file {}: {}", path, e);
        }
    }
    create_hello_json().await
}

fn create_example_routes(_config_dir: PathBuf) -> Vec<Route> {
    let mut headers = HashMap::new();
    headers.insert(
        String::from("Content-Type"),
        String::from("application/json"),
    );
    vec![
        Route {
            path: String::from("/hello"),
            methods: vec![Method::Get],
            enable_cors: Some(false),
            conditions: vec![Condition {
                matcher: None,
                matchers: None,
                response: Response {
                    status: StatusCode::OK.into(),
                    headers: headers.clone(),
                    body: Some(Body::Include(Include {
                        include: "hello.json".into(),
                    })),
                },
            }],
        },
        Route {
            path: String::from("/hello/{name}"),
            methods: vec![Method::Post],
            enable_cors: None,
            conditions: vec![Condition {
                matcher: Some(Matcher::BodyContains(BodyContainsMatcher {
                    values: vec![String::from("hello")],
                })),
                matchers: None,
                response: Response {
                    status: StatusCode::OK.into(),
                    headers: HashMap::new(),
                    body: Some(Body::String(String::from("Hello world"))),
                },
            }],
        },
        Route {
            path: String::from("/hello/{name}/{age}"),
            methods: vec![Method::Get],
            conditions: vec![Condition {
                matchers: Some(Matchers::Or(vec![
                    Matcher::PathParam(PathParamMatcher {
                        name: String::from("name"),
                        values: vec![Value::String(String::from("mocked"))],
                    }),
                    Matcher::PathParam(PathParamMatcher {
                        name: String::from("age"),
                        values: vec![Value::Integer(42)],
                    }),
                ])),
                matcher: None,
                response: Response {
                    status: StatusCode::OK.into(),
                    headers,
                    body: None,
                },
            }],
            enable_cors: None,
        },
        Route {
            path: String::from("/upload"),
            methods: vec![Method::Post],
            enable_cors: None,
            conditions: vec![Condition {
                matcher: Some(Matcher::HeaderContains(HeaderContainsMatcher {
                    name: String::from("Authorization"),
                    values: vec![String::from("Basic"), String::from("Bearer")],
                })),
                matchers: None,
                response: Response {
                    status: StatusCode::ACCEPTED.into(),
                    headers: HashMap::new(),
                    body: Some(Body::String(String::from("Accepted"))),
                },
            }],
        },
    ]
}

pub async fn create_hello_json() {
    let content = r#"{
    "message": "Hello, world!"
}"#;

    match File::create("hello.json").await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()).await {
                eprintln!("Failed to write to hello.json: {}", e);
            } else {
                println!("hello.json created successfully.");
            }
        }
        Err(e) => {
            eprintln!("Failed to create hello.json: {}", e);
        }
    }
}

fn get_directory(path: String) -> PathBuf {
    Path::new(&path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| ".".to_string().into())
}
