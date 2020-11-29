use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ferrisflow")]
pub struct Opt {
    #[structopt(short, long, default_value = "2055")]
    pub port: String,

    #[structopt(long)]
    pub netflow_v5: bool,

    #[structopt(long)]
    pub netflow_v9: bool,

    #[structopt(long)]
    pub print: bool,

    #[structopt(short, long)]
    pub json: bool,

    #[structopt(short, long)]
    pub csv: bool,

    #[structopt(short, long)]
    pub header_none: bool,
}
