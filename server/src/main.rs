use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data,
};
use async_graphql_warp;
use schema::MySchema;
use sqlx::{migrate, postgres::PgPoolOptions, PgPool};
use std::convert::Infallible;
use warp::Filter;

use crate::context::MyContext;
mod context;
mod domain;
mod schema;

#[derive(Clone)]
struct ProvaCtx {}

impl ProvaCtx {
    fn new() -> Self {
        ProvaCtx {}
    }
}

#[tokio::main]
async fn main() {
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(80)
        .connect("postgres://username:password@localhost:5555/postgres")
        .await
        .unwrap();

    //

    migrate!("./migrations").run(&pool).await.unwrap();

    sqlx::query(r"insert into users(username) values ('ao'),('frate')")
        .execute(&pool)
        .await
        .unwrap();

    let schema = schema::get_schema()
        .data(pool)
        .data(MyContext::new())
        .finish();

    let graphql_post =
        warp::path!("graphql").and(async_graphql_warp::graphql(schema.clone()).and_then(
            |(schema, request): (MySchema, async_graphql::Request)| async move {
                // Execute query
                let resp = schema.execute(request.data(ProvaCtx {})).await;

                // Return result
                Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
            },
        ));
    let graphql_ws = warp::path!("subscription").and(
        async_graphql_warp::graphql_subscription_with_data(schema, |value| async {
            println!("{:?} ao", value);
            #[derive(serde::Deserialize)]
            struct Payload {
                token: Option<String>,
            }

            if let Ok(_payload) = serde_json::from_value::<Payload>(value) {
                let mut data = Data::default();
                data.insert(ProvaCtx {});
                Ok(data)
            } else {
                Err("Token is required".into())
            }
        }),
    );

    let playground = warp::path!("playground").and(warp::get()).map(|| {
        warp::http::Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscription"),
            ))
    });

    warp::serve(graphql_post.or(graphql_ws).or(playground))
        .run(([0, 0, 0, 0], 8001))
        .await;
}
