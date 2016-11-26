use std;
use std::io::Read;
use std::net::{ TcpStream, TcpListener };

pub fn accept() -> Result<TcpStream, String> {
   let listener = try!(TcpListener::bind("127.0.0.1:10000").map_err(|e| format!("bind: {}",e)));
   let (conn, _addr) = try!(listener.accept().map_err(|_|"connection refused"));
   Ok(conn)
}

pub fn main() -> Result<(), String> {
   let mut conn = try!(accept());
   
   let _ = conn.set_nonblocking(true);
   let sleeptime = std::time::Duration::from_millis(1);
   let mut buf = [0u8; 1024];
   loop {
      match conn.read(&mut buf[0..]) {
         Ok(0) => {
            println!("read: closed");
            return Ok(());
         }
         Ok(n) => {
            println!("recv[{}]: {}", n, std::str::from_utf8(&buf[0..n]).unwrap());
         }
         Err(e) => {
            match e.kind() {
               std::io::ErrorKind::WouldBlock => (),
               _ => return Err(format!("read: {}", e)),
            }
         }
      }
      std::thread::sleep(sleeptime);
   }
}
