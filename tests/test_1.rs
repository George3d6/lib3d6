#[cfg(test)]
mod tests {
    #[test]
    fn test_fstrings() {
        use lib3d6::{format, print, println};

        let v1 = ("arga", "barga", 33);
        let v2 = 121421;
        let v3: f32 = 32.12;
        let v4: &str = "Ahhha";
        let v5 = ["1", "2", "three"];
        
        assert_eq!(
            format!("This is the garbage format of old"), 
            "This is the garbage format of old"
        );
        assert_eq!(
            format!(f"This is new and improved. Let me print some variables: {v1}, {v3}, here's {v2} and {v4} oh, and {v5} and here's {v2} again!"), 
            "This is new and improved. Let me print some variables: (\"arga\", \"barga\", 33), 32.12, here's 121421 and \"Ahhha\" oh, and [\"1\", \"2\", \"three\"] and here's 121421 again!"
        );

        print!(f"We have f-string print {v4}\n");
        println!(f"And new-line print {v1}!!");
    }

    #[test]
    fn test_readme() {
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
    }
}