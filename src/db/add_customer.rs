use crate::{errors::my_errors::MyError, Customer};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_customer(client: &Client, user_info: Customer) -> Result<Customer, MyError> {
    let _stmt = include_str!("../../sql/add_customer.sql");
    let _stmt = _stmt.replace("$table_fields", &Customer::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.first_name,
                &user_info.last_name,
                &user_info.is_eav_customer,
                &user_info.is_redpoints_sub,
                &user_info.has_newsletter_sub
            ],
        )
        .await?
        .iter()
        .map(|row| Customer::from_row_ref(row).unwrap())
        .collect::<Vec<Customer>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}
