use hello_world::HelloRequest;
use hello_world::greeter_client::GreeterClient;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // Ensure rustls selects a crypto provider (ring or aws-lc-rs)
    let _ = rustls::crypto::ring::default_provider().install_default();

    let ca_cert_pem = std::fs::read("certs/ca.crt")?;
    let client_cert_pem = std::fs::read("certs/client_2.crt")?;
    let client_key_pem = std::fs::read("certs/client_2.key")?;

    let ca = Certificate::from_pem(ca_cert_pem);
    let identity = Identity::from_pem(client_cert_pem, client_key_pem);

    let tls = ClientTlsConfig::new().ca_certificate(ca).identity(identity);

    let channel = Channel::from_shared("https://localhost:50051".to_string())?
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = GreeterClient::new(channel);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
