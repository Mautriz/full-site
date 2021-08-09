use async_graphql::{
    futures_util::StreamExt, Context, MergedObject, MergedSubscription, Object, Schema,
    SchemaBuilder, Subscription,
};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, Stream};

use crate::{context::MyContext, domain::user::UserMessage};

#[derive(Default)]
pub struct UserQuery;
#[Object]
impl UserQuery {
    async fn ciao(&self) -> u32 {
        12
    }
}

#[derive(Default)]
pub struct UserMutation {}

#[Object]
impl UserMutation {
    async fn provissima(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct UserSubscription {}

#[Subscription]
impl UserSubscription {
    async fn ahah(&self, ctx: &Context<'_>) -> impl Stream<Item = UserMessage> {
        let data = ctx.data_unchecked::<MyContext>();
        let tx = data.user_message_tx.clone();

        for i in 0..100 {
            let txc = tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(i)).await;
                let _ = txc.send(crate::domain::user::UserMessage::PING);
            });
        }

        let rx_stream = BroadcastStream::new(tx.subscribe()).filter_map(|x| async move { x.ok() });

        rx_stream
    }
}

#[derive(MergedObject, Default)]
pub struct MainQuery(UserQuery);

#[derive(MergedObject, Default)]
pub struct MainMutation(UserMutation);

#[derive(MergedSubscription, Default)]
pub struct MainSubcription(UserSubscription);

pub type MySchemaBuilder = SchemaBuilder<MainQuery, MainMutation, MainSubcription>;
pub type MySchema = Schema<MainQuery, MainMutation, MainSubcription>;
pub fn get_schema() -> MySchemaBuilder {
    Schema::build(
        MainQuery::default(),
        MainMutation::default(),
        MainSubcription::default(),
    )
    .limit_complexity(200)
}
