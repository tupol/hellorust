use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::Filter;

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let store = store::Store::new("postgres://maverick:maverick@localhost:5432/datagen").await;
    let store_filter = warp::any().map(move || store.clone());

    // println!("{:?}", store.select().await);

    // POST /token
    let token = warp::post()
        .and(warp::path("token"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(routes::token::print_request);

    let health = warp::get()
        .and(warp::path("_admin"))
        .and(warp::path("health"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::health::health);

    let routes = health.or(token);

    println!("Hellorust open for e-business");

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;

    Ok(())
}
