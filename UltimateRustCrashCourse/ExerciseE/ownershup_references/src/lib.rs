    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    //inspect(&arg);

    pub fn inspect(s: &String) {
        println!("Is string Plural? {}", (s.ends_with("s")));
    }

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    //change(&mut arg);
    //println!("I have many {}", arg);

    pub fn change(s: &mut String) {
        if !s.ends_with("s") {
            s.push_str("s");
        }
    }

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    pub fn eat(s: String) -> bool {
        s.starts_with("b") && s.contains("a")
    }

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //

    pub fn bedazzle(s: &mut String) {
        //s.insert_str(0, "sparkly");
        (*s) = String::from("sparkly");
    }