use std::fs;
use serde::Deserialize;
use serde_json::Result;
#[derive(Clone, Debug, Deserialize)]

struct Student {
    name: String,
    age: u32,
    phone: String,
}

fn main() ->Result<()>{
    let inside_file = fs::read_to_string("file.txt").expect("Nu fisier");

    let mut old: Option<Student> = None;
    let mut new: Option<Student> = None;

    for line in inside_file.lines(){
        let student: Student = serde_json::from_str(line).expect("Nu a parsat");

        if old.is_none() || student.age > old.as_ref().unwrap().age{
            old = Some(student.clone());
        }

        if new.is_none() || student.age < new.as_ref().unwrap().age{
            new = Some(student.clone());
        }
       

    }

    if let Some(student) = old{
        println!("Studentul cel main mare: {}, Phone: {}, Age: {}", student.name, student.phone, student.age);
    }

    if let Some(student) = new{
        println!("Studentul cel main mic: {}, Phone: {}, Age: {}", student.name, student.phone, student.age);
    }

    Ok(())
}
