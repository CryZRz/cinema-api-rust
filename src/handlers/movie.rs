use crate::{
    error::{AppError, ErrorResponse},
    models::movie::{self, ActiveModel, CreateMovieRequest, Entity as MovieEntity, UpdateMovieRequest},
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
    path = "/api/movies",
    responses(
        (status = 200, description = "List all movies", body = Vec<movie::Model>),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Movies"
)]
pub async fn list_movies(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<movie::Model>>, AppError> {
    let movies = MovieEntity::find().all(&db).await?;
    Ok(Json(movies))
}

#[utoipa::path(
    post,
    path = "/api/movies",
    request_body = CreateMovieRequest,
    responses(
        (status = 201, description = "Movie created successfully", body = movie::Model),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Movies"
)]
pub async fn create_movie(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateMovieRequest>,
) -> Result<(StatusCode, Json<movie::Model>), AppError> {
    let active_movie = ActiveModel {
        title: Set(payload.title),
        synopsis: Set(payload.synopsis),
        duration_minutes: Set(payload.duration_minutes),
        genre: Set(payload.genre),
        rating: Set(payload.rating),
        is_active: Set(true),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let movie = active_movie.insert(&db).await?;
    Ok((StatusCode::CREATED, Json(movie)))
}

#[utoipa::path(
    get,
    path = "/api/movies/{id}",
    params(
        ("id" = i64, Path, description = "Movie identifier")
    ),
    responses(
        (status = 200, description = "Movie found", body = movie::Model),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Movies"
)]
pub async fn get_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<movie::Model>, AppError> {
    let movie = MovieEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Movie with id {} not found", id)))?;

    Ok(Json(movie))
}

#[utoipa::path(
    put,
    path = "/api/movies/{id}",
    params(
        ("id" = i64, Path, description = "Movie identifier")
    ),
    request_body = UpdateMovieRequest,
    responses(
        (status = 200, description = "Movie updated successfully", body = movie::Model),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Movies"
)]
pub async fn update_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMovieRequest>,
) -> Result<Json<movie::Model>, AppError> {
    let movie = MovieEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Movie with id {} not found", id)))?;

    let mut active: ActiveModel = movie.into_active_model();

    if let Some(title) = payload.title {
        active.title = Set(title);
    }
    if let Some(synopsis) = payload.synopsis {
        active.synopsis = Set(synopsis);
    }
    if let Some(duration_minutes) = payload.duration_minutes {
        active.duration_minutes = Set(duration_minutes);
    }
    if let Some(genre) = payload.genre {
        active.genre = Set(genre);
    }
    if let Some(rating) = payload.rating {
        active.rating = Set(rating);
    }
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }

    let updated = active.update(&db).await?;
    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/api/movies/{id}",
    params(
        ("id" = i64, Path, description = "Movie identifier")
    ),
    responses(
        (status = 204, description = "Movie deleted successfully"),
        (status = 404, description = "Movie not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Movies"
)]
pub async fn delete_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let movie = MovieEntity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Movie with id {} not found", id)))?;

    let active: ActiveModel = movie.into_active_model();
    active.delete(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}
