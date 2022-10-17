pub mod en_str {
    #[derive(Clone, PartialEq, Copy, Debug)]
    pub enum Gender {
        Female,
        Male,
    }
    #[derive(Clone, PartialEq, Copy, Debug)]
    pub enum Type {
        Urinal,
        Stall,
        FemaleStall,
    }

    #[derive(Clone, PartialEq, Copy, Debug)]
    pub enum BathroomVariant {
        Feciate,
        Urinate,
    }
    #[derive(Clone, PartialEq, Copy, Debug)]
    pub struct Person {
        pub is_vaping: bool,
        pub gender: Gender,
        pub variant: BathroomVariant,
        pub time_remaining: u32,
        pub at_stall: bool,
        pub finished: bool,
        pub stall_type: Type,
        pub index: usize,
    }
}
