pub mod model {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Id {
        pub value: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tuto {
        pub id: i32,
        pub title: String,
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
        pub tuto_id: i32,
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

    impl Recap {
        pub fn new(tuto: Tuto) -> Recap {
            Recap {
                title: tuto.title,
                content: tuto.content,
                tags: Vec::new(),
            }
        }
        pub fn default() -> Recap {
            Recap {
                title: String::from(""),
                content: String::from(""),
                tags: Vec::new(),
            }
        }
    }
}
