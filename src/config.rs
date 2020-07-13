use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(short, long, default_value = "0.0.0.0")]
    pub host: String,
    #[structopt(short, long, default_value = "5000")]
    pub port: u16,
}

impl Config {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
