// #![allow(dead_code, unused_variables)]
#![allow(clippy::match_like_matches_macro)]

mod evaluation;
mod morpho;
mod ortho;
mod wiki;

/*
    Ideas about the strucutre.
    - Having several modes. I want this projec to be quite flexible. The idea is not only to support wikipsalhalhe, but also other projects in the future if need be.
        Right now wikipsalhalhe is the main focus, but it should be extensible to other projects.

    1. Template extraction:
        We give the engine a template string. It extract the necessary information from it.
    2.
*/
fn main() {
    wiki::main();
}
