use std::env;

// use comfy_table::Table;
use colored::Colorize;
use model::{get_resultat, prepare_query_from};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let args_len: usize = args.len();

    if args_len > 0 {
        rechercher_tutos(args);
    } else {
        println!("Aucun argument passé");

        //let tuto = tutos.find_one(doc! {"id":1});
        // println!("tuto: {:?}", tuto);

        // let commande = Text::new("Que souhaitez-vous faire?").prompt();

        // match commande {
        //     Ok(commande) => {}
        //     Err(_) => println!("An error happened when asking for your name, try again later."),
        // }
    }
}

fn rechercher_tutos(args: &[String]) -> () {
    let args_length = args.len();
    let query = prepare_query_from(args);

    let resultats = get_resultat(query);

    println!("{} résultats trouvés: \n", resultats.len());

    for resultat in resultats {
        //
        let score = &resultat.score;
        let tags = resultat.tags.join(", ");
        let affichage = format!("{score}/{args_length} tags trouvés: {tags}");

        println!(
            "{}\n{}\n>>> {}\n",
            affichage,
            resultat.title.bold(),
            resultat.content.bold().blue()
        );
    }
}
