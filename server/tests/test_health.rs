mod common;

#[cfg(test)]
mod tests {
    use actix_web::test;
    use serial_test::serial;

    use crate::common::app::create_app;
    use ::lib::models::health::HealthStatus;

    #[actix_rt::test]
    #[serial]
    async fn test_health() {
        let mut app = create_app().await;

        let req = test::TestRequest::get().uri("/health").to_request();

        let response: HealthStatus = test::read_response_json(&mut app, req).await;

        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        let expected = HealthStatus {
            version: VERSION.unwrap_or("").to_string(),
            api: "ok".to_string(),
            database_postgres: "ok".to_string(),
            database_influx: "ok".to_string(),
        };

        assert_eq!(response, expected);
    }
}
