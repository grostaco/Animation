use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Mul, sync::Mutex};

lazy_static! {
    static ref CHOOSE_CACHE: Mutex<HashMap<u64, HashMap<u64, u64>>> = Mutex::new(HashMap::new());
}

fn choose_using_cache(n: u64, r: u64) -> u64 {
    let mut cache = CHOOSE_CACHE.lock().unwrap();
    if !cache.contains_key(&n) {
        cache.insert(n, HashMap::new());
    }

    if !cache.get(&n).unwrap().contains_key(&r) {
        let v = choose(n, r, false);
        cache.get_mut(&n).unwrap().insert(r, v);
        v
    } else {
        *cache.get(&n).unwrap().get(&r).unwrap()
    }
}

pub fn choose(n: u64, r: u64, use_cache: bool) -> u64 {
    if use_cache {
        return choose_using_cache(n, r);
    }
    if n < r {
        return 0;
    }
    if r == 0 {
        return 1;
    }

    let denominator = (1..r + 1).fold(1u64, u64::mul);
    let numerator = (n - r + 1..=n).fold(1u64, u64::mul);
    numerator / denominator
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1. + (-x).exp())
}

#[cfg(test)]
mod tests {
    use super::choose;

    #[test]
    fn choose_test() {
        assert!(choose(10, 3, true) == 120);
        assert!(choose(10, 3, true) == 120);
        assert!(choose(5, 4, true) == 5);
    }
}
