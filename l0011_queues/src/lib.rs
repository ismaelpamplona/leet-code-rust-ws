use std::collections::VecDeque;
pub fn play() {
    // Declaration: Rust uses VecDeque from std::collections
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Enqueueing/adding elements:
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    // Dequeing/removing elements:
    queue.pop_front();

    // Check if empty:
    let is_empty = queue.is_empty(); // false

    // Check element at front of queue (next element to be removed):
    let front = *queue.front().unwrap(); // 2

    // Get size:
    let size = queue.len(); // 2

    println!("Is Empty: {}", is_empty);
    println!("Front: {}", front);
    println!("Size: {}", size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_play() {
        play();
    }
}
