use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::data::bd_handler_server::{BdHandler, BdHandlerServer};
use crate::data::{HelloRequest, HelloResponse};

pub mod data {
    tonic::include_proto!("data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let handler = Handler::default();

    println!("Server start on {}", addr);

    Server::builder()
        .add_service(BdHandlerServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct Handler {}

#[tonic::async_trait]
impl BdHandler for Handler {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Получен запрос от: {:?}", request.remote_addr());

        let reply = HelloResponse {
            message: format!("Hello, {}", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
