use rand::{rng, Rng};

pub fn flip_coin() -> bool {
    let num = rng().random_range(2..=20);
    let mut is_head = false;

    if is_prime(num) {
        is_head = true;
    }

    is_head
}

pub fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..=(num as f32).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }

    true
}
