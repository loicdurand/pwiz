pub mod invite {

    use super::super::rendus::rendu::afficher_recap_table_raw;
    use colored::Colorize;
    use inquire::Text;
    use model::{Recap, Tuto};

    pub fn demander_tags() -> Vec<String> {
        if let Ok(tags) = Text::new("Saisissez les tags à rechercher:").prompt() {
            let tags: Vec<String> = tags.trim().split_whitespace().map(String::from).collect();
            return tags;
        } else {
            // Aucun tag saisi par l'utilisateur -> retour menu
            return Vec::new();
        };
    }

    pub fn demander_infos_tuto() -> Recap {
        let abort = Recap::new();
        let mut content = Vec::new();
        let mut i: usize = 0;
        if let Ok(title) = Text::new("Quel sera le titre de votre tutoriel?").prompt() {
            loop {
                let line = Text::new("Contenu du tutoriel: [:q] pour terminer")
                    .prompt()
                    .expect("Contenu non valable");
                if line == ":q" {
                    println!("Terminé!");
                    break;
                } else {
                    println!("Ligne ajoutée: {}", line);
                    content.push(line);
                }
                i = &i + 1;
            }

            if let Ok(tags) =
                Text::new("Indiquez les tags permettant de rechercher ce tutoriel:").prompt()
            {
                let tags = tags.trim().split_whitespace().map(String::from).collect();
                return Recap {
                    title,
                    content_type: String::from("command"),
                    content,
                    tags,
                };
            }

            return abort;
        } else {
            return abort;
        };
    }

    pub fn demander_index_du_tuto(indexes: Vec<i32>) -> i32 {
        return Text::new(&format!(
            "Saisissez l'index sur lequel vous souhaitez agir: [{}]",
            indexes
                .into_iter()
                .map(|index| index.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ))
        .prompt()
        .expect("Vous devez saisir un chiffre!")
        .parse::<i32>()
        .expect(
            "Veuillez entrer un chiffre correspondant à l'action que vous souhaitez exécuter.",
        );
    }

    pub fn demander_infos_tuto_a_modifier(id: i32, recap: Recap) -> Recap {
        let mut title = Text::new(&format!(
            "Saisissez le nouveau titre de ce tutoriel [{}]",
            &recap.title
        ))
        .prompt()
        .expect("Titre non valable");
        if title == "" {
            title = recap.title;
        }
        //
        let content_type = String::from("command");
        let mut content = Vec::new();
        let mut i: usize = 0;
        loop {
            if i == recap.content.len() {
                break;
            }
            println!("Ligne {}: >>> {}", i, recap.content[i].green());
            let line = Text::new("Contenu du tutoriel: [:q] pour terminer, [-] pour supprimer")
                .prompt()
                .expect("Contenu non valable");
            if line == ":q" {
                println!("Terminé!");
                break;
            } else if line == "" {
                println!("Ligne inchangée");
                content.push(recap.content[i].to_owned());
            } else if line != "-" {
                println!("Ligne modifiée: {}", &line);
                content.push(String::from(line));
            } else {
                println!("Ligne supprimée");
            }
            i = &i + 1;
        }
        let mut mod_recap = Recap::default(Tuto {
            id,
            title,
            content_type,
            content,
        });

        for recap_tag in recap.tags {
            let tag = Text::new(&format!(
                "Modifier ce tag? [{}]\nNota: pour le supprimer, indiquez {}",
                &recap_tag,
                String::from('-').bold().red()
            ))
            .prompt()
            .expect("Contenu non valable");
            if tag == "" {
                println!("Tag inchangé");
                mod_recap.tags.push(recap_tag);
            } else if tag != "-" {
                println!("Tag modifié: {}", tag);
                mod_recap.tags.push(tag);
            } else {
                println!("Tag supprimé");
            }
        }

        loop {
            let tag = Text::new("Ajouter un tag supplémentaire? Nota: laisser vide pour terminer")
                .prompt()
                .expect("Contenu non valable");
            if tag == "" {
                println!("Terminé");
                break;
            } else {
                mod_recap.tags.push(tag);
            }
        }

        mod_recap
    }

    pub fn confirmer_suppression(recap: Recap) -> String {
        let table = afficher_recap_table_raw(&recap);
        let confirm = Text::new(&format!(
            "{}\nConfirmez-vous la suppression de ce tutoriel? [Y/n]",
            table
        ))
        .prompt()
        .expect("Saisissez une lettre: [Y/n]");
        confirm
    }
}
