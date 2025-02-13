pub mod rendu {

    use colored::Colorize;
    use comfy_table::*;
    use inquire::Text;
    use model::{Recap, Resultat};
    use utils::pluralize;

    fn ligne_des_tags(nb_tags_demandes: usize, resultat: &Resultat) -> String {
        let score: i8 = if nb_tags_demandes > 0 {
            resultat.score.into()
        } else {
            0
        };
        let tags = resultat.tags.join(", ");
        let x_tags_trouves = pluralize(nb_tags_demandes, "tag trouvé", "tags trouvés");
        let ligne_des_tags = format!("{score}/{x_tags_trouves}: {tags}");

        ligne_des_tags
    }

    pub fn afficher_resultat_simple(nb_tags_demandes: usize, resultat: Resultat) -> i32 {
        let mut shorted_index: i32 = -1;
        let ligne_des_tags = ligne_des_tags(nb_tags_demandes, &resultat);
        let shorter = if resultat.content.len() > 8 {
            let mut tmp = resultat.content[..4].to_vec();
            tmp.push(format!(
                "... -> [{}] pour afficher la suite",
                &resultat.tuto_id
            ));
            shorted_index = resultat.tuto_id;
            tmp
        } else {
            resultat.content.clone()
        };
        println!(
            "{}\nAuteur: {}\n{}\n{}\n",
            ligne_des_tags,
            resultat.author,
            resultat.title.bold(),
            shorter.join("\n").bold().blue()
        );

        shorted_index
    }

    pub fn afficher_table_des_tutoriels(mut resultats: Vec<Resultat>) -> Vec<i32> {
        let mut indexes = Vec::new();
        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("Index").set_alignment(CellAlignment::Center),
            Cell::new("Contenu").set_alignment(CellAlignment::Center),
        ]);
        resultats.sort_by(|a, b| a.tuto_id.cmp(&b.tuto_id));
        for resultat in resultats {
            indexes.push(resultat.tuto_id);
            let content = if resultat.content.len() > 8 {
                let mut tmp = resultat.content[..4].to_vec();
                tmp.push(String::from("..."));
                tmp
            } else {
                resultat.content.clone()
            };

            table.add_row(vec![
                Cell::new(&resultat.tuto_id),
                Cell::new(format!(
                    "Auteur: {}\nTitre: {}\nContenu:\n{}\nTags: {}",
                    &resultat.author,
                    &resultat.title.bold(),
                    &content.join("\n").bold().blue(),
                    &resultat.tags.join(", ")
                )),
            ]);
        }
        println!("{}\n", table);

        indexes
    }

    pub fn afficher_recap(recap: Recap) -> () {
        println!(
            "Titre: {}\nContenu: \n{}\nTags: {}",
            &recap.title.bold(),
            &recap.content.join("\n").bold().blue(),
            &recap.tags.join(", ")
        );
    }

    pub fn afficher_recap_table_raw(recap: &Recap) -> Table {
        let mut table = Table::new();
        table
            .add_row(vec![
                Cell::new("Titre"),
                Cell::new(&recap.title).add_attribute(Attribute::Bold),
            ])
            .add_row(vec![
                Cell::new("Contenu"),
                Cell::new(&recap.content.join("\n"))
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Blue),
            ])
            .add_row(vec!["Tags", &recap.tags.join(", ")]);
        table
    }

    pub fn afficher_recap_table(recap: &Recap) -> String {
        let table = afficher_recap_table_raw(&recap);
        println!("{table}\nC'est bon pour vous? [Y/n]");
        let confirm = Text::new("").prompt().expect("Saisissez une lettre [Y/n]");
        confirm
    }
}
