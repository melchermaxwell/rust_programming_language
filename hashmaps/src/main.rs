use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    
    hashmap_insert_entry();
}

fn hashmap_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map =HashMap::new();
    map.insert(field_name, field_value);
    //field_name and field_value are invalid at this point, try using them and see what compiler error

    //println!("Field name is {}", field_name);
    //error value borrowed here after move
}

fn hashmap_access(scores :HashMap<String, u8>) {

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
}

fn hashmap_iterate(scores :HashMap<String, u8>){

    for(key,value) in scores {
        println!("{}, {}", key, value);
    }
    
}

fn hashmap_insert(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Green"),11);
    scores.insert(String::from("Green"),21);

    println!("{:?}", scores);
}

fn hashmap_insert_entry(){

    //using entry to only insert if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
