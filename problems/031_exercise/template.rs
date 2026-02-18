// Exercise 031: References - Scope Rules
//
// Learning Objective: Understand the borrowing rules and how scopes affect
// references. Learn the rules: 1) One mutable OR any number of immutable.
//
// Topics covered:
// - Borrowing rules: one &mut XOR many &
// - Non-lexical lifetimes (NLL)
// - Using scopes to manage reference lifetimes

fn main() {
    // RULE 1: You can have multiple immutable references
    let s1 = String::from("shared");
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    
    // TODO: Print all three references
    // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);

    // RULE 2: You can have only ONE mutable reference
    let mut s2 = String::from("exclusive");
    let m1 = &mut s2;
    // let m2 = &mut s2; // ERROR! Can't have two mutable references
    
    // TODO: Print the mutable reference
    // println!("m1: {}", m1);

    // After m1 is no longer used, we can create another mutable reference
    let m2 = &mut s2; // This is OK now because m1's scope ended
    // TODO: Print m2
    // println!("m2: {}", m2);

    // RULE 3: Can't mix mutable and immutable references
    let mut s3 = String::from("mixed");
    let imm1 = &s3;
    let imm2 = &s3;
    // TODO: Print the immutable references
    // println!("imm1: {}, imm2: {}", imm1, imm2);
    
    // After imm1 and imm2 are done, we can create a mutable reference
    let mut_ref = &mut s3;
    mut_ref.push_str(" modified");
    // TODO: Print the modified string
    // println!("After mutable: {}", mut_ref);

    // Using scopes to enforce the rules
    let mut text = String::from("scope demo");
    
    // TODO: Create a scope with immutable references
    // {
    //     let ref_a = &text;
    //     let ref_b = &text;
    //     println!("Immutable in scope: {} and {}", ref_a, ref_b);
    // } // Immutable references end here
    
    // TODO: Now create a mutable reference (allowed because imms are done)
    // let mut_ref = &mut text;
    // mut_ref.push_str(" - changed");
    // println!("After mutable: {}", mut_ref);

    // Demonstrate: returning references (lifetime elision)
    let word1 = String::from("short");
    let word2 = String::from("verylongwordindeed");
    
    // TODO: Call longest with references to word1 and word2
    // let result = longest(&word1, &word2);
    // println!("Longest is: {}", result);
}

// This function returns a reference to the longest string slice
// Note: The 'a is a lifetime parameter (we'll cover lifetimes later)
// For now, just understand it returns a reference
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// COMPLETION CHECKLIST:
// [ ] Print r1, r2, r3 (multiple immutable refs)
// [ ] Print m1 (single mutable ref)
// [ ] Print m2 (another mutable ref after m1 done)
// [ ] Print imm1 and imm2
// [ ] Print mut_ref after modification
// [ ] Create scope with immutable references
// [ ] Create mutable reference after scope ends
// [ ] Call longest() and print result
