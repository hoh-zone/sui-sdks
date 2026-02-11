use rand::seq::SliceRandom;

pub fn shuffle<T>(items: &mut [T]) {
    let mut rng = rand::thread_rng();
    items.shuffle(&mut rng);
}

pub fn weighted_shuffle<T: Clone>(items: &[(T, u64)]) -> Vec<T> {
    let mut expanded = Vec::new();
    for (item, w) in items {
        for _ in 0..*w {
            expanded.push(item.clone());
        }
    }
    shuffle(&mut expanded);
    expanded
}
