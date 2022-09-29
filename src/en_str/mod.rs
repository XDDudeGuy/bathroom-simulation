pub mod en_str {
    #[derive(Clone, PartialEq, Copy)]
    pub enum Gender {
        Female,
        Male,
    }
    #[derive(Clone, PartialEq, Copy)]
    pub enum Type {
        Urinal,
        Stall,
        FemaleStall,
    }

    #[derive(Clone, PartialEq, Copy)]
    pub enum BathroomVariant {
        Feciate,
        Urinate,
    }
    #[derive(Clone, PartialEq, Copy)]
    pub struct Person {
        pub gender: Gender,
        pub variant: BathroomVariant,
        pub time_remaining: u32,
        pub at_stall: bool,
        pub finished: bool,
        pub stall_type: Type,
    }
}
