// author: xunfei
// Project: untitled4
// File: ttmp
// Date: 2023/12/26 21:16
use crate::model::ttmp::{get_service, get_vehicle_id, Service, Vehicle};

pub async fn vehicle2service() -> Result<(i64, i64), sqlx::Error> {
    let vehicle: Vehicle = get_vehicle_id().await?;
    let service: Service = get_service().await?;
    Ok((vehicle.vehicle_id, service.service_id))
}
