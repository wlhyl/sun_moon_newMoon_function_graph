use actix_web::web;

use crate::handlers::{moon_long, new_moon_long, sun_long};

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sun_long)
        .service(moon_long)
        .service(new_moon_long);
}
