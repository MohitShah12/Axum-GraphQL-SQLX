use async_graphql::{*};
use sqlx:: PgPool;
use crate::model::User;

pub struct Query;

// Implement methods for the query object
#[Object]
impl Query {
    // Asynchronously return a string
    async fn hello(&self) -> String {
        "Hello, world!!!!!!".to_string()
    }
    async fn hello_rorld(&self) -> &'static str {
        "Hello, world!"
    }

    async fn get_users(&self, context:&Context<'_>) -> Result<Option<Vec<User>>, String>{
        let db_pool = match context.data::<PgPool>() {
            Ok(db) => db,
            Err(err) => return Err(err.message.to_string()),
        };

        let res = match sqlx::query_as::<_, User>(
            "SELECT *,uuid::text,created_at::text FROM public.user",
        )
        .fetch_all(db_pool)
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        Ok(Some(res))
    }

}