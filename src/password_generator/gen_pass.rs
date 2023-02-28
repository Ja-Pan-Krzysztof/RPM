use rand::seq::SliceRandom;
use rand::thread_rng;


fn random_array(array: &mut Vec<char>) -> Vec<char> {
    let mut new_vec = vec![];

    for _ in 0..array.len() {
        let rng_char = array.choose(&mut thread_rng())
            .unwrap();

        new_vec.push(*rng_char);

        let target_index = array.iter()
            .position(|&x| x == *rng_char)
            .unwrap();

        array.remove(target_index);
    }

    new_vec
}


pub fn gen_pass(lenght: u64, letters: &mut Vec<char>) -> String {
    let mut result = String::new();
    let array = random_array(letters);

    for _ in 0..lenght {
        let rng_char = array.choose(&mut thread_rng())
            .unwrap();

        result.push(*rng_char);
    }

    result
}
