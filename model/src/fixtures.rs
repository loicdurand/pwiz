use std::process;

use dotenvy::dotenv;
use std::env;

use polodb_core::{bson::doc, Collection, CollectionT, Database};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// struct Tuto {
//     id: i8,
//     content: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Tag {
//     tuto_id: i8,
//     value: String,
// }

use crate::model::model;
use model::Tag;
use model::Tuto;

pub fn up() {
    dotenv().ok(); //charge les variables présente dans le .env dans l'environnement

    let db_path = env::var("DB_PATH") //on tente de récuperer le chemin de la BDD depuis l'environnement
        .expect("DB_PATH doit etre précisé dans .env"); //si elle n'existe pas on lève une erreur

    let db = Database::open_path(&db_path).unwrap();
    db.create_collection("tutos").unwrap();
    db.create_collection("tags").unwrap();

    let tutos = db.collection("tutos");
    let tags: Collection<Tag> = db.collection("tags");

    tutos
        .insert_many([
            Tuto {
                id: 1,
                content: String::from("sudo cryptsetup luksFormat  /dev/hdXX"),
            },
            Tuto {
                id: 2,
                content: String::from("sudo mkfs.vfat /dev/sdXX"),
            },
        ])
        .unwrap();

    tags.insert_many(["chiffrer", "disque", "dur", "partition"].map(|value| Tag {
        tuto_id: 1,
        value: String::from(value),
    }))
    .unwrap();

    tags.insert_many(["formater", "partition", "fat32"].map(|value| Tag {
        tuto_id: 2,
        value: String::from(value),
    }))
    .unwrap();

    // tags.insert_many(vec![
    //     Tag {
    //         tuto_id: 1,
    //         value: String::from("chiffrer"),
    //     },
    //     Tag {
    //         tuto_id: 1,
    //         value: String::from("disque"),
    //     },
    //     Tag {
    //         tuto_id: 1,
    //         value: String::from("dur"),
    //     },
    //     Tag {
    //         tuto_id: 1,
    //         value: String::from("partition"),
    //     },
    // ])
    // .unwrap();

    let tuto = tutos.find_one(doc! {"id":1});

    println!("name: {:?}", tuto);

    process::exit(1);

    // let tuto_tags = tags
    //     .find(doc! {
    //       "tuto_id": tuto.id
    //     })
    //     .run();
    // for &tag in tuto_tags.iter() {
    //     println!("{:#?}", &tag);
    // }
}
