use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;

use crate::models::aliment::{AlimentInDTO, AlimentOutNewIdDTO};
use crate::models::enums::persistence_type::PersistenceType;
use crate::repository::aliment_repository;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;

const ENDPOINT: &str = "/aliments";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT).wrap(AuthenticationMiddleware).service(
            web::resource("")
                .route(web::get().to(get))
                .route(web::post().to(post)),
        ),
    );
}

async fn get(ThinData(db_pool): ThinData<Pool>) -> Result<HttpResponse, AppRequestError> {
    info!("/GET aliments");

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

async fn post(
    ThinData(db_pool): ThinData<Pool>,
    aliment_json: web::Json<Vec<AlimentInDTO>>,
) -> Result<HttpResponse, AppRequestError> {
    info!("/POST aliments");

    let db_client: Client = db_pool
        .get()
        .await
        .map_err(DbError::PoolError)
        .map_err(AppRequestError::InternalDbError)?;

    let aliment_json = aliment_json.into_inner();

    // Sending new ids back to the client for the newly created aliments
    let mut new_ids: Vec<AlimentOutNewIdDTO> = Vec::with_capacity(aliment_json.len());

    for aliment in aliment_json {
        match aliment.persistence_type {
            PersistenceType::Create => {
                let aliment_id = aliment_repository::insert(&db_client, &aliment)
                    .await
                    .map_err(AppRequestError::InternalDbError)?;

                new_ids.push(AlimentOutNewIdDTO {
                    old_id: aliment.id,
                    new_id: aliment_id,
                });
            }
            PersistenceType::Update => {
                info!("Mise à jour aliment avec ID : {}", aliment.id);
                aliment_repository::update(&db_client, &aliment)
                    .await
                    .map_err(AppRequestError::InternalDbError)?;
            }
            PersistenceType::Delete => {
                info!("Suppression aliment avec ID : {}", aliment.id);
                aliment_repository::delete(&db_client, aliment.id)
                    .await
                    .map_err(AppRequestError::InternalDbError)?;
            }
        }
    }

    Ok(HttpResponse::Ok().body(
        serde_json::to_string(&new_ids).map_err(AppRequestError::InternalSerializationError)?,
    ))
}
