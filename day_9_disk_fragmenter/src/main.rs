fn main() {
    solve_puzzle1();
    // solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let file_system_elements = read_file_system_elements();

    let mut compacted_file_system = vec![];

    for element in file_system_elements.iter() {
        match element {
            FileSystemElement::File(file_ids) => {
                for file_id in file_ids {
                    compacted_file_system.push(file_id);
                }
            }
            FileSystemElement::FreeSpace(blocks) => {
                todo!()
            }
        }
    }

    let mut checksum = 0;
    for (i, file_id) in compacted_file_system.iter().enumerate() {
        checksum += i * file_id;
    }

    println!("{checksum}");
}

fn read_file_system_elements() -> Vec<FileSystemElement> {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let trimmed_line = line.trim();

    let mut result = vec![];
    let mut file_id = 0;
    let mut is_reading_file = true;

    for ch in trimmed_line.chars() {
        let blocks = ch.to_digit(10).unwrap() as usize;
        if is_reading_file {
            result.push(FileSystemElement::File(vec![file_id; blocks]));

            file_id += 1;
        } else {
            result.push(FileSystemElement::FreeSpace(blocks));
        }

        is_reading_file = !is_reading_file;
    }

    result
}

enum FileSystemElement {
    File(Vec<usize>),
    FreeSpace(usize),
}
