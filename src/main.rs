use inquire::Text;
use std::{env, process};

use colored::Colorize;
use comfy_table::Table;
use model::{get_resultats, prepare_query_from};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let args_len: usize = args.len();

    // Tags passés en arguments -> lance la recherche et affiche les résultats
    if args_len > 0 {
        afficher_tutos_lignes(args);
    } else {
        // Aucun argument -> lance le menu:
        lancer_menu();
    }
}

fn afficher_tutos_lignes(args: &[String]) -> () {
    let args_length = args.len();
    let query = prepare_query_from(args);

    let resultats = get_resultats(query);

    println!("{} résultats trouvés: \n", resultats.len());

    for resultat in resultats {

        let score: i8 = if args_length > 0 {
            resultat.score.into()
        } else {
            0
        };
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

fn afficher_tutos_invit(_: &[String]) -> () {
    if let Ok(tags) = Text::new("Saisissez les tags à rechercher:").prompt() {
        let tags: Vec<String> = tags.trim().split_whitespace().map(String::from).collect();
        println!("tags: {:?}",&tags);
        afficher_tutos_lignes(&tags);
    } else {
        lancer_menu();
    };
}

fn lancer_menu() -> () {
    let mut table = Table::new();
    table
        .set_header(vec!["Choix", "Action"])
        .add_row(vec!["0", "Afficher tous les tutoriels"])
        .add_row(vec!["1", "Rechercher des tutoriels"])
        .add_row(vec!["2", "Créer un tutoriel"])
        .add_row(vec!["3", "Mettre à jour un tutoriel"]);
    println!("Aucun argument passé, lancement du menu:\n{table}");

    let num = Text::new("Que souhaitez-vous faire? [0, 1, 2, 3]").prompt();

    match num {
        Ok(num) => {
            let actions: Vec<&dyn Fn(&[String]) -> ()> = vec![
                &afficher_tutos_lignes,
                &afficher_tutos_invit,
                &afficher_tutos_lignes,
            ];
            let cmd_index: usize = num.parse::<usize>().expect(
                "Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.",
            );
            match cmd_index {
                0..3 => actions[cmd_index](&[]),
                _ => process::exit(1),
            }
        }
        Err(_) => println!("An error happened when asking for your name, try again later."),
    }
}
