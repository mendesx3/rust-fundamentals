struct Student {
    name: String,
    grade: f32,
}

fn main() {
    get_slices()
}

fn get_slices() {
    let a = [0, 1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}", slice.is_empty());

    for elem in slice.iter() {
        println!("{}", elem);
        println!("{}", elem);
    }

    let students = vec![
        Student { name: String::from("Jhon"), grade: 10.0 },
        Student { name: String::from("Smith"), grade: 95.0 },
        Student { name: String::from("Peter"), grade: 85.0 },
        Student { name: String::from("Jack"), grade: 50.0 },
    ];

    let slice = &students[1..2];
    let average = calculate_average(slice);
    println!("average {}", average);
}

fn calculate_average(s: &[Student]) -> f32 {
    let total: f32 = s.iter().map(|student| student.grade).sum();
    total / s.len() as f32
}

