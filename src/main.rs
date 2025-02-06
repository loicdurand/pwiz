use inquire::Text;
use std::{env, process};

use colored::Colorize;
use comfy_table::*;
use model::{get_resultats, insert_tuto, prepare_query_from, Recap};

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
        println!("tags: {:?}", &tags);
        afficher_tutos_lignes(&tags);
    } else {
        lancer_menu();
    };
}

fn creer_tuto(_: &[String]) -> () {
    if let Ok(title) = Text::new("Quel sera le titre de votre tutoriel?").prompt() {
        if let Ok(content) = Text::new("Quel sera le contenu de votre tutoriel?").prompt() {
            if let Ok(tags) =
                Text::new("Indiquez les tags permettant de rechercher ce tutoriel:").prompt()
            {
                let tags: Vec<String> = tags.trim().split_whitespace().map(String::from).collect();
                let recap = Recap {
                    title,
                    content,
                    tags,
                };
                afficher_recap_table(recap);
            }
        }
    } else {
        lancer_menu();
    };
}

fn afficher_recap_table(recap: Recap) -> () {
    let mut table = Table::new();
    table
        .add_row(vec![
            Cell::new("Titre"),
            Cell::new(&recap.title).add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Contenu"),
            Cell::new(&recap.content)
                .add_attribute(Attribute::Bold)
                .fg(Color::Blue),
        ])
        .add_row(vec!["Tags", &recap.tags.join(", ")]);
    println!("{table}\nC'est bon pour vous? [Y/n]");
    let confirm = Text::new("").prompt().expect("  ");
    if confirm == "Y" {
        if insert_tuto(recap) {
            println!("Le tutoriel a été inséré.");
        } else {
            println!("Une erreur est survenue lors de l'insertion du tutorial. On recommence");
            creer_tuto(&[]);
        }
    } else {
        afficher_recap_table(recap);
    };
}

fn lancer_menu() -> () {
    let mut table = Table::new();
    table
        .set_header(vec!["Choix", "Action"])
        .add_row(vec!["0", "Afficher tous les tutoriels"])
        .add_row(vec!["1", "Rechercher des tutoriels"])
        .add_row(vec!["2", "Créer un tutoriel"]);

    println!("Aucun argument passé, lancement du menu:\n{table}");

    let num = Text::new("Que souhaitez-vous faire? [0, 1, 2]")
        .prompt()
        .expect("Vous devez saisir un chiffre!");
    let cmd_index: usize = num
        .parse::<usize>()
        .expect("Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.");
    let actions: Vec<&dyn Fn(&[String]) -> ()> = vec![
        &afficher_tutos_lignes, // affiche tous les tutos
        &afficher_tutos_invit,  // invite à saisir les tags à rechercher
        &creer_tuto,
    ];

    match cmd_index {
        0..3 => actions[cmd_index](&[]),
        _ => process::exit(1),
    }
}
