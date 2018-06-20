// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::01d31ed5-faf3-4dc3-8193-149a359ae48e][01d31ed5-faf3-4dc3-8193-149a359ae48e]]
extern crate gchemol;
#[macro_use] extern crate quicli;

use quicli::prelude::*;
use gchemol::formats::vasp::PoscarFile;

#[derive(Debug, StructOpt)]
struct Cli {
    // Add a positional argument that the user has to supply:
    /// The file to read
    file: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbose: Verbosity,
}

main!(|args: Cli, log_level: verbose| {
    info!("reading from: {}", &args.file);
    let txt =  read_file(&args.file)?;
});
// 01d31ed5-faf3-4dc3-8193-149a359ae48e ends here

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::c2b8efc4-fcdb-4c4c-96f2-e5a46a9224c7][c2b8efc4-fcdb-4c4c-96f2-e5a46a9224c7]]
use std::path::{Path, PathBuf};

/// read element symbols from input.dat in the parent directory of pso_opt_* file
fn get_symbols_from_pso_input(pso_result_file: &Path) -> Result<Vec<String>> {
    let parent = pso_result_file.parent().ok_or(format_err!("wrong path"))?;
    let input_dat = parent.join("../input.dat");

    let txt = read_file(input_dat)?;
    let mut line = txt
        .lines()
        .filter(|line| line.starts_with("NameOfAtoms"))
        .take(1);

    if let Some(line) = line.next() {
        let syms: Vec<_> = line.split('=')
            .last()
            .ok_or(format_err!("xx"))?
            .trim()
            .split(' ')
            .map(|s| s.into())
            .collect();
        return Ok(syms)

    } else {
        bail!("cannot found NameOfAtoms in input.dat");
    }
}

#[test]
fn test_pso_input() {
    let p = Path::new("/home/ybyygu/Incoming/research/星辰表面搜索/calypso-dftb/test2/pso_opt_50");
    let x = get_symbols_from_pso_input(p).expect("pso input");
    println!("{:#?}", x);
}
// c2b8efc4-fcdb-4c4c-96f2-e5a46a9224c7 ends here
