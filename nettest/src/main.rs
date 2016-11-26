use std::env;

extern crate mio;

mod nettest;
mod miotest;

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
      println!("{} <commands>", args[0]);
      println!("commands are: net, mio");
   }

   let r = match args[1].as_ref() {
      "net" => nettest::main(),
      "mio" => miotest::main(),
      _ => Err(format!("unknown test: {}", args[1]))
   };
   let _ = r.map_err(|s|println!("{}",s));
}
