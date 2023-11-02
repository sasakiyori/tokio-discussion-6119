mod proto;
mod xx_service;

use std::sync::{Arc, Mutex};

use proto::xx::{xx_service_server::XxServiceServer, TailResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Status};

use xx_service::XXService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel(1);
    // read thread
    read_buffer(sender);
    // grpc server
    let addr = "0.0.0.0:50050".parse()?;
    let xx_service = XXService::new(Arc::new(Mutex::new(receiver)));
    Server::builder()
        .add_service(XxServiceServer::new(xx_service))
        .serve(addr)
        .await?;

    Ok(())
}

pub fn read_buffer(sender: mpsc::Sender<Result<TailResponse, Status>>) {
    tokio::spawn(async move {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let data = TailResponse { data: buf };
        let res = Result::Ok(data);
        sender.send(res).await.unwrap();
    });
}
