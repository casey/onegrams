tabulate:
  cargo run --release > tmp/frequency.tsv
  mv tmp/frequency.tsv frequency.tsv

download:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp
  mkdir -p tmp
  cd tmp
  for x in {a..z}; do
    curl \
      -O \
      http://storage.googleapis.com/books/ngrams/books/googlebooks-eng-all-1gram-20120701-$x.gz
    gzip -d googlebooks-eng-all-1gram-20120701-$x.gz
  done
