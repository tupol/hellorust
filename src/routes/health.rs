use warp::http::StatusCode;

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Healthy".to_string(),
        StatusCode::OK,
    ))
}
