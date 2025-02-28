mod ds18b20;
mod w1_errors;
use java_properties::read;

use std::{fs::File, io::BufReader, thread, time, net::UdpSocket};
use postcard::to_stdvec;
use protocol::Temperature;



fn main() {

    let file_name = "/home/pero/src/rust/pero/templogger-ru/logger/src/rutemplogger.properties"; //env::current_dir().unwrap();

    // Reading
    let  f = File::open(&file_name).unwrap();
    let map2 = read(BufReader::new(f)).unwrap();
    println!("map{} ", map2["sensornames"]);

    let expectedsensors: Vec<&str> = map2["expectedsensors"].split(',').collect();
    let sensornames: Vec<&str> = map2["sensornames"].split(',').collect();

    if expectedsensors.len() != sensornames.len() {
        println!("expected vectors of equal length");  
        panic!("Panicking...");          
    }

    let target_ip = "192.168.50.60:5000"; // Change this to your target IP and port
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to UDP socket");

    let sensors = ds18b20::DS18B20::new().unwrap();

    for sensor in &sensors.w1_id {
        let s = String::from(sensor);
        //let ss = String::from(sensor);
        let i = expectedsensors.iter().position(|&r| r == &s).unwrap();
        if expectedsensors.iter().any(|e| e == &s) {
            println!("expectedsensor found:{} name: {}", s, sensornames[i]);
        } else {
            println!("expectedsensor not found:{} ", s);  
            panic!("Panicking...");          
        }
    }

    let sleep_delay = time::Duration::from_secs(30);
    let mut now = time::Instant::now();
    loop {
        
        for sensor in &sensors.w1_id {
            let s = String::from(sensor);
            //let ss = String::from(sensor);
            let temp_res = sensors.read_temp(s.clone());
            match temp_res {
                Err(_error) => println!("Error reading {sensor}" ),
                Ok(temp) =>  {
                        let t = temp.to_celsius();
                        let i = expectedsensors.iter().position(|&r| r == &s).unwrap();
                        println!("{} no {}: {:.1} C", sensor, &i, &t);
                        let ii: u8 = i as u8;
                        let data = Temperature {  id: ii ,temperature:t};
                        //let data = CpuTemperature { temperature: temp };
                        match to_stdvec(&data) {
                            Ok(serialized) => {
                                let _ = socket.send_to(&serialized, target_ip);
                            }
                            Err(e) => eprintln!("Serialization error: {}", e),
                        }
                },
            }
        }
        let read_done = time::Instant::now();
        now = now.checked_add(sleep_delay).unwrap();
        let dur = now - read_done;
        println!("Sleep {:#?} s",  dur );
        thread::sleep(dur);
    }
}