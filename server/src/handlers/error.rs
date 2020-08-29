use actix_web::dev::{self, Body, ResponseBody};
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::web::Bytes;
use actix_web::{http, Result};

pub fn error_handler(mut res: dev::ServiceResponse<Body>) -> Result<ErrorHandlerResponse<Body>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );

    let res = res.map_body(|_, response_body| {
        if let ResponseBody::Other(body) = response_body {
            if let Body::Bytes(bytes) = body {
                let message = serde_json::to_string(&String::from_utf8_lossy(&bytes)).unwrap();
                let json = format!(r#"{{"error": {}}}"#, message);

                ResponseBody::Body(Body::Bytes(Bytes::from(json)))
            } else {
                ResponseBody::Body(body)
            }
        } else {
            response_body
        }
    });

    Ok(ErrorHandlerResponse::Response(res))
}
