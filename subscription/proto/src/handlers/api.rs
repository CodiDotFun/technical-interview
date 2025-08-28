use crate::pb::subscription_service_server::SubscriptionService;
use crate::pb::{
    ListSubscriptionsRequest, ListSubscriptionsResponse, PingRequest, PingResponse,
    SubscribeRequest, SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse,
};
use tonic::{Request, Response, Status};

pub struct SubscriptionHandler {
    pool: sqlx::SqlitePool,
}
impl SubscriptionHandler {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}
