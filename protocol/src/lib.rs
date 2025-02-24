
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    pub id:u8,
    pub temperature: f32,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
