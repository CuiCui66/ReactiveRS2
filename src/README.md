## To build the library 

The library has two different options:
* `par`, which enable the parallelism.
* `funsafe`, which improve the performance by using unsafe functions 
  and structures where it is possible.
  In theory, this option maintains memory safety for the user.

To compile the library, use the command `cargo build`, and give the options with
`--features "option1,option2,..."`

To run the game of life example, use the command `cargo run --bin gol`,
with any option flags.

To build the documentation, use the command `cargo doc`.
