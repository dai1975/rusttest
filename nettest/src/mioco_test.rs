use std;
use mioco;
use std::str::FromStr;
use std::io::Read;

pub fn bind() -> Result<mioco::tcp::TcpListener, String> {
   let addr          = try!(std::net::SocketAddr::from_str("127.0.0.1:10000").map_err(|e| format!("parse: {}", e)));
   let listener      = try!(mioco::tcp::TcpListener::bind(&addr).map_err(|e| format!("bind: {}",e)));
   return Ok(listener);
}

pub fn main() -> Result<(), String> {
   mioco::start(|| -> Result<(), String> {
      let listener: mioco::tcp::TcpListener = try!(bind());

      loop {
         let mut conn: mioco::tcp::TcpStream = try!(listener.accept().map_err(|e| format!("accept: {}", e)));

         mioco::spawn(move || -> Result<(), String> {
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
                     return Err(format!("read: {}", e));
                  }
               }
            }
         });
      }
   }).unwrap()
}



