fn main() {
    solve_puzzle1();
    // solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let (files, free_space_blocks) = read_files_and_free_space_blocks();

    let compacted_file_system_blocks = compact_file_system_blocks(&files, &free_space_blocks);

    let mut checksum = 0;
    for (i, file_id) in compacted_file_system_blocks.iter().enumerate() {
        checksum += i * file_id;
    }

    println!("{checksum}");
}

fn compact_file_system_blocks(files: &Vec<File>, free_space_blocks: &Vec<usize>) -> Vec<usize> {
    let mut compacted_file_system_blocks = vec![];
    let mut last_file_index = files.len() - 1;
    let mut last_block_in_last_file_index = files[last_file_index].blocks;

    for (i, file) in files.iter().enumerate() {
        if i >= last_file_index {
            break;
        }

        for _ in 0..file.blocks {
            compacted_file_system_blocks.push(file.id);
        }

        let mut blocks_to_fill = free_space_blocks[i];
        while blocks_to_fill > 0 {
            let mut file_to_move = &files[last_file_index];
            if last_block_in_last_file_index == 0 {
                last_file_index -= 1;

                if i >= last_file_index {
                    break;
                }

                file_to_move = &files[last_file_index];
                last_block_in_last_file_index = file_to_move.blocks;
            }

            compacted_file_system_blocks.push(file_to_move.id);

            blocks_to_fill -= 1;
            last_block_in_last_file_index -= 1;
        }
    }

    let last_file_id = files[last_file_index].id;
    while last_block_in_last_file_index > 0 {
        compacted_file_system_blocks.push(last_file_id);

        last_block_in_last_file_index -= 1;
    }

    compacted_file_system_blocks
}

fn read_files_and_free_space_blocks() -> (Vec<File>, Vec<usize>) {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let trimmed_line = line.trim();

    let mut files = vec![];
    let mut free_space_blocks = vec![];
    let mut file_id = 0;
    let mut is_reading_file = true;

    for ch in trimmed_line.chars() {
        let blocks = ch.to_digit(10).unwrap() as usize;
        if is_reading_file {
            files.push(File {
                id: file_id,
                blocks,
            });

            file_id += 1;
        } else {
            free_space_blocks.push(blocks);
        }

        is_reading_file = !is_reading_file;
    }

    (files, free_space_blocks)
}

struct File {
    id: usize,
    blocks: usize,
}
