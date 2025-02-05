use inquire::Text;
use std::{env, process};

use colored::Colorize;
use comfy_table::Table;
use model::{get_resultat, prepare_query_from};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let args_len: usize = args.len();

    // Si des tags sont passés en arguments,
    // on lance la recherche
    if args_len > 0 {
        rechercher_tutos(args);
    } else {
        // si commande lancée sans argument,
        // on lance le menu:
        lancer_menu();
    }
}

fn rechercher_tutos(args: &[String]) -> () {
    let args_length = args.len();
    let query = prepare_query_from(args);

    let resultats = get_resultat(query);

    println!("{} résultats trouvés: \n", resultats.len());

    for resultat in resultats {
        //
        let score = if args_length == 0 { 0 } else { resultat.score.into() };
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

fn lancer_menu() -> () {
    let mut table = Table::new();
    table
        .set_header(vec!["Choix", "Action"])
        .add_row(vec!["0", "Afficher tous les tutoriels"])
        .add_row(vec!["1", "Rechercher des tutoriels"])
        .add_row(vec!["2", "Insérer un nouveau tutoriel"])
        .add_row(vec!["3", "Mettre à jour un tutoriel"]);
    println!("Aucun argument passé, lancement du menu:\n{table}");

    let commande = Text::new("Que souhaitez-vous faire? [0, 1, 2, 3]").prompt();

    match commande {
        Ok(commande) => {
            let commandes: Vec<&dyn Fn(&[String]) -> ()> = vec![
                &rechercher_tutos,
                &rechercher_tutos,
                &rechercher_tutos,
                &rechercher_tutos,
            ];
            let cmd_index: usize = commande.parse::<usize>().expect(
                "Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.",
            );
            match cmd_index {
                0..4 => commandes[cmd_index](&[]),
                _ => process::exit(1),
            }
        }
        Err(_) => println!("An error happened when asking for your name, try again later."),
    }
}
