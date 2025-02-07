pub mod service {

    use dotenvy::dotenv;
    use polodb_core::{bson, bson::doc, Collection, CollectionT, Database};
    use std::env;

    use crate::fixtures;
    use crate::Recap;
    use crate::Resultat;
    use crate::{Id, Tag, Tuto};

    fn establish_connection() -> Database {
        dotenv().ok(); //charge les variables présente dans le .env dans l'environnement

        let db_path = env::var("DB_PATH") //on tente de récuperer le chemin de la BDD depuis l'environnement
            .expect("DB_PATH doit etre précisé dans .env"); //si elle n'existe pas on lève une erreur
        let load_fixtures = env::var("LOAD_FIXTURES") //on vérifie s'il faut lancer les fixtures
            .expect("LOAD_FIXTURES doit etre précisé dans .env"); //si elle n'existe pas on lève une erreur

        if load_fixtures.parse::<i8>().unwrap() == 1 {
            // Attention! Le programme quittera après exécution des fixtures: -> remettre LOAD_FIXTURES=0
            fixtures::up();
        }

        let db = Database::open_path(&db_path).unwrap();

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

    pub fn insert_tuto(recap: Recap) -> bool {
        let db: Database = establish_connection();
        let ids: Collection<Id> = db.collection("id");
        let tutos: Collection<Tuto> = db.collection("tutos");
        let tags: Collection<Tag> = db.collection("tags");

        let id = ids.find_one(doc! {}).unwrap();
        match id {
            Some(id) => {

                let id = id.value + 1;
                
                if let Ok(_) = tutos.insert_one(Tuto {
                    id,
                    title: recap.title,
                    content: recap.content,
                }) {
                    let docs = recap
                        .tags
                        .into_iter()
                        .map(|term| Tag {
                            tuto_id: id,
                            value: term,
                        })
                        .collect::<Vec<_>>();

                    if let Ok(_) = tags.insert_many(docs) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            None => false
        }
    }
}
