use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;

use crate::models::aliment::{AlimentInDTO, AlimentQuantityInDTO};
use crate::repository::aliment_repository;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/aliment";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .service(
                web::resource("")
                    .route(web::get().to(get_all))
                    .route(web::post().to(post_one)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::put().to(put_one))
                    .route(web::delete().to(delete_one)),
            ),
    );
}

async fn get_all(ThinData(db_pool): ThinData<Pool>) -> Result<HttpResponse, AppRequestError> {
    info!("/GET aliment");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let aliments = aliment_repository::get(&client)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Ok().json(aliments))
}

async fn post_one(
    ThinData(db_pool): ThinData<Pool>,
    aliment_json: web::Json<AlimentInDTO>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST aliment");

    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let aliment_json = aliment_json.into_inner();

    let aliment_id = aliment_repository::insert(&db_client, &aliment_json)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::Created().body(aliment_id.to_string()))
}

async fn put_one(
    ThinData(db_pool): ThinData<Pool>,
    aliment_quantity_json: web::Json<AlimentQuantityInDTO>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/PUT aliment/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    let aliment_quantity_json = aliment_quantity_json.into_inner();

    aliment_repository::update(&client, aliment_quantity_json.quantity, id)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn delete_one(
    ThinData(db_pool): ThinData<Pool>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/DELETE aliment/{id}");

    let client: Client = db_pool
        .get()
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let id = id.into_inner();
    aliment_repository::delete(&client, id)
        .await
        .map_err(AppRequestError::InternalDbError)?;

    Ok(HttpResponse::NoContent().finish())
}
