pub mod menu_principal {

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

        let num = Text::new("Que souhaitez-vous faire? [0, 1, 2, 3, 4]")
            .prompt()
            .expect("Vous devez saisir un chiffre!")
            .parse::<usize>()
            .expect(
                "Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.",
            );

        num
    }
}
