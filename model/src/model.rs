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

    // #[derive(Debug, Serialize, Deserialize)]
    // pub struct Resultat {
    //     score: i8, // nombre de tags trouvés pour ce tuto
    //     pub tuto_id: i8,
    //     pub tags: Vec<Tag>,
    //     pub content: String,
    // }

    // impl Resultat {
    //     fn up_score(&mut self) -> i8 {
    //         self.score += 1;
    //         self.score
    //     }
    // }
}
