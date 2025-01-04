fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/9#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let (files, free_space_blocks) = read_files_and_free_space_blocks();

    let checksum = calculate_checksum_of_compacted_file_system_blocks_by_moving_files(
        &files,
        &free_space_blocks,
    );

    println!("{checksum}");
}

// https://adventofcode.com/2024/day/9
#[allow(dead_code)]
fn solve_puzzle1() {
    let (files, free_space_blocks) = read_files_and_free_space_blocks();

    let checksum = calculate_checksum_of_compacted_file_system_blocks_by_moving_file_blocks(
        &files,
        &free_space_blocks,
    );

    println!("{checksum}");
}

fn calculate_checksum_of_compacted_file_system_blocks_by_moving_files(
    files: &[File],
    free_space_blocks: &[i32],
) -> u64 {
    let mut compacted_file_system_blocks =
        Vec::with_capacity(files.len() + free_space_blocks.len());

    // Build initial file system blocks
    for (i, file) in files.iter().enumerate() {
        for _ in 0..file.blocks {
            compacted_file_system_blocks.push(file.id);
        }

        if i <= free_space_blocks.len() - 1 && free_space_blocks[i] > 0 {
            compacted_file_system_blocks.push(-1 * free_space_blocks[i]);
        }
    }

    for file in files.iter().rev() {
        // Find big enough free space to move file into
        let free_space_index = compacted_file_system_blocks
            .iter()
            .enumerate()
            .filter(|(i, b)| {
                **b < 0
                    && b.abs() >= file.blocks
                    && *i
                        < compacted_file_system_blocks
                            .iter()
                            .position(|b| *b == file.id)
                            .unwrap()
            })
            .map(|(i, _)| i)
            .next();

        if let Some(free_space_index) = free_space_index {
            // Update the space left after file is moved
            let free_space_blocks = compacted_file_system_blocks[free_space_index].abs();
            if free_space_blocks > file.blocks {
                compacted_file_system_blocks[free_space_index] = file.blocks - free_space_blocks;
            } else {
                compacted_file_system_blocks.remove(free_space_index);
            }

            // Move file blocks into free space blocks
            for _ in 0..file.blocks {
                compacted_file_system_blocks.insert(free_space_index, file.id);
            }

            let old_file_block_index = compacted_file_system_blocks
                .iter()
                .rposition(|x| *x == file.id)
                .unwrap();

            // Replace old file blocks with free space blocks
            for i in 0..file.blocks {
                compacted_file_system_blocks[old_file_block_index - i as usize] = -1;
            }

            // Compact free space after moving file blocks
            for i in (1..compacted_file_system_blocks.len()).rev() {
                if compacted_file_system_blocks[i] < 0 && compacted_file_system_blocks[i - 1] < 0 {
                    compacted_file_system_blocks[i - 1] += compacted_file_system_blocks[i];
                    compacted_file_system_blocks.remove(i);
                }
            }
        }
    }

    let mut checksum = 0;
    let mut file_block_index = 0;
    for block in compacted_file_system_blocks.iter() {
        if *block >= 0 {
            checksum += file_block_index * (*block as u64);
            file_block_index += 1;
        } else {
            file_block_index += block.abs() as u64;
        }
    }

    checksum
}

fn calculate_checksum_of_compacted_file_system_blocks_by_moving_file_blocks(
    files: &[File],
    free_space_blocks: &[i32],
) -> u64 {
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

    let mut checksum = 0;
    for (i, file_id) in compacted_file_system_blocks.iter().enumerate() {
        checksum += (i as u64) * (*file_id as u64);
    }

    checksum
}

fn read_files_and_free_space_blocks() -> (Vec<File>, Vec<i32>) {
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
        let blocks = ch.to_digit(10).unwrap() as i32;
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
    id: i32,
    blocks: i32,
}
