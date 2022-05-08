#![allow(non_snake_case)]
#![allow(dead_code)]

mod InformedSeaching;
mod UninformedSearching;


struct Node <T>{
    state: T,
    self_cost: u16,
    heur_cost: u16,
    nbrs: HashMap
}


pub fn runner() {
    println!("Goodbye, Mars");
}
