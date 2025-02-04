use dotenvy::dotenv;
use std::env;

use polodb_core::{bson, bson::doc, Collection, CollectionT, Database};

//use inquire::Text;
use model::fixtures;
use model::Resultat;
use model::Tag;
use model::Tuto;

fn main() {
    let db: Database = establish_connection();
    let tutos: Collection<Tuto> = db.collection("tutos");
    let tags: Collection<Tag> = db.collection("tags");

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let max_score: usize = args.len();
    let mut resultats: Vec<Resultat> = Vec::new(); // Vec qui contiendra nos résultats de recherche

    let query = prepare_query_from(args);

    let tags_result = tags.find(query).run();
    match tags_result {
        Ok(tags) => {
            //
            for tag in tags {
                match tag {
                    Ok(tag) => {
                        //
                        println!("tag trouvé: {:?}", tag);
                        //
                        let tuto_result =
                            tutos.find_one(doc! {"id": {"$eq":tag.tuto_id} }).unwrap();
                        match tuto_result {
                            Some(tuto) => {
                                //
                                println!("==> tuto trouvé pour le tag ci-dessus: {:?}", tuto);
                                //
                                let index =
                                    resultats[0..].iter().position(|x| x.tuto_id == tuto.id);

                                if let None = index {
                                    let res = Resultat {
                                        score: 1,
                                        max_score,
                                        tuto_id: tuto.id,
                                        tags: vec![tag.value],
                                        titre: tuto.titre,
                                        content: tuto.content,
                                    };
                                    resultats.push(res);
                                } else if let Some(index) = index {
                                    resultats[index].score += 1;
                                    resultats[index].tags.push(tag.value);
                                }
                            }
                            None => println!("Aucun tuto n'a été trouvé"),
                        }
                    }
                    Err(e) => println!(
                        "Erreur survenue lors de la recherche de tags dans la BDD: {:?}",
                        e
                    ),
                }
            }

            // classement des résultats par score (nombre de tags trouvés)
            resultats.sort_by(|a, b| b.score.cmp(&a.score));
            //
            println!("Résultats trouvés: {:?}", resultats);
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

    if load_fixtures.parse::<i8>().unwrap() == 1 {
        // Attention! Le programme quittera après exécution des fixtures: -> remettre LOAD_FIXTURES=0
        fixtures::up();
    }

    let db = Database::open_path(&db_path).unwrap();

    return db;
}

fn prepare_query_from(args: &[String]) -> bson::Document {
    let criterias = args
        .into_iter()
        .map(|term| doc! {"value":term })
        .collect::<Vec<_>>();
    doc! {
        "$or": criterias
    }
}
