mod utils {

    pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
        if count <= 1 {
            format!("{} {}", count, singular)
        } else {
            format!("{} {}", count, plural)
        }
    }
}

pub use utils::pluralize;