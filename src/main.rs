mod en_str;
use en_str::en_str::{Gender, BathroomVariant, Person, Type};
/*
    3 sets for each gender
    boys have 3 urinals, 3 stalls
     girls have three stalls
*/

fn main() {
    let mut failures = 0;
    let mut success = 0;
    loop {
        let mut people: Vec<Person> = vec![];
        let chance = rand::random::<u32>()%400+51;
        println!("{:?}", chance.clone());
        for _ in 1..chance {
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

            let stall_type: Type;

            if gender == Gender::Female {
                stall_type = Type::FemaleStall;
            } else if variant == BathroomVariant::Feciate {
                stall_type = Type::Stall;
            } else {
                stall_type = Type::Urinal;
            }


            let person = Person {
                gender,
                variant,
                time_remaining,
                at_stall: false,
                finished: false,
                stall_type,
            };
            people.push(person);
        }
        if simulate(people) {
            success +=1;
        } else {
            failures +=1;
        }
        println!("\nFailures: {:?}\nSuccesses: {:?}", failures, success);
    }
}

fn simulate(people: Vec<Person>) -> bool {
    /* 18 boys at a time, 9 girls at a time
     Feciating takes ~3 minutes
     Urinating takes ~45 seconds
     %50 of the students are girls the other half are boys
    */
	let mut male_stalls = 9;
	let mut urinals = 9;
	let mut female_stalls = 9;
    let mut total_time = 300;

    while total_time > 0 {
        for mut person in people.clone() {
            if person.time_remaining == 0 {
                person.finished = true;
                if person.gender == Gender::Female {
                    female_stalls += 1;
                } else if person.stall_type == Type::Stall {
                    male_stalls += 1;
                } else if person.stall_type == Type::Urinal {
                    urinals += 1;
                }
            }
            if total_time == 0 {
                break
            }
            if person.finished {
                continue
            }
            if person.at_stall {
                person.time_remaining -= 1;
                total_time -= 1;
                continue
            }
            if person.gender == Gender::Female {
                if female_stalls > 0 {
                    female_stalls -= 1;
                    person.at_stall = true;
                }
            } else {
                if person.variant == BathroomVariant::Feciate {
                    if male_stalls > 0 {
                        male_stalls -= 1;
                        person.at_stall = true;
                    }
                } else if urinals > 0 && person.stall_type == Type::Urinal {
                    urinals -= 1;
                    person.at_stall = true;
                } else if urinals == 0 && person.stall_type == Type::Urinal && male_stalls > 0 {
                    male_stalls -= 1;
                    person.at_stall = true;
                }
            }
            total_time -= 1;
            continue
	    }
    }
    return check_satisfaction(people);
}

fn check_satisfaction(people: Vec<Person>) -> bool {
    for person in people {
        if person.finished != true {
            return person.finished
        }
    }
    true
}
