pub mod model {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tuto {
        pub id: i8,
        pub content: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tag {
        pub tuto_id: i8,
        pub value: String,
    }
}
