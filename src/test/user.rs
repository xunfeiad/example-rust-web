// author: xunfei
// Project: untitled4
// File: user
// Date: 2023/12/24 14:35
use crate::model;

#[cfg(test)]
mod tests {
    use std::fs::read;
    use std::io;
    use super::*;

    #[test]
    fn get_user_by_id() {
        let mut s = String::new();
        let mut id = io::stdin().lock();
        id.read(&mut s).unwrap();
        let s1 = s.parse::<i32>().unwrap();
       let user = model::get_user_by_id(s1);
        assert_eq!(user, 0)
    }
}
