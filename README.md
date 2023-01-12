This is a Rust implementation of Quine-McCluskey boolean function optimization algorithm.
It takes a truth table and produces a (possibly) minimal expression of its variables using AND, OR and NOT logic gates.

### Building
Due to its dependencies, this projects demands to be compiled using nightly channel, which must be installed.
Just run `cargo +nightly build --release` to generate the program executable at directory `./target/release`.

### Usage
Run `quine-mccluskey <path>` to execute the optimization over the specified truth table. Note that this table must:
- have its inputs written as 1s and 0s or Ts and Fs;
- list them from the least significant line to the most significant one;
- be formatted as csv.

Check examples at `example_tables/`.

You can also provide the following arguments:
- `--help` or `-h` to print a help message;
- `--step-by-step` to pause the execution at the end of each step, prompting the user to press any key to move on;
- `--dump <path>` to write the resulting formula in a txt file.
