use std::env;

mod net_test;

extern crate mio;
mod mio_test;

extern crate mioco;
mod mioco_test;

extern crate futures;
extern crate tokio_core;
mod tokio_test;

extern crate coio;
mod coio_test;

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
      println!("{} <commands>", args[0]);
      println!("commands are: net, mio, mioco, tokio, coio");
   }

   let r = match args[1].as_ref() {
      "net"   => net_test::main(),
      "mio"   => mio_test::main(),
      "mioco" => mioco_test::main(),
      "tokio" => tokio_test::main(),
      "coio"  => coio_test::main(),
      _ => Err(format!("unknown test: {}", args[1]))
   };
   let _ = r.map_err(|s|println!("{}",s));
}
