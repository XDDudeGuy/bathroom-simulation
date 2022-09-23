pub mod en_str {
    #[derive(Clone, PartialEq)]
    pub enum Gender {
        Female,
        Male,
    }

    #[derive(Clone, PartialEq)]
    pub enum BathroomVariant {
        Feciate,
        Urinate,
    }
    #[derive(Clone, PartialEq)]
    pub struct Person {
        pub gender: Gender,
        pub variant: BathroomVariant,
        pub time_remaining: u32,
        pub at_stall: bool,
        pub finished: bool,
    }
}
