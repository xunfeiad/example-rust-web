// author: xunfei
// Project: untitled4
// File: ttmp
// Date: 2023/12/26 21:16

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Vehicle {
    pub vehicle_id: i64,
}
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Service {
    pub service_id: i64,
}

use crate::common::pool::{get_pool, PostgresqlPoll};
use crate::common::variable::{MAX_CONNECTIONS, URL};
use serde::{Deserialize, Serialize};

/// For delete vehicle
/// Because if the vehicle with `onboard` tire that can't not be deleted.
/// So we query one vehicle that have no `onboard` tire.
/// Provide API for auto-test use.
pub async fn get_vehicle_id() -> Result<Vehicle, sqlx::Error> {
    let pool = get_pool(PostgresqlPoll::new(MAX_CONNECTIONS, URL)).await?;
    let vehicle: Vehicle = sqlx::query_as(
        "select vehicle.id as vehicle_id
             from main.operation_tirestatus tire
                join main.configuration_vehicle vehicle on vehicle.id = tire.vehicle_id
             where tire._deleted = 0
                and vehicle._deleted = 0
                and tire.tire_status != 'onboard' limit 1",
    )
    .fetch_one(&pool)
    .await?;
    Ok(vehicle)
}

// For delete job_sheet
pub async fn get_service() -> Result<Service, sqlx::Error> {
    let pool = get_pool(PostgresqlPoll::new(MAX_CONNECTIONS, URL)).await?;
    let service: Service = sqlx::query_as(
        "select service.id as service_id
            from main.vehicle_service service
                     join main.configuration_vehicle vehicle on service.id = vehicle.last_inspection_id
                     join main.operation_tirestatus tire on vehicle.id = tire.vehicle_id
            where tire._deleted = 0
              and vehicle._deleted = 0
              and service._deleted = 0
            limit 1")
        .fetch_one(&pool)
        .await?;
    Ok(service)
}
