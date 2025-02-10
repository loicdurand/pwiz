pub mod service {

    use polodb_core::{bson, bson::doc, Collection, CollectionT, Database};
    use std::process;

    use crate::fixtures;
    use crate::Recap;
    use crate::Resultat;
    use crate::{Id, Tag, Tuto};

    fn establish_connection() -> Database {
        let db_path = "./pwiz.db"; // chemin de la BDD
        let db = Database::open_path(&db_path).unwrap();
        let ids: Collection<Id> = db.collection("id");

        // Si aucun id en base, on insère un "Hello, World"
        match ids.find_one(doc! {}).unwrap() {
            Some(_) => return db,
            None => fixtures::up(&db),
        }

        return db;
    }

    pub fn prepare_query_from(args: &[String]) -> bson::Document {
        let criterias = args
            .into_iter()
            .map(|term| doc! {"value":term })
            .collect::<Vec<_>>();
        doc! {
            "$or": criterias
        }
    }

    pub fn get_resultats(query: bson::Document) -> Vec<Resultat> {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let mut resultats: Vec<Resultat> = Vec::new(); // Vec qui contiendra nos résultats de recherche

        let tags_result = tags.find(query).run();
        match tags_result {
            Ok(tags) => {
                //
                for tag in tags {
                    match tag {
                        Ok(tag) => {
                            //
                            let tuto_result = tutos
                                .find_one(doc! {"id": {"$eq": tag.tuto_id as i32} })
                                .unwrap();
                            match tuto_result {
                                Some(tuto) => {
                                    //
                                    let index =
                                        resultats[0..].iter().position(|x| x.tuto_id == tuto.id);

                                    if let None = index {
                                        let res = Resultat {
                                            score: 1,
                                            tuto_id: tuto.id,
                                            tags: vec![tag.value],
                                            title: tuto.title,
                                            content_type: tuto.content_type,
                                            content: tuto.content,
                                        };
                                        resultats.push(res);
                                    } else if let Some(index) = index {
                                        resultats[index].score += 1;
                                        resultats[index].tags.push(tag.value);
                                    }
                                }
                                None => println!("Aucun résultat n'a pu etre trouvé"),
                            }
                        }
                        Err(e) => {
                            println!(
                                "Erreur survenue lors de la recherche de tags dans la BDD: {:?}",
                                e
                            );
                        }
                    }
                }

                // classement des résultats par score (nombre de tags trouvés)
                resultats.sort_by(|a, b| b.score.cmp(&a.score));
                return resultats;
            }
            Err(_) => resultats,
        }
    }

    pub fn get_tuto(tuto_id: i32) -> Recap {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let tuto = tutos
            .find_one(doc! {
            "id": tuto_id as i32 })
            .unwrap();

        match tuto {
            Some(tuto) => {
                let mut recap = Recap {
                    title: tuto.title,
                    content_type: tuto.content_type,
                    content: tuto.content,
                    tags: Vec::new(),
                };
                let tags = tags
                    .find(doc! {
                    "tuto_id": tuto_id as i32 })
                    .run();
                match tags {
                    Ok(tags) => {
                        for tag in tags {
                            if let Ok(tag) = tag {
                                recap.tags.push(tag.value);
                            } else {
                                continue;
                            }
                        }
                    }
                    Err(_) => println!(
                        "Une erreur est survenue lors de la récupération des tags de ce tutoriel"
                    ),
                }
                recap
            }
            None => {
                println!("Aucun tuto trouvé");
                process::exit(1);
            }
        }
    }

    pub fn insert_tuto(recap: Recap) -> i32 {
        let db: Database = establish_connection();
        let ids: Collection<Id> = db.collection("id");
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let id = ids.find_one(doc! {}).unwrap();
        let tuto_id: i32;
        match id {
            Some(id) => {
                tuto_id = &id.value + 1;
                ids.update_one(
                    doc! {"value":&id.value},
                    doc! {
                        "$set":{
                            "value":&tuto_id,
                        }
                    },
                )
                .unwrap();
            }
            None => {
                tuto_id = 1;
                ids.insert_one(Id { value: tuto_id }).unwrap();
            }
        }

        if let Ok(_) = tutos.insert_one(Tuto {
            id: tuto_id,
            title: recap.title,
            content_type:recap.content_type,
            content: recap.content,
        }) {
            let docs = recap
                .tags
                .into_iter()
                .map(|term| Tag {
                    tuto_id,
                    value: term,
                })
                .collect::<Vec<_>>();

            if let Ok(_) = tags.insert_many(docs) {
                tuto_id
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn update_tuto(id: i32, recap: &Recap) -> () {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        tags.delete_many(doc! {"tuto_id":{"$eq":id}}).unwrap();
        //
        let updated = tutos.update_one(
            doc! {
                "id":id.to_owned() as i32
            },
            doc! {
                "$set":{
                    "title":&recap.title,
                    "content":&recap.content
                }
            },
        );
        match updated {
            Ok(_) => println!("\nTutoriel ayant pour index [{}] mis à jour:", id),
            Err(e) => println!("Erreur: {}", e),
        }

        tags.insert_many(
            recap
                .tags
                .iter()
                .filter(|term| term.to_string().cmp(&"".to_string()).is_ne())
                .map(|term| Tag {
                    tuto_id: id,
                    value: term.clone(),
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }

    pub fn delete_tuto(id: i32) {
        let db: Database = establish_connection();
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        tags.delete_many(doc! {"tuto_id":{"$eq":id}}).unwrap();
        tutos.delete_one(doc! {"id":{"$eq":id}}).unwrap();
    }
}
