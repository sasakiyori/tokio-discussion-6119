use std::{pin::Pin, sync::Mutex};

use futures_core::Stream;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::proto::xx::{xx_service_server::XxService, TailRequest, TailResponse};

pub struct XXService {
    receiver: Mutex<Option<Receiver<Result<TailResponse, Status>>>>,
}

impl XXService {
    pub fn new(receiver: Mutex<Option<Receiver<Result<TailResponse, Status>>>>) -> Self {
        Self { receiver }
    }

    fn take_receiver(&self) -> Option<Receiver<Result<TailResponse, Status>>> {
        self.receiver.lock().unwrap().take()
    }
}

#[tonic::async_trait]
impl XxService for XXService {
    type TailStream = Pin<Box<dyn Stream<Item = Result<TailResponse, Status>> + Send + 'static>>;

    async fn tail(
        &self,
        request: Request<TailRequest>,
    ) -> Result<Response<Self::TailStream>, Status> {
        // access control before take receiver
        // ...

        let reply_stream = ReceiverStream::new(self.take_receiver().unwrap());
        Ok(Response::new(Box::pin(reply_stream) as Self::TailStream))
    }
}
