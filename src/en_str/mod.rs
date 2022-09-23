pub mod en_str {
    pub enum Gender {
        Female,
        Male,
    }

    #[derive(Clone)]
    pub enum BathroomVariant {
        Feciate,
        Urinate,
    }

    pub struct Person {
        pub gender: Gender,
        pub variant: BathroomVariant,
        pub time_remaining: u32,
        pub at_stall: bool,
    }
}
