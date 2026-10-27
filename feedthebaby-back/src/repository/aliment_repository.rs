use crate::errors::db_error::DbError;
use crate::models::aliment::{AlimentInDTO, AlimentOutDTO};
use deadpool_postgres::Client;

pub async fn get(client: &Client) -> Result<Vec<AlimentOutDTO>, DbError> {
    let _stmt = include_str!("sql/aliment/get.sql");
    let _stmt = client.prepare(_stmt).await?;

    let aliments = client
        .query(&_stmt, &[])
        .await?
        .iter()
        .map(|row| AlimentOutDTO {
            id: row.get(0),
            name: row.get(1),
            quantity: row.get(2),
            expire_at: row.get(3),
            aliment_type: row.get(4),
        })
        .collect::<Vec<AlimentOutDTO>>();

    Ok(aliments)
}

pub async fn update(client: &Client, quantity: i32, id: i64) -> Result<(), DbError> {
    let _stmt = include_str!("sql/aliment/update.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&quantity, &id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}

pub async fn insert(client: &Client, aliment_dto: &AlimentInDTO) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/aliment/insert.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(
            &_stmt,
            &[
                &aliment_dto.name,
                &aliment_dto.quantity,
                &aliment_dto.expire_at,
                &aliment_dto.aliment_type,
            ],
        )
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn delete(client: &Client, id: i64) -> Result<(), DbError> {
    let _stmt = include_str!("./sql/aliment/delete.sql");
    let _stmt = client.prepare(_stmt).await?;

    client
        .query(&_stmt, &[&id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)?;

    Ok(())
}
