#[allow(unused_variables)]
use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        enum State {
            CreateNew,
            Append,
        }

        let state = match self.grades.get(&grade) {
            Some(_) => State::Append,
            None => State::CreateNew,
        };

        match state {
            State::Append => {
                let mut vector = self.grades.get_mut(&grade).unwrap();
                vector.push(student.to_string());
                vector.sort();
            },
            State::CreateNew => {
                let mut vector = Vec::new();
                vector.push(student.to_string());
                self.grades.insert(grade, vector);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut keys: Vec<u32> = self.grades.keys().map(|&d| d).collect();
        keys.sort();
        keys
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.grades.get(&grade) {
            Some(students) => Option::Some(students.to_vec()),
            None => None,
        }
    }
}
