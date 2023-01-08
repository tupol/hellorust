use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Serialize, Deserialize)]
    struct LogonRequest {
        username: String,
        password: String,
    }

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

    let mut sched = JobScheduler::new()
        .await
        .expect("Scheduler starting problem");

    let job = Job::new_repeated(Duration::from_secs(7), |_uuid, _l| {
        println!("I'm repeated every 7 seconds");
    })
    .unwrap();

    sched.add(job).await.expect("Could not add job");

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Scheduler shutting down done");
        })
    }));
    sched.start().await.expect("`Could not start scheduler");

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;

    Ok(())
}
