use dotenvy::dotenv;
use std::env;

use polodb_core::{bson::doc, Collection, CollectionT, Database};
use serde::{Deserialize, Serialize};

//use inquire::Text;
mod fixtures;

#[derive(Debug, Serialize, Deserialize)]
struct Tuto {
    id: i8,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    tuto_id: i8,
    value: String,
}

fn main() {
    let db = establish_connection();
    // let tutos: Collection<Tuto> = db.collection("tutos");
    let tags: Collection<Tag> = db.collection("tags");

    let tags_result = tags.find(doc! {"value":"partition" }).run();
    match tags_result {
        Ok(tags) => {
            for tag in tags {
                match tag {
                    Ok(tag) => println!("tags trouvés avec \"partition\": {:?}", tag),
                    Err(e) => println!("Error retrieving tag: {:?}", e),
                }
            }
        }
        Err(_) => (),
    }
    //let tuto = tutos.find_one(doc! {"id":1});
    // println!("tuto: {:?}", tuto);

    // let commande = Text::new("Que souhaitez-vous faire?").prompt();

    // match commande {
    //     Ok(commande) => {}
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }
}

pub fn establish_connection() -> Database {
    dotenv().ok(); //charge les variables présente dans le .env dans l'environnement

    let db_path = env::var("DB_PATH") //on tente de récuperer le chemin de la BDD depuis l'environnement
        .expect("DB_PATH doit etre précisé dans .env"); //si elle n'existe pas on lève une erreur
    let load_fixtures = env::var("LOAD_FIXTURES") //on vérifie s'il faut lancer les fixtures
        .expect("LOAD_FIXTURES doit etre précisé dans .env"); //si elle n'existe pas on lève une erreur

    let db = Database::open_path(&db_path).unwrap();

    if load_fixtures.parse::<i8>().unwrap() == 1 {
        fixtures::up();
    }

    return db;
}
