mod utils {

    pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
        if count <= 1 {
            format!("{} {}", count, singular)
        } else {
            format!("{} {}", count, plural)
        }
    }

    pub fn is_string(s: &str) -> bool {
        s.chars().all(|c| c.is_alphabetic())
    }
}

pub use utils::*;