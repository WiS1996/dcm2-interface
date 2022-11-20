use std::{error::Error, io::Write, str, time::Duration};

use serialport::{DataBits, Parity, SerialPort, SerialPortType, StopBits};

struct SerialConnection {
    port: Box<dyn SerialPort>,
}

impl SerialConnection {
    fn new(port_name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            port: serialport::new(port_name, 19200)
                .data_bits(DataBits::Eight)
                .parity(Parity::None)
                .stop_bits(StopBits::One)
                .timeout(Duration::from_millis(100))
                .open()?,
        })
    }

    fn test_inf_response(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; 1000];
        if let Err(e) = self.port.write_all(b"INF\n") {
            Err(Box::new(e))
        } else if let Err(e) = self.port.read(buf.as_mut()) {
            Err(Box::new(e))
        } else {
            let s = str::from_utf8(&buf)?;
            println!("{}", s); // compare with the expected output of the DCM2
            Ok(())
        }
    }
}

fn find_serialport() -> Result<Box<SerialConnection>, Box<dyn Error>> {
    let ports = serialport::available_ports()?;
    for p in ports {
        match p.port_type {
            SerialPortType::UsbPort(_) => {
                println!("Found USB port: {}", p.port_name);
                let mut test_port = SerialConnection::new(&p.port_name)?;
                match test_port.test_inf_response() {
                    Ok(_) => return Ok(Box::new(test_port)),
                    Err(e) => println!("Error: {}", e),
                }
            }
            _ => println!("Found non-USB port: {} => skipping", p.port_name),
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No USB port found",
    )))
}

fn main() {
    match find_serialport() {
        Ok(dcm2_port) => println!("Found port: {}", dcm2_port.port.name().as_ref().unwrap()),
        Err(e) => println!("Error: {}", e),
    }
}
