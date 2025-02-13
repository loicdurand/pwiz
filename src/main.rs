use model::{
    appliquer_reglages, delete_tuto, get_resultats, get_tuto, insert_tuto, prepare_query_from,
    update_tuto,
};
use prompts::{invite, menu_principal, rendu};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{env, error::Error, io, process, thread};
use utils::pluralize;

fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new([SIGINT])?;

    thread::spawn(move || {
        for _ in signals.forever() {
            println!("Bye!");
            process::exit(0);
        }
    });

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let args_len: usize = args.len();

    // Tags passés en arguments -> lance la recherche et affiche les résultats
    if args_len > 0 {
        afficher_tutos(args);
    } else {
        // Aucun argument -> lance le menu:
        lancer_menu();
    }

    Ok(())
}

fn lancer_menu() -> () {
    let num = menu_principal::new();

    let actions: Vec<&dyn Fn(&[String]) -> ()> = vec![
        &afficher_tutos,       // affiche tous les tutos
        &afficher_tutos_invit, // invite à saisir les tags à rechercher
        &creer_tuto,
        &modifier_tuto,
        &supprimer_tuto,
    ];

    match num {
        0..5 => actions[num](&[]),
        _ => process::exit(1),
    }
}

fn afficher_tutos(args: &[String]) -> () {
    let cmd_args: Vec<&String> = args
        .iter()
        .filter(|term| term.starts_with("-"))
        .collect::<Vec<_>>();
    if cmd_args.len() > 0 {
        appliquer_reglages(cmd_args);
    }

    let query = prepare_query_from(args);
    let resultats = get_resultats(query);
    let mut indexes: Vec<i32> = Vec::new();
    let mut shorted: i32 = -1;

    println!(
        "{}",
        pluralize(resultats.len(), "résultat trouvé", "résultats trouvés")
    );
    for resultat in resultats {
        indexes.push(resultat.tuto_id);
        let i = rendu::afficher_resultat_simple(args.len(), resultat);
        if i != -1 {
            shorted = i;
        }
    }
    if shorted != -1 {
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Échec de la lecture de l'entrée utilisateur");

        if let Ok(id) = index.trim().parse::<i32>() {
            if indexes.contains(&id) {
                let infos = get_tuto(id);
                rendu::afficher_recap(infos);
            }
        }
    }
}

fn afficher_tutos_invit(_: &[String]) -> () {
    let tags = invite::demander_tags();
    if tags.len() > 0 {
        afficher_tutos(&tags);
    } else {
        lancer_menu();
    }
}

fn creer_tuto(_: &[String]) -> () {
    let recap = invite::demander_infos_tuto();
    if &recap.title != "" {
        let confirm = rendu::afficher_recap_table(&recap);
        if confirm == "Y" {
            let inserted_id = insert_tuto(recap);
            println!("Le tutoriel a été inséré avec l'id: [{}]", inserted_id);
        } else {
            process::exit(1);
        }
    } else {
        lancer_menu();
    };
}

fn modifier_tuto(_: &[String]) -> () {
    let query = prepare_query_from(&[]);
    let resultats = get_resultats(query);
    let indexes: Vec<i32> = rendu::afficher_table_des_tutoriels(resultats);
    let id: i32 = invite::demander_index_du_tuto(indexes);

    let infos = get_tuto(id);
    let nouvelles_infos = invite::demander_infos_tuto_a_modifier(id, infos);

    update_tuto(id, &nouvelles_infos);

    rendu::afficher_recap(nouvelles_infos);
}

fn supprimer_tuto(_: &[String]) -> () {
    let query = prepare_query_from(&[]);
    let resultats = get_resultats(query);
    let indexes: Vec<i32> = rendu::afficher_table_des_tutoriels(resultats);
    let id: i32 = invite::demander_index_du_tuto(indexes);

    let infos = get_tuto(id);

    let confirm = invite::confirmer_suppression(infos);
    if confirm == "Y" {
        delete_tuto(id);
        println!("Le tutoriel ayant pour id: [{}] a été supprimé.", id);
    } else {
        process::exit(1);
    }
}
