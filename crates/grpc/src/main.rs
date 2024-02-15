pub mod service {
    tonic::include_proto!("yxy");
}

pub struct MyService();

#[tonic::async_trait]
impl service::greeter_server::Greeter for MyService {
    async fn say_hello(
        &self,
        request: tonic::Request<service::HelloRequest>,
    ) -> Result<tonic::Response<service::HelloReply>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let reply = service::HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyService();

    tonic::transport::Server::builder()
        .add_service(service::greeter_server::GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
