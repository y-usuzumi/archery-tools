use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct Distance {
    value: f64,
}

async fn func(event: LambdaEvent<Point>) -> Result<Distance, Error> {
    let (point, _ctx) = event.into_parts();

    let result = f64::sqrt(point.x as f64 * point.x as f64 + point.y as f64 * point.y as f64);

    Ok(Distance { value: result })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let func = service_fn(func);

    lambda_runtime::run(func).await?;
    Ok(())
}
