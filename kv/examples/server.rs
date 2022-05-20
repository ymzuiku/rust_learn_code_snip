use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse, MemTable, Service};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service: Service = Service::new(MemTable::new());

    let addr = "127.0.0.1:9527";
    let listenerr = TcpListener::bind(addr).await?;
    info!("start listening on {}", addr);

    loop {
        let (stream, addr) = listenerr.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(msg)) = stream.next().await {
                info!("Got a new command: {:?}", msg);
                // let mut resp = CommandResponse::default();
                // resp.status = 404;
                // resp.message = "not found".to_string();
                // stream.send(resp).await.unwrap();
                let res = svc.execute(msg);
                stream.send(res).await.unwrap();
            }
            info!("client {:?} disconnected", addr);
        });
    }
}
