use tonic::{
    Request, Response, Status,
    transport::{Certificate, Identity, Server, ServerTlsConfig},
};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
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

        let peer_certs = request.peer_certs();
        if let Some(certs) = peer_certs {
            log::info!("Peer certs: {:?}", certs.len());
            let cert = certs.iter().next().unwrap();
            let cert_bytes = cert.as_ref();
            match x509_parser::parse_x509_certificate(cert_bytes) {
                Ok((_, x509_cert)) => {
                    log::info!("Peer cert subject: {}", x509_cert.subject());
                    log::info!("Peer cert issuer: {}", x509_cert.issuer());
                    log::info!("Peer cert extensions: {:?}", x509_cert.extensions());
                    log::info!("Peer cert raw serial: {:?}", x509_cert.raw_serial());
                    log::info!("Peer cert version: {:?}", x509_cert.version());
                }
                Err(e) => {
                    log::warn!("Failed to parse peer certificate: {:?}", e);
                }
            }
        }

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

    Server::builder()
        .tls_config(tls)?
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
