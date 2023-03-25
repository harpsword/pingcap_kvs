#![feature(type_alias_impl_trait)]
use anyhow::Ok;
use clap::Parser;
use kvs::KvsEngine;
use pilota::{lazy_static::lazy_static};
use tokio::sync::Mutex;
use std::{net::SocketAddr, sync::{Arc}};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "[::]:8080")]
    addr: String,

    #[arg(long, default_value = "kvs_store")]
    engine: String,
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

lazy_static!(
    static ref VEC:Vec<u8> = vec![0x18u8, 0x11u8];
    static ref KVSTORE: kvs::AsyncKvStore = kvs::kv::AsyncKvStore(Arc::new(Mutex::new(kvs::KvsEngine::open("example").unwrap())),);
    static ref KVSTORE_V2: Arc<Mutex<kvs::KvStore>> = Arc::new(Mutex::new(kvs::KvStore::open("asdf").unwrap()));
);

pub struct S;

fn new_base_response() -> volo_gen::kvs::BaseResponse {
    volo_gen::kvs::BaseResponse{
        status_code: 0,
        msg: "".into(),
    }
}

#[volo::async_trait]
impl volo_gen::kvs::KvsService for S {
    async fn get(
        &self,
        req: volo_gen::kvs::GetRequest,
    ) -> core::result::Result<volo_gen::kvs::GetResponse, volo_thrift::AnyhowError> {
        let b = Arc::clone(&KVSTORE_V2);
        let mut lock = b.lock().await;
        let res = lock.get(req.key.into_string())?;
        core::result::Result::Ok(volo_gen::kvs::GetResponse{
            value: res.map(|v| v.into()),
            base_response: Some(new_base_response()),
        })
    }
    
    async fn set(
        &self,
        req: volo_gen::kvs::SetRequest,
    ) -> core::result::Result<volo_gen::kvs::SetResponse, volo_thrift::AnyhowError> {
        let b = Arc::clone(&KVSTORE_V2);
        let mut lock = b.lock().await;
        lock.set(req.key.into(), req.value.into())?;
        Ok(volo_gen::kvs::SetResponse{
            base_response: None,
        })
    }

    async fn remove(
        &self,
        req: volo_gen::kvs::RemoveRequest,
    ) -> core::result::Result<volo_gen::kvs::RemoveResponse, volo_thrift::AnyhowError> {
        let b = Arc::clone(&KVSTORE_V2);
        let mut lock = b.lock().await;
        lock.remove(req.key.into())?;
        Ok(volo_gen::kvs::RemoveResponse{
            base_response: None,
        })
    }
}

#[volo::main]
async fn main() {
    init_tracing();

    let args = Args::parse();

    let addr: SocketAddr = args.addr.parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::kvs::KvsServiceServer::new(S)
        .run(addr)
        .await
        .unwrap();
}
