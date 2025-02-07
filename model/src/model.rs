pub mod model {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Id {
        pub value: usize,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tuto {
        pub id: usize,
        pub title: String,
        pub content: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tag {
        pub tuto_id: usize,
        pub value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Resultat {
        pub score: i8, // nombre de tags trouvés pour ce tuto
        pub tuto_id: usize,
        pub tags: Vec<String>, // tags trouvés parmi les arguments
        pub title: String,
        pub content: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Recap {
        pub title: String,
        pub content: String,
        pub tags: Vec<String>, // tags trouvés parmi les arguments
    }
}
