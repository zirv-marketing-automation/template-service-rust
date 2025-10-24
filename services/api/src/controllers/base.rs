use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn test_health_check() {
        let app =
            actix_web::test::init_service(actix_web::App::new().service(super::health_check)).await;
        let req = actix_web::test::TestRequest::get().uri("/").to_request();
        let resp = actix_web::test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_not_found() {
        let resp = super::not_found()
            .await
            .respond_to(&actix_web::test::TestRequest::default().to_http_request());
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
