// Structs in this file represent the various forms that
// westwork may receive during configuration.

#[derive(FromForm)]
#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}