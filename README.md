# Lib3d6

This is my Rust utility library, there are many like it, many superior in almost all aspects, but this one is mine.

Currently it's a sort of toy project as I'm re-learning the language.

## F-String Print

Python printing is cool, it's pretty easy to get it working in rust.

**Warning: Implementation is currently WIP, it works but I'm sure there are bugs and it's inefficient and somewhat ugly.**

```
// Importing is kind of ugly, looking for help making it better
// `*` import leads to ambiguity, and I can't get an import macro working
use lib3d6::{format, print, println};

let v1 = ("arga", "barga", 33);
let v2 = 121421;
let v3: f32 = 32.12;
let v4: &str = "Ahhha";
let v5 = ["1", "2", "three"];
let v6 = "have".to_owned();
let v7 = Some("tea");

assert_eq!(
    // Overrides format to support f-string
    format!(f"This is {v7}"),
    
    // Ugly old format still works
    format!("This is {:?}", v7)
);

// Same thing happens with print and println
println!(f"{v5} {v6} {v7}");
print!(f"It can work with any object of a type that implements the Debug trait, such as this tuple: {v1}\n");
print!(f"But also with primitives like v3 ({v3}), and it handles duplicates: {v4}, {v4}, {v4}");
println!("\nBut as always, {} still {:?}", "the old way of doing things", "works");
```