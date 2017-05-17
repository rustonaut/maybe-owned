extern crate maybe_owned;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::SystemTime;

struct Data {
    text: String,
    // this should be some think like
    // chrono::Date, but then it's just an examples
    time: SystemTime
}

impl Data {
    fn new<T>(text: T) -> Data
        where T: Into<String>
    {
        let time = SystemTime::now();
        Data { text, time }
    }
}

#[derive(Default)]
struct Regestry<'a> {
    registry: HashMap<String, MaybeOwned<'a, Data>>
}

impl<'a> Regestry<'a> {

    fn new() -> Regestry {
        Default::default()
    }

    fn register_data<K,D>(&mut self, key: K, data: D)
        where K: Into<String>, D: Into<MaybeOwned<'a, Data>>
    {
        self.registry.insert(key.into(), data.into()'\)
    }

    fn print_me(&self) {
        for (key, val) in self.registry {
            println!(
                "got: {} [{}]",
                //we can just deref MaybeOwned
                val.
            )
        }
    }
}


fn main() {
    let reg = Regestry::new();
    reg.registry("tom", Data::new("abc"));
    let shared_data = Data::new("--missing--");
    reg.registry("lucy", &shared_data);
    reg.registry("peter", &shared_data);
    reg.print_me();
}