use std::io::IoSlice;
use zmq::{SNDMORE};
use std::{io, str};

pub struct ContextServiceStatus {
    context_name: String,
    host: String,
    pub is_running: bool,
    pub port: u16,
    process_id: u32,
    stdout: String,
    stderr: String,
}

// pub fn check_context_status(context_name: &str) -> ContextServiceStatus {
//     let kiara_dirs = directories::ProjectDirs::from("org", "kiara", "kiara").unwrap();
// }

pub fn check_context_status_old(context_name: &str) {

    const version_min: u8 = 0;
    const version_maj: u8 = 0;

    let version:Box<[u8]> = Box::from(vec!(version_maj, version_min));

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::REQ).unwrap();

    let conn = socket.connect("tcp://127.0.0.1:8000").unwrap();
    println!("{:?}", conn);

    socket.send(version, SNDMORE);
    socket.send("ping", 0);

    let response: Vec<Vec<u8>> = socket.recv_multipart(0).expect("recv failed");

    let version = &response[0];
    let endpoint = IoSlice::new(&response[1]);
    let data = IoSlice::new(&response[2]);
    let response_string = str::from_utf8(&data).unwrap();
    println!("version: {:?}", version);
    println!("endpoint: {:?}", endpoint);
    println!("data: {:?}", response_string);
}
