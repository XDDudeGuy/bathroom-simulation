mod en_str;
use en_str::en_str::{BathroomVariant, Gender, Person, Type};

fn main() {
    // counters
    let mut failures = 0;
    let mut success = 0;
    loop {
        let mut people: Vec<Person> = vec![];
        // amount of people going to the bathroom
        let people_amt = rand::random::<u32>() % 400 + 51;

        for u in 1..people_amt {
            let gender: Gender;
            let variant: BathroomVariant;

            let random_variant: u32 = rand::random();
            let random_gender: u32 = rand::random();

            // 2 / 10 people have to feciate, the others urinate
            if random_variant % 10 >= 9 {
                variant = BathroomVariant::Feciate;
            } else {
                variant = BathroomVariant::Urinate;
            }
            // half of the people are males, the other half females
            if random_gender % 10 >= 5 {
                gender = Gender::Male;
            } else {
                gender = Gender::Female;
            }

            let time_to_get = rand::random::<u32>()%151+2;

            let time_remaining: u32 = match variant {
                BathroomVariant::Feciate => (rand::random::<u32>()%101+80)+time_to_get,
                BathroomVariant::Urinate => (rand::random::<u32>()%31+15)+time_to_get,
            };

            let stall_type: Type;

            // which type of stall should be used (only did this for ease of use later on)
            if gender == Gender::Female {
                stall_type = Type::FemaleStall;
            } else if variant == BathroomVariant::Feciate {
                stall_type = Type::Stall;
            } else {
                stall_type = Type::Urinal;
            }

            // creating a new person
            let person = Person {
                gender,
                variant,
                time_remaining,
                at_stall: false,
                finished: false,
                stall_type,
                index: u as usize -1 as usize,
            };
            // adding that person to the list of people
            people.push(person);
        }
        // simulating those people and checking if it was a success
        let percent = simulate(people);
        if percent.clone() > 80.0 {
            success += 1;
        } else {
            failures += 1;
        }
        // printing to the console the amount of failures and successes
        println!("\nFailures: {:?}\nSuccesses: {:?}\nPercentage: {:?}", failures, success, percent);
    }
}

fn simulate(mut people: Vec<Person>) -> f32 {
    // the amount of stalls, urinals, and female stalls in the building along with the total time to do your business
    let mut male_stalls: u8 = 6;
    let mut urinals: u8 = 6;
    let mut female_stalls: u8 = 10;
    let mut total_time: u16 = 300;

    // while there is still time left
    while total_time > 0 {
        for mut person in people.clone() {
            // if the person is done make person.finished true and free up their stall
            if person.time_remaining <= 0 {
                person.finished = true;
                person.at_stall = false;
                if person.gender == Gender::Female {
                    female_stalls += 1;
                } else if person.stall_type == Type::Stall {
                    male_stalls += 1;
                } else if person.stall_type == Type::Urinal {
                    urinals += 1;
                }
                people[person.index] = person;
            }
            // (just for safety) if there is no time left leave this loop and stop immediately
            if total_time == 0 {
                break;
            }
            // if the person is finished they don't need the bathroom anymore
            if person.finished {
                continue;
            }
            // count down the time remaining for people who are at a stall
            if person.at_stall {
                person.time_remaining -= 1;
                people[person.index] = person;
                continue;
            }
            // if the person is a girl and there are female stalls put them in a stall
            if person.gender == Gender::Female {
                if female_stalls > 0 {
                    female_stalls -= 1;
                    person.at_stall = true;
                }
            }
            /* if the person is not a girl*/
            else {
                // if he has to feciate and there are stalls put him in one
                if person.variant == BathroomVariant::Feciate {
                    if male_stalls > 0 {
                        male_stalls -= 1;
                        person.at_stall = true;
                    }
                }
                /* if he has to urinate and there are urinals put him in one*/
                else if urinals > 0 && person.stall_type == Type::Urinal {
                    urinals -= 1;
                    person.at_stall = true;
                }
                /* if there are stalls,
                no urinals,
                and the person has to urinate,
                put him in a stall*/
                else if urinals == 0 && person.stall_type == Type::Urinal && male_stalls > 0 {
                    male_stalls -= 1;
                    person.at_stall = true;
                }
            }
            // if there are no stalls or urinals the person just has to wait
            people[person.index] = person;
        }
        // tick down the total time
        if total_time != 0 {
            total_time -= 1;
        }
    }
    // check if everyone who needed to use the bathrooms can use the bathroom
    return check_satisfaction(people);
}

fn check_satisfaction(people: Vec<Person>) -> f32 {
    let mut finished_people: f32 = 0.0;
    let mut unfinished_people: f32 = 0.0;
    for person in people {
        match person.finished {
            true => finished_people += 1.0,
            false => unfinished_people += 1.0
        }
    }
    let percentage: f32 = finished_people/(unfinished_people+finished_people);
    percentage*100.0
}