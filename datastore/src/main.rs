use chrono::prelude::*;
use influxdb_rs::{Client, Point, Value};
use postcard::from_bytes;
use protocol::Temperature;
use std::net::UdpSocket;
use tokio;
use url::Url;

#[tokio::main]
async fn main() {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:5000").expect("Could not bind to UDP socket");

    let client: Client = Client::new(
        Url::parse("http://localhost:8086").unwrap(),
        "temperature",
        "root",
        "tHWZxGg3_wdzqLUmikkOwXZtylFnD8l1u9sOGbeZuWYMiThBck6LO9qvexMwNrL4iOnxQQp8cVZG_dSkxGmUYw==",
    )
    .await
    .unwrap();

    let mut buf: [u8; 1024] = [0u8; 1024];

    loop {
        let (size, _src) = socket
            .recv_from(&mut buf)
            .expect("Failed to receive UDP packet");

        if let Ok(data) = from_bytes::<Temperature>(&buf[..size]) {
            println!("Received: {:?}", data);

            let now: DateTime<Utc> = Utc::now();
            let point: Point<'_> = Point::new("temperature")
                .add_tag("sensor_id", Value::Integer(data.id.into()))
                .add_field("temperature", Value::Float(data.temperature.into()))
                .add_timestamp(now.timestamp());

            let result: Result<(), influxdb_rs::Error> =
                client.write_point(point, None, None).await;
            if let Err(e) = result {
                eprintln!("Failed to write to InfluxDB: {:?}", e);
            }
        } else {
            eprintln!("Failed to deserialize received data");
        }
    }
}
