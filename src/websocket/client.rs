use crate::Rpc;

use std::{
    format,
    sync::{
        RwLock,
        Arc,
    },
};
use async_trait::async_trait;

use tokio::{
    sync::{
        watch,
        mpsc,
    },
};

pub struct Client {
    pub handle: ezsockets::Client<Self>,
}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        println!("received message: {}", text);
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        println!("received bytes: {:?}", bytes);
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        let () = call;
        println!("received call: {:?}", call);
        Ok(())
    }
}

// Open WS connections to our nodes and accept and process internal WS calls
// whenever we receive something from incoming_rx
fn ws_conn_manager(
    rpc_list: Arc<RwLock<Vec<Rpc>>>,
    incoming_rx: mpsc::UnboundedReceiver<String>,
    outgoing_tx: watch::Sender<String>,
) -> () {
    let rpc_list_clone = rpc_list.read().unwrap().clone();


    let mut ws_handles = Vec::new();
    for rpc in rpc_list_clone {
        let config = ClientConfig::new(rpc.ws_url);
        let (handle, future) = ezsockets::connect(|handle| Client { handle }, config).await;        
        tokio::spawn(async move {
            future.await.unwrap();
        });
        ws_handles.push(handle);
    }



}

// Receive JSON-RPC call from balancer thread and respond with ws response
pub async fn execute_ws_call(call: String) -> Result<String, ezsockets::Error> {
    Ok(format!("Hello from blutgang!: {}", call))
}
