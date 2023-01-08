use crate::store::Store;
use warp::http::StatusCode;
use warp::reply::with_status;

pub async fn health(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.ping().await {
        Ok(p) => Ok(warp::reply::with_status(
            format!("Healthy {}", p),
            StatusCode::OK,
        )),
        Err(err) => Ok(with_status(
            format!("Not healthy {:?}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
