pub fn basic_traits() {
  let name = String::from("Bird")  ;
  let bird = Bird {name, attack: 5};
  bird.print_name();
  println!("{}")
}

struct Bird {
    name: String,
    attack: u64,
}


impl Bird {
    fn print_name(&self) {
        println!("{self.name}");
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        true
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}