mod health_check; // DECLARE: "health_check.rs exists."
mod subscriptions; // RE-EXPORT: "Everything inside that file is now publicly available under this module's name"

pub use health_check::*;
pub use subscriptions::*;


// the mod filename.rs tells the compiler "There are two files in this folder named ... Include them in the project." These files are private.
// in simple words, it declares that module exists and binds it into a specific file, you are defining a module tree with the `mod` statement

// the pub use filename::*  takes everything inside those private files and re-exports them as if they lived directly inside mod.rs/this_current_file 

// the "use" creates a symbolic link to an item (function, struct, etc.) so you don't have to type the full path. It means you are importing a name into the current namespace.

// "pub use" combines the "bringing into scope" with "public visibility". It takes an item from a private location and makes it available to the public as if it were define in the current file. You are re-exporting. You are changing the public API of your library.

// "*" means everything, meaning all of the functions within the 