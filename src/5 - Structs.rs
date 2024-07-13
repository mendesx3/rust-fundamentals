#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    grades: Vec<f32>,
}

impl Student {
    fn new(name: String, age: u8, grades: Vec<f32>) -> Student {
        Student { name, age, grades }
    }
    fn add_grade(&mut self, grade: f32) {
        self.grades.push(grade)
    }

    fn remove_grade(&mut self, index: usize) -> Result<f32, String> {
        if index < self.grades.len() {
            Ok(self.grades.remove(index))
        } else {
            Err(String::from("Index out of bounds"))
        }
    }
    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Grades: {:?}", self.grades);
    }
}

fn main() {
    let s1 = Student {
        name: String::from("Jack"),
        age: 18,
        grades: vec![9.7, 4.9, 5.5, 10.00],
    };
    println!("Name: {}, Age: {}, Grade: {:?}", s1.name, s1.age, s1.grades);

    //new instance
    let mut s2 = Student {
        name: String::from("Will"),
        ..s1
    };
    s2.display();

    s2.add_grade(1.00);
    s2.display();

    let mut s3 = Student::new(String::from("Vic"), 95, vec![9.7, 4.9, 5.5, 10.00]);
    s3.display();

    match s3.remove_grade(2) {
        Ok(grade) => println!("Removed grade: {}", grade),
        Err(err) => println!("Error: {}", err),
    }

    match s3.remove_grade(5) {
        Ok(grade) => println!("Removed grade: {}", grade),
        Err(err) => println!("Error: {}", err),
    }
}