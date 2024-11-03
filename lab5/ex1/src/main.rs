use std::fs;
#[derive(Clone)]

struct Student {
    name: String,
    age: u32,
    phone: String,
}

fn main(){
    let inside_file = fs::read_to_string("file.txt").expect("Nu fisier");

    let mut old: Option<Student> = None;
    let mut new: Option<Student> = None;

    for line in inside_file.lines(){
       let i: Vec<&str> = line.split(',').collect();

       if i.len() == 3 {
        let name = i[0].to_string();
        let phone = i[1].to_string();
        let age: u32 = i[2].trim().parse().unwrap();
        let student = Student{name, phone, age};

        if old.is_none() || student.age > old.as_ref().unwrap().age{
            old = Some(student.clone());
        }

        if new.is_none() || student.age < new.as_ref().unwrap().age{
            new = Some(student.clone());
        }
       }

    }

    if let Some(student) = old{
        println!("Studentul cel main mare: {}, Phone: {}, Age: {}", student.name, student.phone, student.age);
    }

    if let Some(student) = new{
        println!("Studentul cel main mic: {}, Phone: {}, Age: {}", student.name, student.phone, student.age);
    }
}
