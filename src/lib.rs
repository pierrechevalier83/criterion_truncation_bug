const THE_ANSWER: u8 = 32;

pub fn fast() -> u8 {
    THE_ANSWER
}

pub fn slow() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(1));
    THE_ANSWER
}
