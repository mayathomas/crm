use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use crm_send::{
    pb::{
        notification_client::NotificationClient, EmailMessage, InAppMessage, SendRequest,
        SmsMessage,
    },
    AppConfig, NotificationService,
};
use futures::StreamExt;
use tokio::time::sleep;
use tonic::{transport::Server, Request};

const PORT_BASE: u32 = 60000;

#[tokio::test]
async fn test_send() -> Result<()> {
    let addr = start_server(PORT_BASE + 3).await?;
    let mut client = NotificationClient::connect(format!("http://{}", addr)).await?;
    let stream = tokio_stream::iter(vec![
        SendRequest {
            msg: Some(EmailMessage::fake().into()),
        },
        SendRequest {
            msg: Some(SmsMessage::fake().into()),
        },
        SendRequest {
            msg: Some(InAppMessage::fake().into()),
        },
    ]);

    let req = Request::new(stream);
    let response = client.send(req).await?.into_inner();
    let ret: Vec<_> = response.then(|res| async { res.unwrap() }).collect().await;
    assert_eq!(ret.len(), 3);
    Ok(())
}

async fn start_server(port: u32) -> Result<SocketAddr> {
    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", port).parse()?;
    let svc = NotificationService::new(config).into_server();
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_micros(1)).await;
    Ok(addr)
}
