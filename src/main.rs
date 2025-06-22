use printpdf::*;
use rustyline::Editor;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
struct Student {
    name: String,
    total_marks: f64,
    max_marks: f64,
    num_subjects: u32,
    average: f64,
    percentage: f64,
    grade: String,
}

fn calculate_average(total: f64, subjects: u32) -> f64 {
    total / subjects as f64
}

fn calculate_percentage(total: f64, max: f64) -> f64 {
    (total / max) * 100.0
}

fn assign_grade(percentage: f64) -> String {
    match percentage {
        90.0..=100.0 => "A".to_string(),
        75.0..=89.9 => "B".to_string(),
        60.0..=74.9 => "C".to_string(),
        _ => "D".to_string(),
    }
}

fn generate_pdf(student: &Student) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .expect("Failed to load font");

    // Title
    let title_x = Mm(60.0);
    let title_y = Mm(270.0);
    current_layer.use_text("Student Report Card", 18.0, title_x, title_y, &font);

    // Underline under title
    let underline_start = Point::new(Mm(55.0), Mm(268.0));
    let underline_end = Point::new(Mm(150.0), Mm(268.0));
    let title_line = Line {
        points: vec![(underline_start, false), (underline_end, false)],
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.add_shape(title_line);

    // Report card box dimensions
    let box_x = Mm(40.0);
    let box_y_top = Mm(240.0);
    let box_width = Mm(130.0);
    let box_height = Mm(90.0);
    let box_bottom = Mm(box_y_top.0 - box_height.0);

    // Draw report box
    let box_points = vec![
        (box_x, box_y_top),
        (Mm(box_x.0 + box_width.0), box_y_top),
        (Mm(box_x.0 + box_width.0), box_bottom),
        (box_x, box_bottom),
        (box_x, box_y_top),
    ];
    let report_box = Line {
        points: box_points
            .into_iter()
            .map(|(x, y)| (Point::new(x, y), false))
            .collect(),
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.add_shape(report_box);

    // Centered report content
    let lines = vec![
        format!("{:<16}: {}", "Name", student.name),
        format!("{:<16}: {}", "Marks Obtained", student.total_marks),
        format!("{:<16}: {}", "Max Marks", student.max_marks),
        format!("{:<16}: {}", "Subjects", student.num_subjects),
        format!("{:<16}: {:.2}", "Average Marks", student.average),
        format!("{:<16}: {:.2}%", "Percentage", student.percentage),
        format!("{:<16}: {}", "Grade", student.grade),
    ];

    let font_size = 12.0;
    let total_lines = lines.len() as f64;
    let line_height = 10.0;

    // Vertical centering: calculate starting Y
    let text_block_height = total_lines * line_height;
    let start_y = box_y_top.0 - ((box_height.0 - text_block_height) / 2.0);

    for (i, line) in lines.iter().enumerate() {
        // Estimate horizontal center (since printpdf doesn't measure text width, we "eyeball" it)
        let text_x = Mm(box_x.0 + (box_width.0 / 2.0) - 40.0); // Adjust -40.0 if needed
        let text_y = Mm(start_y - (i as f64 * line_height));
        current_layer.use_text(line, font_size, text_x, text_y, &font);
    }

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap()))
        .unwrap();

    println!("✅ PDF saved as report_card.pdf");
}

fn main() {
    let mut rl = Editor::<()>::new().unwrap();

    let name: String = rl.readline("Enter student name: ").unwrap();
    let total_marks: f64 = rl
        .readline("Enter total marks obtained: ")
        .unwrap()
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let max_marks: f64 = rl
        .readline("Enter maximum possible marks: ")
        .unwrap()
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let num_subjects: u32 = rl
        .readline("Enter number of subjects: ")
        .unwrap()
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let average = calculate_average(total_marks, num_subjects);
    let percentage = calculate_percentage(total_marks, max_marks);
    let grade = assign_grade(percentage);

    let student = Student {
        name,
        total_marks,
        max_marks,
        num_subjects,
        average,
        percentage,
        grade,
    };

    println!("\n🧾 Student Summary:");
    println!("Name: {}", student.name);
    println!("Average Marks: {:.2}", student.average);
    println!("Percentage: {:.2}%", student.percentage);
    println!("Grade: {}", student.grade);

    generate_pdf(&student);
}
