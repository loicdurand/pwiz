use crate::model::model::{Id, Tag, Tuto};
use polodb_core::{bson::doc, Collection, CollectionT, Database};

pub fn up(db: &Database) -> () {
    db.create_collection("id").unwrap();
    db.create_collection("tutos").unwrap();
    db.create_collection("tags").unwrap();

    let ids = db.collection("id");
    let tutos = db.collection("tutos");
    let tags: Collection<Tag> = db.collection("tags");

    let first = 1;

    tutos
        .insert_one(Tuto {
            id: first,
            title: String::from("Saluer le monde!"),
            content: String::from("echo \"Hello, world!\""),
        })
        .unwrap();

    tags.insert_many(
        ["saluer", "bonjour", "monde", "hello", "world"].map(|value| Tag {
            tuto_id: first,
            value: String::from(value),
        }),
    )
    .unwrap();

    ids.insert_one(Id { value: first }).unwrap();

    let sample = tutos.find_one(doc! {"id":first});

    println!("Tuto inséré: {:?}", sample);
}
