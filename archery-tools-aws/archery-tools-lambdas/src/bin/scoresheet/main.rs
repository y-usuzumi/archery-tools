use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    http::HeaderMap,
};
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

// async fn func(event: LambdaEvent<Point>) -> Result<Distance, Error> {
//     let (point, _ctx) = event.into_parts();

//     let result = f64::sqrt(point.x as f64 * point.x as f64 + point.y as f64 * point.y as f64);

//     Ok(Distance { value: result })
// }

async fn func(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (point, _ctx) = event.into_parts();

    let point: Point = serde_json::from_str(&point.body.unwrap())?;

    let result = f64::sqrt(point.x as f64 * point.x as f64 + point.y as f64 * point.y as f64);

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "text/html".parse().unwrap());
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        multi_value_headers: headers.clone(),
        is_base64_encoded: false,
        body: Some((serde_json::to_string(&Distance { value: result })?).into()),
        headers,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let func = service_fn(func);

    lambda_runtime::run(func).await?;
    Ok(())
}
