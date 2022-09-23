mod en_str;
use en_str::en_str::{Gender, BathroomVariant, Person};
/*
    3 sets for each gender
    boys have 3 urinals, 3 stalls
     girls have three stalls
*/

fn main() {
    let mut people: Vec<Person> = vec![];
    for _ in 1..450 {
        let gender: Gender;
        let variant: BathroomVariant;

        let random_variant: u32 = rand::random();
        let random_gender: u32 = rand::random();

        if random_variant % 10 >= 9 {
            variant = BathroomVariant::Feciate;
        } else {
            variant = BathroomVariant::Urinate;
        }
        if random_gender % 10 >= 5 {
            gender = Gender::Male;
        } else {
            gender = Gender::Female;
        }

        let time_remaining: u32 = match variant.clone() {
            BathroomVariant::Feciate => 180,
            BathroomVariant::Urinate => 45,
        };

        let person = Person {
            gender,
            variant,
            time_remaining,
			at_stall: false
        };
        people.push(person);
    }
    simulate(people);
}

fn simulate(people: Vec<Person>) {
    /* 18 boys at a time, 9 girls at a time
     Feciating takes ~3 minutes
     Urinating takes ~45 seconds
     %50 of the students are girls the other half are boys
    */
	let male_stalls = 9;
	let urinals = 9;
	let female_stalls = 9;

	for person in people {
		
	}
}
