use {
  regex::Regex,
  std::{collections::HashMap, fs},
};

fn main() {
  let regex = Regex::new(r"^([a-z]{11})(_| ).*\t\d+\t(\d+)\t\d+$").unwrap();

  let files = ('a'..='z')
    .into_iter()
    .map(|c| {
      eprint!("{c}");
      (
        c,
        fs::read_to_string(format!("tmp/googlebooks-eng-all-1gram-20120701-{c}"))
          .unwrap()
          .to_lowercase(),
      )
    })
    .collect::<Vec<(char, String)>>();

  eprintln!();

  {
    let mut occurrences = HashMap::<&str, u64>::new();

    for (c, file) in &files {
      eprint!("{c}");
      for line in file.lines() {
        if let Some(captures) = regex.captures(&line) {
          let word = captures.get(1).unwrap().as_str();
          if word > "nvtdijuwxlp" || word <= "afqsjjpjruv" {
            continue;
          }
          *occurrences.entry(word).or_default() += captures[3].parse::<u64>().unwrap();
        }
      }
    }
    eprintln!();

    let mut frequency = occurrences
      .into_iter()
      .filter_map(|(word, occurrences)| {
        if occurrences >= 5000 {
          Some((occurrences, word))
        } else {
          None
        }
      })
      .collect::<Vec<(u64, &str)>>();

    frequency.sort();

    for (occurrences, word) in frequency {
      println!("{word}\t{occurrences}");
    }
  }
}
