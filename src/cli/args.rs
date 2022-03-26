use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub input_file: Option<String>,

    #[clap(short, long, default_value_t = String::from("./library.json"), env="BOOK_OUTPUT")]
    pub output_file: String,

    #[clap(short, long, default_value_t = String::from("~/.books/config.json"), env="BOOK_CONFIG")]
    pub config_file: String,

    pub isbn_list: Vec<String>,
}
