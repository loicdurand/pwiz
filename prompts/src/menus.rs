pub mod menu_principal {

    use std::process;

    use comfy_table::*;
    use inquire::Text;

    pub fn new() -> usize {
        let mut table = Table::new();
        table
            .set_header(vec!["Choix", "Action"])
            .add_row(vec!["0", "Afficher tous les tutoriels"])
            .add_row(vec!["1", "Rechercher des tutoriels"])
            .add_row(vec!["2", "Créer un tutoriel"])
            .add_row(vec!["3", "Modifier un tutoriel"])
            .add_row(vec!["4", "Supprimer un tutoriel"]);

        println!("Aucun argument passé, lancement du menu:\n{table}");
        let cmd_num: usize;

        loop {
            let cmd = Text::new("Que souhaitez-vous faire? [0, 1, 2, 3, 4]\n").prompt();
            match cmd {
                Ok(cmd) => {
                    if cmd.parse::<usize>().is_ok() {
                        if let Ok(num) = cmd.parse::<usize>() {
                            if num <= 4 {
                                cmd_num = num;
                                break;
                            } else {
                                println!("Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.");
                            }
                        } else {
                            println!("Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.");
                        }
                    } else if utils::is_string(&cmd) {
                        println!("Vous devez saisir un chiffre!");
                    } else {
                        process::exit(0);
                    }
                }
                _ => process::exit(0),
            }
        }

        cmd_num
    }
}
