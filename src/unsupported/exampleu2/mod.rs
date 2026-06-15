struct Person {
    name: String,
}

fn exampleu2() {
    let person = Person { name: String::from("Peter") };
    assert!(person.name.eq("Peter"));
}