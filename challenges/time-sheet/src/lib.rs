use std::{io::Read, fmt::Display};

pub struct Week {
    week: Vec<(String, i8)>,
    count: usize,
    min: (String, i8),
    max: (String, i8)
}

impl Week {
    pub fn new() -> Self {
        Self {
            week: Vec::new(),
            count: 0,
            min: (String::new(), 0),
            max: (String::new(), 0)
        }
    }

    pub fn add(&mut self, day: (String, i8)) {
        if self.week.is_empty() {
            self.min = day.clone();
            self.max = day.clone();
        }

        match day.1 {
            d if d > self.max.1 => self.max.1 = d, 
            d if d < self.min.1 => self.min.1 = d,
            _ => {}
        }
        
        self.week.push(day);
        self.count += 1;
    }

    fn is_valid_input(&self, input: &(String, i8)) -> bool {
        let day = &input.0;
        let days: [&str; 7] = [
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ];

        let correct_day: bool = days.contains(&day.to_lowercase().as_str());
        let correct_hours: bool = input.1 <= 24 && input.1 >= 0;

        correct_day && correct_hours
    }
}

pub fn prompt(week: &mut Week) {
    let mut user_input: String = String::new();
    let mut day_name: String = String::new();
    let mut hours: i8 = 0;

    loop { 
        println!("What day did you work for day {}?", week.count + 1);
        
        // Get two pieces of input
        std::io::stdin().read_line(&mut user_input).expect("Could not read input.");
        day_name = user_input.clone();

        println!("How many hours did you work on {}?", user_input);
        std::io::stdin().read_line(&mut user_input).expect("Could not read input.");

        if is_numeric(&user_input) {
            println!("Invalid hour passed. Please try again");
            continue;
        }

        hours = user_input.trim().parse::<i8>().expect("Could not parse to i8.");

        if week.is_valid_input(&(day_name.clone(), hours)) {
            week.add((day_name, hours));
        } else {
            println!("Invalid input, please try again.");
        }

        if week.count >= 5 {
            break;
        }
    }

    print_results(week);
}

fn print_results(week: &mut Week) {
    let mut total_hours: i8 = 0;

    for day in week.week.clone() {
        total_hours += day.1;  
    }

    println!("Total hours worked: {}", total_hours);
    println!("Least hours worked: {:?}", week.min);
    println!("Most hours worked: {:?}", week.max);
}

fn is_numeric(str: &String) -> bool {
    str.chars().all(char::is_numeric)
}

#[test]
fn test_validate() {
    let day_wrong:(String, i8) = (String::from("thing"), 24);
    let day_correct: (String, i8) = (String::from("monday"), 23);
    let day_hour_incorrect: (String, i8) = (String::from("monday"), 25);
    let day_hour_incorrect_2: (String, i8) = (String::from("tuesday"), -1);
    let week: Week = Week::new();

    assert!(!week.is_valid_input(&day_wrong));
    assert!(week.is_valid_input(&day_correct));
    assert!(!week.is_valid_input(&day_hour_incorrect));
    assert!(!week.is_valid_input(&day_hour_incorrect_2));
}

#[test]
fn test_parsing() {
    let num_str: String = String::from("8");
    let invalid_num_str: String = String::from("a");   

    assert!(is_numeric(&num_str));
    assert!(!is_numeric(&invalid_num_str));
}