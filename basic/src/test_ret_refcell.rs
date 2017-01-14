#![cfg(test)]
mod tests {
   use std::cell::RefCell;
   use std::rc::Rc;
   use std::collections::HashMap;
   struct Frame {
      map: RefCell<HashMap<String, Rc<String>>>
   }
   impl Frame {
      pub fn new() -> Self {
         Frame { map: RefCell::new(HashMap::new()) }
      }
      pub fn insert(&mut self, k:&str, v:&str) {
         self.map.borrow_mut().insert(k.to_string(), Rc::new(v.to_string()));
      }
      //pub fn lookup<'a>(&'a self, k:&str) -> Option<Rc<String>> {
      pub fn lookup(&self, k:&str) -> Option<Rc<String>> {
         self.map.borrow().get(k).map(|x| x.clone())
      }
   }
   #[test]
   fn t() {
      println!("refcell:");
      let mut f = Frame::new();
      f.insert("a", "Aclie");
      f.insert("b", "Bob");
      f.insert("c", "Carol");
      println!("  {}: {:?}", "a", f.lookup("a"));
   }
}
