use std::collections::HashMap;
use std::str::FromStr;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};
use warp::{Filter, http::StatusCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    #[derive(Debug, Serialize, Deserialize)]
    struct LogonRequest { username: String, password: String }

    #[derive(Debug)]
    struct QuestionId(String);
    impl FromStr for QuestionId {
        type Err = std::io::Error;
        fn from_str(id: &str) -> Result<Self, Self::Err> {
            match id.is_empty() {
                false => Ok(QuestionId(id.to_string())),
                true => Err(Error::new(ErrorKind::InvalidInput, "No id provided"))
            }
        }
    }
    #[derive(Debug)]
    struct Question { id: QuestionId, title: String, content: String, tags: Option<Vec<String>> }
    impl Question {
        fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
            Question{id, title, content, tags}
        }
    }
    let question = Question::new(
        QuestionId::from_str("id").expect("No id provided"),
        "title".to_string(),
        "content".to_string(),
        Some(vec!("tag".to_string()))
    );
    println!("{:?}", question);

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // GET /test => 200 OK with body "test"
    let test = warp::path!("test")
        .map(|| format!("test"));

    async fn print_request(item: LogonRequest) -> Result<impl warp::Reply, warp::Rejection> {
        println!("{:?}", item);
        Ok(warp::reply::with_status(
            "Question added",
            StatusCode::OK,
        ))
    }

    // POST /token
    let token = warp::post()
        .and(warp::path("token"))
        .and(warp::path::end())
        .and(warp::body::json())
        // .map(|item| format!("{:?}", item));
        .and_then(print_request);

    let routes = hello.or(test).or(token);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3031))
        .await;

    Ok(())
}
