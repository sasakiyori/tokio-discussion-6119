use std::pin::Pin;

use futures_core::Stream;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::proto::xx::{xx_service_server::XxService, TailRequest, TailResponse};

pub struct XXService {
    receiver: Receiver<Result<TailResponse, Status>>,
}

impl XXService {
    pub fn new(receiver: Receiver<Result<TailResponse, Status>>) -> Self {
        Self { receiver }
    }
}

#[tonic::async_trait]
impl XxService for XXService {
    type TailStream = Pin<Box<dyn Stream<Item = Result<TailResponse, Status>> + Send + 'static>>;

    async fn tail(
        &self,
        request: Request<TailRequest>,
    ) -> Result<Response<Self::TailStream>, Status> {
        // Error:
        // cannot move out of `__self.receiver` which is behind a shared reference
        // move occurs because `__self.receiver` has type `tokio::sync::mpsc::Receiver<Result<TailResponse, Status>>`,
        // which does not implement the `Copy` trait
        let reply_stream = ReceiverStream::new(self.receiver);
        Ok(Response::new(Box::pin(reply_stream) as Self::TailStream))
    }
}
