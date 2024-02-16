use actix_web::{post, web, HttpResponse, Responder};
use swe::{swe_calc_ut, swe_close, swe_degnorm, swe_set_ephe_path, Body};

use crate::{
    error::{DateTimeError, Error},
    horo_date_time::horo_date_time,
    request::DateRangeRequest,
    response::LongResponser,
    state::AppState,
};

/// 太阳的黄道经度
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="太阳黄道经度",
    context_path="/api",
    request_body=DateRangeRequest,
    responses(
        (status = 200, description = "OK", body = Vec<LongResponser>),
    ),
)
)]
#[post("/sun")]
pub async fn sun_long(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<DateRangeRequest>,
) -> Result<impl Responder, Error> {
    // let r = r.into_inner();

    let start = horo_date_time(
        r.start.year,
        r.start.month,
        r.start.day,
        r.start.hour,
        r.start.minute,
        r.start.second,
        8.0,
        false,
    )?;

    let end = horo_date_time(
        r.end.year,
        r.end.month,
        r.end.day,
        r.end.hour,
        r.end.minute,
        r.end.second,
        8.0,
        false,
    )?;

    if end.jd_utc <= start.jd_utc {
        let err = DateTimeError::InvalidDateTime("start date 必需小于 end date".to_string());
        return Err(err.into());
    }

    let d = if end.jd_utc - start.jd_utc < 1.0 {
        1.0 / 24.0
    } else {
        1.0
    };

    let mut longs = vec![];
    let mut date = start;
    while date.jd_utc < end.jd_utc {
        swe_set_ephe_path(&app_state.ephe_path);
        let xx = swe_calc_ut(date.jd_utc, &Body::SeSun, &[])
            .map_err(|e| Error::Function(format!("计算太阳位置错误:{e}")))?;
        swe_close();

        let long = xx[0];
        let res = LongResponser::new(date.clone(), long);
        longs.push(res);

        date = date.plus_days(d)?;
    }

    let res = HttpResponse::Ok().json(longs);
    Ok(res)
}

/// 月亮的黄道经度
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="月亮黄道经度",
    context_path="/api",
    request_body=DateRangeRequest,
    responses(
        (status = 200, description = "OK", body = Vec<LongResponser>),
    ),
)
)]
#[post("/moon")]
pub async fn moon_long(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<DateRangeRequest>,
) -> Result<impl Responder, Error> {
    let start = horo_date_time(
        r.start.year,
        r.start.month,
        r.start.day,
        r.start.hour,
        r.start.minute,
        r.start.second,
        8.0,
        false,
    )?;

    let end = horo_date_time(
        r.end.year,
        r.end.month,
        r.end.day,
        r.end.hour,
        r.end.minute,
        r.end.second,
        8.0,
        false,
    )?;

    if end.jd_utc <= start.jd_utc {
        let err = DateTimeError::InvalidDateTime("start date 必需小于 end date".to_string());
        return Err(err.into());
    }

    let d = if end.jd_utc - start.jd_utc < 1.0 {
        1.0 / 24.0
    } else {
        1.0
    };

    let mut longs = vec![];
    let mut date = start;
    while date.jd_utc < end.jd_utc {
        swe_set_ephe_path(&app_state.ephe_path);
        let xx = swe_calc_ut(date.jd_utc, &Body::SeMoon, &[])
            .map_err(|e| Error::Function(format!("计算月亮位置错误:{e}")))?;
        swe_close();

        let long = xx[0];
        let res = LongResponser::new(date.clone(), long);
        longs.push(res);

        date = date.plus_days(d)?;
    }

    let res = HttpResponse::Ok().json(longs);
    Ok(res)
}

/// 新月的黄道经度
/// 月亮黄道经度-太阳黄道经度
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="新月黄道经度",
    context_path="/api",
    request_body=DateRangeRequest,
    responses(
        (status = 200, description = "OK", body = Vec<LongResponser>),
    ),
)
)]
#[post("/new_moon")]
pub async fn new_moon_long(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<DateRangeRequest>,
) -> Result<impl Responder, Error> {
    let start = horo_date_time(
        r.start.year,
        r.start.month,
        r.start.day,
        r.start.hour,
        r.start.minute,
        r.start.second,
        8.0,
        false,
    )?;

    let end = horo_date_time(
        r.end.year,
        r.end.month,
        r.end.day,
        r.end.hour,
        r.end.minute,
        r.end.second,
        8.0,
        false,
    )?;

    if end.jd_utc <= start.jd_utc {
        let err = DateTimeError::InvalidDateTime("start date 必需小于 end date".to_string());
        return Err(err.into());
    }

    let d = if end.jd_utc - start.jd_utc < 1.0 {
        1.0 / 24.0
    } else {
        1.0
    };

    let mut longs = vec![];
    let mut date = start;
    while date.jd_utc < end.jd_utc {
        swe_set_ephe_path(&app_state.ephe_path);
        let xx = swe_calc_ut(date.jd_utc, &Body::SeSun, &[])
            .map_err(|e| Error::Function(format!("计算太阳位置错误:{e}")))?;

        let sun_of_long = xx[0];

        let xx = swe_calc_ut(date.jd_utc, &Body::SeMoon, &[])
            .map_err(|e| Error::Function(format!("计算月亮位置错误:{e}")))?;
        swe_close();

        let moon_of_long = xx[0];

        let long = swe_degnorm(moon_of_long - sun_of_long);

        let res = LongResponser::new(date.clone(), long);
        longs.push(res);

        date = date.plus_days(d)?;
    }

    let res = HttpResponse::Ok().json(longs);
    Ok(res)
}
