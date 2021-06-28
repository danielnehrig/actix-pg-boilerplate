use crate::{db, errors::my_errors::MyError, Customer};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn add_customer(
    user: web::Json<Customer>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: Customer = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_customer::add_customer(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
