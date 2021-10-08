fn process_hello_world() {
    assert!(rtasks::hello_world());
}

#[test]
fn get_user_input_that_matches(){
    process_hello_world("hello world!")
}