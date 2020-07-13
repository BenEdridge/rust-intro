use std::collections::HashMap;

fn main(){
    let mut map = HashMap::new();

    let list = [("hello", "world"), ("goodbye", "world")];

    for (key, val) in list.iter() {
        map.entry(key).or_insert_with(Vec::new).push(val);
    }

    println!("{:?}", map)
}

