pub mod rendu {

    use colored::Colorize;
    use comfy_table::*;
    use inquire::Text;
    use model::{Recap, Resultat};

    pub fn afficher_resultat(nb_tags_demandes: usize, resultat: Resultat) -> () {
        let score: i8 = if nb_tags_demandes > 0 {
            resultat.score.into()
        } else {
            0
        };
        let tags = resultat.tags.join(", ");
        let ligne_des_tags = format!("{score}/{nb_tags_demandes} tags trouvés: {tags}");

        println!(
            "{}\n{}\n>>> {}\n",
            ligne_des_tags,
            resultat.title.bold(),
            resultat.content.bold().blue()
        );
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
            table.add_row(vec![
                Cell::new(&resultat.tuto_id),
                Cell::new(format!(
                    "Titre: {}\nContenu: {}\nTags: {}",
                    &resultat.title.bold(),
                    &resultat.content.bold().blue(),
                    &resultat.tags.join(", ")
                )),
            ]);
        }
        println!("{}\n", table);

        indexes
    }

    pub fn afficher_recap(recap: Recap) -> () {
        println!(
            "Titre: {}\nContenu: {}\nTags: {}",
            &recap.title.bold(),
            &recap.content.bold().blue(),
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
                Cell::new(&recap.content)
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
