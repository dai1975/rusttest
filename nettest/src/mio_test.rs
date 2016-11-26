use std;
use mio;
use mio::tcp::{ TcpListener, TcpStream };
use std::str::FromStr;
use std::io::Read;

pub fn bind() -> Result<TcpListener, String> {
   let addr          = try!(std::net::SocketAddr::from_str("127.0.0.1:10000").map_err(|e| format!("parse: {}", e)));
   let listener      = try!(TcpListener::bind(&addr).map_err(|e| format!("bind: {}",e)));
   return Ok(listener);
}
pub fn accept(listener:TcpListener) -> Result<TcpStream, String> {
   let sleeptime = std::time::Duration::from_millis(1);
   loop {
      match listener.accept() {
         Ok( (conn, _addr) ) => return Ok(conn),
         Err(e) => {
            match e.kind() {
               std::io::ErrorKind::WouldBlock => (),
               _ => return Err(format!("connection refused: {}", e)),
            }
         }
      }
      std::thread::sleep(sleeptime);
   }
}

pub fn main() -> Result<(), String> {
   let poller = mio::Poll::new().unwrap();
   let token_id  = 0;
   let mut conn: mio::tcp::TcpStream = try!( bind().and_then(|l| accept(l)) );
   poller.register(&conn, mio::Token(token_id), mio::Ready::readable(), mio::PollOpt::edge()).unwrap();

   let mut buf = [0u8; 1024];
   let mut events = mio::Events::with_capacity(1024);
   loop {
      poller.poll(&mut events, None).unwrap();
      for ev in events.iter() {
         match ev.token() {
            mio::Token(id) if id == token_id => {
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
            mio::Token(_) => (),
         }
      }
   }
}



