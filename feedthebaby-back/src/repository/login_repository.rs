use crate::errors::app_request_error::AppRequestError;
use crate::errors::db_error::DbError;
use crate::models::login::LoginDTO;
use deadpool_postgres::Client;
use tokio_postgres::Row;

pub async fn login(client: &Client, login_dto: LoginDTO) -> Result<i64, AppRequestError> {
    let _stmt = include_str!("./sql/users/login.sql");
    let _stmt = client
        .prepare(_stmt)
        .await
        .map_err(DbError::from)
        .map_err(AppRequestError::InternalDbError)?;

    let rows = client
        .query(&_stmt, &[&login_dto.plain_pwd])
        .await
        .map_err(DbError::PGError)
        .map_err(AppRequestError::InternalDbError)?;

    let row = rows
        .iter()
        .collect::<Vec<&Row>>()
        .pop()
        .ok_or(DbError::NotFound)
        .map_err(AppRequestError::InternalDbError)?;

    let user_id: i64 = row.get(0);
    Ok(user_id)
}
