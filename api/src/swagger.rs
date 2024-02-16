use crate::{
    handlers::{__path_moon_long, __path_new_moon_long, __path_sun_long},
    horo_date_time::HoroDateTime,
    request::{DateRangeRequest, DateRequest},
    response::LongResponser,
};
use utoipa::OpenApi;

// swagger
#[derive(OpenApi)]
#[openapi(
    paths(sun_long, moon_long, new_moon_long),
    components(schemas(LongResponser, HoroDateTime, DateRequest, DateRangeRequest))
)]
pub struct ApiDoc;
