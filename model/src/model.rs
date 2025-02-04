pub mod model {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tuto {
        pub id: i32,
        pub content: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tag {
        pub tuto_id: i32,
        pub value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Resultat {
        pub score: i8, // nombre de tags trouvés pour ce tuto
        pub max_score: usize, // nombre de tags recherchés
        pub tuto_id: i32,
        pub tags: Vec<String>, // tags trouvés parmi les arguments
        pub content: String,
    }

}
