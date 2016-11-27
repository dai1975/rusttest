use std;
use std::str::FromStr;
//use std::io::Read;
use futures::{ self, Future };
use futures::stream::Stream;
use tokio_core;

pub fn main() -> Result<(), String> {
   let addr = try!(std::net::SocketAddr::from_str("127.0.0.1:10000").map_err(|e| format!("parse: {}", e)));

   let mut core = tokio_core::reactor::Core::new().unwrap();
   let handle = core.handle();
   
   let listener = tokio_core::net::TcpListener::bind(&addr, &handle).unwrap();

   let r = listener.incoming().for_each(move |(conn,_addr)| {
      let buf  = vec![0u8; 1024]; // tokio_core::io::read requires AsMut
      let iter = futures::stream::iter(std::iter::repeat(()).map(Ok::<(), std::io::Error>));
      let f = iter.fold((conn,buf), |(conn,buf), _| { // use iter and fold to move ownership of buf to closure
         tokio_core::io::read(conn, buf).and_then(|(r,t,size)| {
            if size == 0 {
               println!("closed");
               Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "closed"))
            } else {
               println!("recv[{}]: {}", size, std::str::from_utf8(&t[0..size]).unwrap());
               Ok((r,t))
            }
         })
      });
      handle.spawn(f.then(|_| Ok(())));
      Ok(())
   });
   core.run(r).unwrap();
   Ok(())
}




