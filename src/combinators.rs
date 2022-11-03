fn main() {
    let students = vec![
        "Bodgan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoly 4.0",
    ];

    // * Declarative Way
    let good_students: Vec<Student> = students
        .iter()
        .flat_map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_string();
            let gpa: f32 = s.next()?.parse().ok()?;

            Some(Student { name, gpa })
        })
        .filter(|p| p.gpa >= 3.5)
        .collect();
    println!("{:?}", good_students);

    // * Imperative Way
    let mut good_students: Vec<Student> = vec![];
    for student in students {
        let mut s = student.split(' ');

        if let (Some(name), Some(gpa)) = (s.next(), s.next()) {
            if let Ok(gpa) = gpa.parse::<f32>() {
                if gpa >= 3.5 {
                    good_students.push(Student {
                        name: name.to_string(),
                        gpa,
                    });
                }
            }
        }
    }
    println!("{:?}", good_students);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    gpa: f32,
}
