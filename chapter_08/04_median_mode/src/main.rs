use std::collections::HashMap;
use rand::Rng;

fn main() {
    let median: i32;
    let mut v: Vec<i32> = Vec::new();
    
    for _ in 0..100 {
        v.push(rand::thread_rng().gen_range(-100..100));
    }
    println!("Set of data to be analyze: {:?}", v);

    v.sort();
    let length_vector = v.len();
    
    // If v.len()%2 == 0 then we need the average of n/2 and n/2 + 1
    if length_vector%2 == 0{
        median = (&v[(length_vector/2)-1] + &v[length_vector/2])/2;
    }
    // Else is just n/2
    else{
        median = v[length_vector/2];
    }

    let mut map = HashMap::new();

    // We check every element of the vector and record the amount of times each shows up
    for i in &v {
        let count: &mut i32 = map.entry(i).or_insert(0);
        *count+=1;
    }

    // By finding the max value in the hashmap we know which value to look for to determine the mod
    let mode_value = map.values().max().expect("Vector was empty somehow?");
    let mut v2: Vec<&i32> = Vec::new();

    for (key, value) in map.iter() {
        if value == mode_value {
            v2.push(*key);
        }
    }

    println!("The median of the vector is {median}");
    // println!("{:?}", map);
    println!("The modes of the vector are {:?}", v2);
}