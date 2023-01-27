fn main() {
    let students = vec![
        "Bodgan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoly 4.0",
    ];

    // * Declarative Approach
    let good_students = students
        .iter()
        .flat_map(|s| {
            let mut student = s.split_whitespace();
            let name = student.next()?.to_string();
            let gpa: f32 = student.next()?.parse().ok()?;

            Some(Student::new(name, gpa))
        })
        .filter(|p| p.gpa >= 3.5)
        .collect::<Vec<_>>();

    println!("{good_students:?}");

    // * Imperative Approach
    let mut good_students: Vec<Student> = vec![];
    for student in students {
        let mut student = student.split(' ');

        if let (Some(name), Some(gpa)) = (student.next(), student.next()) {
            if let Ok(gpa) = gpa.parse::<f32>() {
                if gpa >= 3.5 {
                    good_students.push(Student::new(name.to_string(), gpa));
                }
            }
        }
    }
    println!("{good_students:?}");
}

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    gpa: f32,
}

impl Student {
    const fn new(name: String, gpa: f32) -> Self {
        Self { name, gpa }
    }
}
