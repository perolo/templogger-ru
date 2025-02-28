use std::net::UdpSocket;
//use surrealdb::{Surreal, opt::auth::Root};
//use surrealdb::engine::local::Mem;
//use surrealdb::engine::remote::ws::Ws;
use postcard::from_bytes;
//use serde::{Serialize, Deserialize};
use tokio;
use protocol::Temperature;
use influxdb_rs::Client;
use influxdb_rs::Point;
use influxdb_rs::Value;
//use influxdb_rs::Precision;
use url::Url;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5000").expect("Could not bind to UDP socket");
//    let socket = UdpSocket::bind("127.0.0.1:5000").expect("Could not bind to UDP socket");
//    let socket = UdpSocket::bind("127.0.0.1:5000").unwrap();

//    let db = Surreal::new::<Ws>("localhost:8000").await.expect("Failed to initialize SurrealDB");
//    db.signin(Root { username: "root", password: "root" }).await.expect("Failed to sign in");
//    db.use_ns("temperature").use_db("data").await.expect("Failed to select namespace and database");
    
    let client = Client::new(Url::parse("http://localhost:8086").unwrap(), 
                        "temperature", 
                        "root", 
                        "tHWZxGg3_wdzqLUmikkOwXZtylFnD8l1u9sOGbeZuWYMiThBck6LO9qvexMwNrL4iOnxQQp8cVZG_dSkxGmUYw==")
                        .await.unwrap();

    let mut buf = [0u8; 1024];
    
    loop {
        let (size, _src) = socket.recv_from(&mut buf).expect("Failed to receive UDP packet");
        
        if let Ok(data) = from_bytes::<Temperature>(&buf[..size]) {
            println!("Received: {:?}", data);

            let now = Utc::now();
            let ii:i32 = data.id as i32;
            println!("ii: {:?}", ii);

            let point = Point::new("temperature")
            .add_tag("sensor_id", Value::Integer(ii.into()))
            .add_field("temperature", Value::Float(data.temperature.into()))
            .add_timestamp(now.timestamp());

        let result = client.write_point(point, None, None).await;
        if let Err(e) = result {
            eprintln!("Failed to write to InfluxDB: {:?}", e);
        } else {
            println!("Data written to InfluxDB");
        }   
            /*
            let _: Option<Temperature> = db
                .create("temperature")
                .content(data)
                .await
                .expect("Failed to insert data into SurrealDB");
            */
            
        } else {
            eprintln!("Failed to deserialize received data");
        }
    }
}
