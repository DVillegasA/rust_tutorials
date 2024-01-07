use std::collections::HashMap;

fn calc_median(vector: &mut Vec<u32>) -> f32 {
    vector.sort();
    let vector_half_size = vector.len()/2;

    if vector.len() % 2 == 0 {
        return (&vector[vector_half_size - 1] + &vector[vector_half_size]) as f32 / 2.0;
    } else {
        return vector[vector_half_size] as f32;
    }
}

fn calc_modes(vector: Vec<u32>) -> Vec<u32> {
    let mut mode_map = HashMap::new();

    for value in vector {
        let count = mode_map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut current_max_key: Vec<u32> = Vec::new();
    let mut current_max_value = 0;
    for (key, value) in &mode_map {
        if value > &current_max_value {
            current_max_key = vec![*key];
            current_max_value = *value;
        } else if value == &current_max_value {
            current_max_key.push(*key);
        }
    }

    current_max_key.sort();
    current_max_key
}

fn main() {
    let mut values: Vec<u32> = vec![
        161,528,77,62,602,260,93,575,733,735,705,228,322,64,528,999,488,266,
        161,774,512,617,224,982,223,245,480,830,404,399,463,188,200,720,485,
        527,948,863,391,348,924,801,489,757,451,236,986,492,91,791,829,875,
        652,249,47,259,697,298,561,530,63,244,671,38,46,289,465,944,211,574,
        502,455,734,76,284,943,8,576,523,788,139,106,105,628,901,308,281,712,
        757,678,667,57,663,864,154,338,858,900,258,434
    ];

    let median: f32 = calc_median(&mut values);
    let mode: Vec<u32> = calc_modes(values);
    
    println!("The median of the list is: {}",median);
    println!("The mode of the list is: {:?}",mode);
}
