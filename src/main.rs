use anyhow::{anyhow, Context, Result};
use argh::FromArgs;
use tallystick::{stv::DefaultTally, Quota};

#[derive(FromArgs)]
#[argh(description = "Run election using STV method")]
struct CmdLineOptions {
    #[argh(positional)]
    #[argh(description = "input csv file containing ballots")]
    csv_filename: String,
}

fn generate_ballots(csv_filename: &str) -> Result<(Vec<String>, Vec<Vec<String>>)> {
    let mut rdr = csv::Reader::from_path(csv_filename)
        .context(format!("Could not open candidate file: {}", csv_filename))?;

    let candidates = rdr
        .records()
        .next()
        .ok_or(anyhow!("could not extract candidates"))??
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let mut bx = vec![];

    for result in rdr.records().skip(1) {
        let record = result?
            .iter()
            .skip(1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut pref = vec![];

        for (k, v) in candidates.iter().zip(record.iter()) {
            pref.push((k.to_string(), v.parse::<i32>()?));
        }

        pref.sort_by_key(|k| k.1);
        let x: Vec<String> = pref.iter().map(|p| p.0.to_string()).collect();
        bx.push(x);
    }

    Ok((candidates, bx))
}

fn main() -> Result<()> {
    let args: CmdLineOptions = argh::from_env();
    let (candidates, ballots) = generate_ballots(&args.csv_filename)?;

    let mut tally = DefaultTally::new(4, Quota::Droop);
    for b in ballots {
        tally.add(b);
    }

    let winners = tally.winners();

    println!("###################");
    println!("Candidates: ");
    println!("-----------");
    for c in candidates {
        println!("- {}", c);
    }
    println!("###################");
    println!("Winners: ");
    println!("--------");

    for w in winners.into_vec() {
        println!("{} (rank {})", w.candidate, w.rank);
    }

    Ok(())
}
