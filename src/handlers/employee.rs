use crate::{
    error::{AppError, ErrorResponse},
    models::employee::{self, ActiveModel, CreateEmployeeRequest, Entity as EmployeeEntity, UpdateEmployeeRequest},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

#[utoipa::path(
    get,
    path = "/api/employees",
    responses(
        (status = 200, description = "List all employees", body = Vec<employee::Model>),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Employees"
)]
pub async fn list_employees(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<employee::Model>>, AppError> {
    let employees = EmployeeEntity::find().all(&db).await?;
    Ok(Json(employees))
}

#[utoipa::path(
    post,
    path = "/api/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee registered successfully", body = employee::Model),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Employees"
)]
pub async fn create_employee(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateEmployeeRequest>,
) -> Result<(StatusCode, Json<employee::Model>), AppError> {
    let active_employee = ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        phone: Set(payload.phone),
        role: Set(payload.role),
        is_active: Set(true),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let employee = active_employee.insert(&db).await?;
    Ok((StatusCode::CREATED, Json(employee)))
}

#[utoipa::path(
    get,
    path = "/api/employees/{id}",
    params(
        ("id" = i64, Path, description = "Employee identifier")
    ),
    responses(
        (status = 200, description = "Employee found", body = employee::Model),
        (status = 404, description = "Employee not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Employees"
)]
pub async fn get_employee(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<employee::Model>, AppError> {
    let employee = EmployeeEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Employee with id {} not found", id)))?;

    Ok(Json(employee))
}

#[utoipa::path(
    put,
    path = "/api/employees/{id}",
    params(
        ("id" = i64, Path, description = "Employee identifier")
    ),
    request_body = UpdateEmployeeRequest,
    responses(
        (status = 200, description = "Employee updated successfully", body = employee::Model),
        (status = 404, description = "Employee not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Employees"
)]
pub async fn update_employee(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateEmployeeRequest>,
) -> Result<Json<employee::Model>, AppError> {
    let employee = EmployeeEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Employee with id {} not found", id)))?;

    let mut active: ActiveModel = employee.into_active_model();

    if let Some(name) = payload.name {
        active.name = Set(name);
    }
    if let Some(email) = payload.email {
        active.email = Set(email);
    }
    if let Some(phone) = payload.phone {
        active.phone = Set(phone);
    }
    if let Some(role) = payload.role {
        active.role = Set(role);
    }
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }

    let updated = active.update(&db).await?;
    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/api/employees/{id}",
    params(
        ("id" = i64, Path, description = "Employee identifier")
    ),
    responses(
        (status = 204, description = "Employee deleted successfully"),
        (status = 404, description = "Employee not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Employees"
)]
pub async fn delete_employee(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let employee = EmployeeEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Employee with id {} not found", id)))?;

    let active: ActiveModel = employee.into_active_model();
    active.delete(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}
