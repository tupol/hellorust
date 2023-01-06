use serde::{Deserialize, Serialize};
use warp::{Filter, http::StatusCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    #[derive(Debug, Serialize, Deserialize)]
    struct LogonRequest { username: String, password: String }

    async fn print_request(item: LogonRequest) -> Result<impl warp::Reply, warp::Rejection> {
        println!("{:?}", item);
        Ok(warp::reply::with_status(
            format!("Received logon request for {:?}", item.username),
            StatusCode::OK,
        ))
    }

    // POST /token
    let token = warp::post()
        .and(warp::path("token"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(print_request);

    let routes = token;

    println!("Hellorust open for e-business");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3031))
        .await;

    Ok(())
}
