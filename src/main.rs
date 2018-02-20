
extern crate semver;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod filter_versions;

#[derive(StructOpt)]
#[structopt(name = "semver-cli", about = "Prints valid versions sorted by SemVer precedence")]
struct Opt {
    #[structopt(short = "r", long = "range")]
    range: Option<String>,
    #[structopt(name = "VERSION")]
    versions: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    match filter_versions::filter_and_sort(opt.versions, opt.range) {
        Err(err) => {
            eprintln!("Error parsing given range: {0}", &err);
            std::process::exit(1);
        },
        Ok(versions) => {
            for v in versions.iter() {
                println!("{}", &v);
            }
            std::process::exit(0);
        },
    }
}
