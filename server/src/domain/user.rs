use async_graphql::*;

#[derive(Clone, Enum, Copy, PartialEq, Eq)]
pub enum UserMessage {
    PING,
}
