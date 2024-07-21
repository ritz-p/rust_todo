pub mod structs {
    #[derive(Clone, PartialEq)]
    pub struct Todo {
        pub id: usize,
        pub text: String,
        pub completed: bool,
    }
}
