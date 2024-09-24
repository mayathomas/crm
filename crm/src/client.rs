use anyhow::Result;
use crm::pb::{crm_client::CrmClient, WelcomeRequestBuilder};
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let pem = include_str!("../../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem))
        .domain_name("localhost");
    let channel = Channel::from_static("https://[::1]:50000")
        .tls_config(tls)?
        .connect()
        .await?;
    // let addr = format!("http://[::1]:50000");
    // let addr = format!("http://127.0.0.1:80");
    let token = include_str!("../../fixtures/token");
    println!("{token}");
    let token: MetadataValue<_> = format!("Bearer {token}").parse()?;
    let mut client = CrmClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    let req = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(90u32)
        .content_ids([1u32, 2, 3])
        .build()?;

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("Response: {:?}", response);

    Ok(())
}
