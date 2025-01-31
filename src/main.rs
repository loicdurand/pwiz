use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Catégorie à utiliser pour limiter la recherche à un domaine spécifique - ex: "informatique" ou juste "inf", "téléphonie", "radio", etc...
    #[arg(short, long)]
    categorie: Option<String>,

    /// Flags pour la recherche - ex: "dovecot", "messagerie", "pulsar", etc...
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub flags: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for flag in args.flags.iter() {
        println!("Flag: {}!", flag);
    }
}
