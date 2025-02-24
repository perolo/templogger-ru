mod ds18b20;
mod w1_errors;
use std::{thread, time};

use std::fs::File;
use std::io::BufReader;
use std::env;
use java_properties::read;

fn main() {

    let mut file_name = env::current_dir().unwrap();
    file_name.push("rutemplogger.properties");


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