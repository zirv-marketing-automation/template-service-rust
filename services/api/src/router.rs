use actix_web::web;

use crate::controllers::base;

pub fn get() -> actix_web::Scope {
    web::scope("/api").service(base::health_check)
}
