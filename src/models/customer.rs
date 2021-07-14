use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "customer")]
pub struct Customer {
    pub first_name: String,
    pub last_name: String,
    pub has_newsletter_sub: bool,
}
