use serde::Serialize;

use crate::horo_date_time::HoroDateTime;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Serialize)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct LongResponser {
    date: HoroDateTime,
    long: f64,
}

impl LongResponser {
    pub fn new(date: HoroDateTime, long: f64) -> Self {
        Self { date, long }
    }
}
