# medium-202310-rust
Git repository to accompany medium post: 

The repository contains the following directory (binary crates):
* `a_naive` contains the source code defining the `Triangle` and `Square` types alongside a straightforward implementation of a draw function;
* `c_trait` contains the implementation of the `draw` functionality using traits and generics;
* `d_declarative_macro` contains the implementation of the `draw` functionality using a declarative macro;
* `e_procedural_macro` contains the implementaiton of the `draw` functionality using a procedural macro - located in `e2_derive_draw`.

Run each example using the following commands:
```
cargo run --bin a_naive # For the code in the `a_naive` directory
cargo run --bin c_trait # For the trait/generic based example
# etc
```

