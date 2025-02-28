# templogger-ru


Temperature logger using a Raspberry pi and a few Dallas 18b20 sensors
* DS18b20 temperature probe temperature sensor 1M 2M 3M DS1820 Stainless steel package Waterproof 18B20 100cm/200cm/300cm
  * Sending data over Udp to server 
    * More rubust
      * Maybe migrate to pico?
    * Better storage options on server - influx db
    * visualization on server

Inspiration from :
*  https://github.com/awendland/rpi-ds18b20-rust

New Iteration of my
* https://github.com/perolo/rutemplogger
  * Written in rust with a database on logger unit (flashcard eventually broke...))
    
* https://github.com/perolo/templogger
  * Written in go with a database on logger unit (not stable)


This is my personal learning journey to NixOs, Embedded Rust and Raspberry Pi development, no guarantees what so ever

## Getting Started


### Using Nix flake
```bash
nix develop
# Build
cargo build


```
## Output


## License

Licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

# Features

* Reads the sensors every 30s, sends data to server, data stored in influx database

## Backlog
* Pico instead
* Datastore moved to server
* Real production setup influx db
* Sensor names in database + graph
* properties file on commandline
* Nix derivation

## Problems
* Intermittent communcations problems with sensors - Move to soldered connections

## Other references

* 