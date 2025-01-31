use clap::Parser;
use polodb_core::Database;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Catégorie à utiliser pour limiter la recherche à un domaine spécifique ("informatique", "téléphonie", "radio", etc...)
    /// - ex: "pwiz -c informatique -f [...]"
    /// Vous pouvez n'écrire que les premières lettres du terme recherché, par exemple "inf" pour "informatique", "telep" pur "téléphonie"
    #[arg(short, long, verbatim_doc_comment)]
    categorie: Option<String>,

    /// Tags pour la recherche - ex: "dovecot", "messagerie", "pulsar", etc...
    /// Vous pouvez mentionner autant de tags que vous le souhaitez:
    /// - ex: pwiz -f messagerie dovecot archives
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ',verbatim_doc_comment)]
    pub tags: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();
    // let db = Database::open_path("pwiz.db").unwrap();

    if let Some(categorie) = args.categorie {
        println!("Categorie: {}", categorie.blue().bold());
    };

    for tag in args.tags.iter() {
        println!("Tag: {}", tag.green().bold());
    }
}
