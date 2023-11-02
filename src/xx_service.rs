use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures_core::Stream;
use tokio::sync::mpsc::Receiver;
use tonic::{Request, Response, Status};

use crate::proto::xx::{xx_service_server::XxService, TailRequest, TailResponse};

struct TailStream {
    inner: Arc<Mutex<Receiver<Result<TailResponse, Status>>>>,
}

impl Stream for TailStream {
    type Item = Result<TailResponse, Status>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.lock().unwrap().poll_recv(cx)
    }
}

impl TailStream {
    fn new(recv: Arc<Mutex<Receiver<Result<TailResponse, Status>>>>) -> Self {
        Self { inner: recv }
    }
}

pub struct XXService {
    receiver: Arc<Mutex<Receiver<Result<TailResponse, Status>>>>,
}

impl XXService {
    pub fn new(receiver: Arc<Mutex<Receiver<Result<TailResponse, Status>>>>) -> Self {
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
        // success
        let reply_stream = TailStream::new(self.receiver.clone());
        Ok(Response::new(Box::pin(reply_stream) as Self::TailStream))
    }
}
