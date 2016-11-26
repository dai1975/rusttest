struct RefNode<'a> {
   name: String,
   next: Option<&'a mut RefNode<'a>>,
   prev: Option<&'a mut RefNode<'a>>,
}
struct RefList<'a> {
   nodes: Vec<RefNode<'a>>,
}

impl <'a> RefList<'a> {
   fn new<'b>(n:usize) -> RefList<'b> {
      RefList{ nodes: Vec::new() }
   }
   fn build(&'a mut self, n:usize) {
      let nodes = &mut self.nodes;
      for i in 0..n {
         nodes.push(RefNode{
            name: format!("node-{}", i),
            next: None,
            prev: None,
         });
      }

      for i in 0..n {
         let (l,rest) = nodes.split_at_mut(i);
         let (c,r)    = rest.split_at_mut(1);
         if let Some(ref_node) = c.get_mut(0) {
            ref_node.prev = l.get_mut(l.len()-1);
            ref_node.next = r.get_mut(0);
         }
      }
   }
}

pub fn t() {
   println!("test_graph");
   RefList::new(5);
}
