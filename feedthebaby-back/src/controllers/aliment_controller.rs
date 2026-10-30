use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::middlewares::authentication_middleware::AuthenticationMiddleware;

use crate::models::aliment::AlimentInDTO;
use crate::repository::aliment_repository;
use actix_web::{
    HttpResponse,
    web::{self, ThinData},
};
use deadpool_postgres::{Client, Pool};
use log::info;
use crate::models::enums::PersistenceType::PersistenceType;

const ENDPOINT: &str = "/aliments";

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ENDPOINT)
            .wrap(AuthenticationMiddleware)
            .service(
                web::resource("")
                    .route(web::get().to(get))
                    .route(web::post().to(post)),
            )
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

    for aliment in aliment_json {
        match aliment.persistence_type {
            PersistenceType::CREATE => {
                let aliment_id = aliment_repository::insert(&db_client, &aliment)
                    .await
                    .map_err(AppRequestError::InternalDbError)?;

                info!("Insertion aliment avec ID : {}", aliment_id);
            }
            PersistenceType::UPDATE => {
                info!("Mise à jour aliment avec ID : {}", aliment.id);
                aliment_repository::update(&db_client, aliment.quantity, aliment.id)
                    .await
                    .map_err(AppRequestError::InternalDbError)?;
            }
            PersistenceType::DELETE => {
                info!("Suppression aliment avec ID : {}", aliment.id);
                aliment_repository::delete(&db_client, aliment.id)
                    .await
                    .map_err(AppRequestError::InternalDbError)?;
            }
        }
    }

    Ok(HttpResponse::Ok().finish())
}
