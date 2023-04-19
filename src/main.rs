use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;
use std::sync::mpsc::channel;
use local_ip_address::local_ip;
use const_format::formatcp;


//96 is the decimal value for the ascii character @
const END_OF_MESSAGE_VALUE : u8 = 96;
//#
const MID_MESSAGE_VALUE : u8 = 35;



fn main() {

    let (tx, rx) = channel::<TcpStream>();
    //Thread 1 handles making the connections with other computers
    //Then will punt those connections over a mspc queue to Thread 2
    let listener = thread::spawn(move || {
        let listener;

        if let Ok(server_ip) = local_ip() {
            listener = TcpListener::bind(format!("{}{}", server_ip.to_string(), ":7878")).unwrap();
        } else {
            println!("Failed to acquire server IP.");

            process::exit(1);
        }

        println!("{:?}", listener.local_addr());
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("{:?}", stream.local_addr());
            let _ = tx.send(stream);
            
            println!("Connection established!");
        }
    });

    //Thread 2 pulls connections off of the queue and then 
    let match_maker = thread::spawn(move || {
        let mut connections = vec![];
        loop {
            match rx.try_recv() {
                Ok(new_stream) => {
                    connections.push(new_stream);
                },
                Err(_) => {

                }
            }

            if connections.len() >= 2 {
                let mut a = connections.remove(0);
                let mut b = connections.remove(0);
                let mut a_message = b.peer_addr().unwrap().to_string().as_bytes().to_vec();
                let mut b_message = a.peer_addr().unwrap().to_string().as_bytes().to_vec();

                a_message.push(MID_MESSAGE_VALUE);
                a_message.push(1);
                a_message.push(END_OF_MESSAGE_VALUE);
                a_message.push(END_OF_MESSAGE_VALUE);

                b_message.push(MID_MESSAGE_VALUE);
                b_message.push(2);
                b_message.push(END_OF_MESSAGE_VALUE);
                b_message.push(END_OF_MESSAGE_VALUE);

                let _ = a.write(&a_message);
                let _ = b.write(&b_message);
            }
        }
    });


    match listener.join() {
        Ok(_) => {
            println!("Listener shut down properly");
        },
        Err(_) => {

        }
    }

    match match_maker.join() {
        Ok(_) => {
            println!("Make maker shut down properly");
        },
        Err(_) => {

        }
    }
}