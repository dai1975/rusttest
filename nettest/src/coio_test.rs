use std;
use coio;
//use std::str::FromStr;
use std::io::Read;

pub fn main() -> Result<(), String> {
   coio::Scheduler::new().with_workers(4).run(|| {
      let listener = coio::net::TcpListener::bind("127.0.0.1:10000").unwrap();

      loop {
         let (mut conn, _addr) = try!(listener.accept().map_err(|e| format!("accept: {}", e)));

         coio::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
               match conn.read(&mut buf[0..]) {
                  Ok(0) => {
                     println!("read: closed");
                     break;
                  }
                  Ok(n) => {
                     println!("recv[{}]: {}", n, std::str::from_utf8(&buf[0..n]).unwrap());
                  }
                  Err(e) => {
                     println!("read: {}", e);
                     break;
                  }
               }
            }
         });
      }
   }).unwrap()
}



