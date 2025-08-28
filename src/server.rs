use tonic::{
    Request, Response, Status,
    service::InterceptorLayer,
    transport::{Certificate, Identity, Server, ServerTlsConfig},
};
use std::time::Duration;
use std::collections::HashMap;
use tower::ServiceBuilder;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
}

fn auth_interceptor(mut request: Request<()>) -> Result<Request<()>, Status> {
    let mut users = HashMap::new();

    users.insert("admin", User {
        id: 0,
        name: "admin".to_string(),
    });

    users.insert("client_1", User {
        id: 1,
        name: "client_1".to_string(),
    });

    users.insert("client_2", User {
        id: 2,
        name: "client_2".to_string(),
    });

    let peer_certs = request.peer_certs();
    if let Some(certs) = peer_certs {
        log::info!("Peer certs: {:?}", certs.len());
        let cert = certs.iter().next().unwrap();
        let cert_bytes = cert.as_ref();
        match x509_parser::parse_x509_certificate(cert_bytes) {
            Ok((_, x509_cert)) => {
                let cnname = x509_cert.subject().to_string();
                let name = cnname.split("CN=").nth(1).unwrap_or_default().to_string();
                let user = users.get(name.as_str()).unwrap();
                log::info!("User: {:?}", user);
                request.extensions_mut().insert(user.clone());
            }
            Err(e) => {
                log::warn!("Failed to parse peer certificate: {:?}", e);
            }
        }
    }

    

    Ok(request)
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        log::info!("Got a request: {:?}", request);
        let user = request.extensions().get::<User>().unwrap();
        log::info!("User: {:?}", user);
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // Ensure rustls selects a crypto provider (ring or aws-lc-rs)
    let _ = rustls::crypto::ring::default_provider().install_default();
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    let server_cert_pem = std::fs::read("certs/server.crt")?;
    let server_key_pem = std::fs::read("certs/server.key")?;
    let ca_cert_pem = std::fs::read("certs/ca.crt")?;

    let identity = Identity::from_pem(server_cert_pem, server_key_pem);
    let client_ca = Certificate::from_pem(ca_cert_pem);

    let tls = ServerTlsConfig::new()
        .identity(identity)
        .client_ca_root(client_ca);

    let layer = ServiceBuilder::new()
        .load_shed()
        .layer(InterceptorLayer::new(auth_interceptor))
        .into_inner();

    Server::builder()
        .tls_config(tls)?
        .layer(layer)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
