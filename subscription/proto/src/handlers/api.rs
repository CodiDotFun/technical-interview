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


#[tonic::async_trait]
impl SubscriptionService for SubscriptionHandler {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            message: format!("Pong {}", request.get_ref().message),
        }))
    }
