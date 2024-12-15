use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let contents = include_str!("../../input/day09_input.txt");

    let solution1 = get_checksum(&fragment_hard_drive(&contents));
    let solution2 = get_checksum(&defragment_hard_drive(&contents));

    (Solution::from(solution1), Solution::from(solution2))
}

#[derive(Debug)]
struct FileId {
    id: usize,
}

#[derive(Debug)]
struct FileWithSize {
    id: i32,
    size: usize,
    moved_once: bool,
    empty: bool,
}

fn fragment_hard_drive(data: &str) -> Vec<Option<FileId>> {
    let mut hard_drive = get_hard_drive(data);

    //sort the fragmented data
    loop {
        let Some(empty_position) = hard_drive.iter().position(|file_id| file_id.is_none()) else {
            break;
        };

        let Some(last_element) = hard_drive.pop() else {
            break;
        };

        if last_element.is_none() {
            continue;
        }

        hard_drive[empty_position] = last_element;            
    }

    hard_drive
}

fn defragment_hard_drive(data: &str) -> Vec<Option<FileId>> {
    let mut hard_drive = get_hard_drive_2(data);
    let mut intermediate_hard_drive = Vec::new();

    //sort the fragmented data
    loop {
        let Some(mut last_element) = hard_drive.pop() else {
            break;
        };

        if last_element.moved_once || last_element.empty {
            intermediate_hard_drive.insert(0, last_element);
            continue;
        }

        last_element.moved_once = true;

        let Some(empty_position) = hard_drive.iter().position(|file| file.empty && file.size >= last_element.size) else {
            intermediate_hard_drive.insert(0, last_element);
            continue;
        };

        let Some(empty_space) = hard_drive.get_mut(empty_position) else {
            panic!("We already confirmed this exists, how did this panic?");
        };

        if empty_space.size == last_element.size {
            hard_drive.push(FileWithSize { id: -1, size: last_element.size, moved_once: false, empty: true });
            hard_drive[empty_position] = last_element;
            continue;
        }
        else {
            empty_space.size -= last_element.size;
            hard_drive.push(FileWithSize { id: -1, size: last_element.size, moved_once: false, empty: true });
            hard_drive.insert(empty_position, last_element);
        }
    }

    convert_file_with_size_to_file_id(intermediate_hard_drive)
}

fn convert_file_with_size_to_file_id(input: Vec<FileWithSize>) -> Vec<Option<FileId>> {

    input.iter()
        .fold(Vec::new(), | mut output, file | {
            for _ in 0..file.size {
                if file.empty {
                    output.push(None);
                } 
                else {
                    output.push(Some(FileId { id: file.id as usize }));
                }
            }

            output
        })
}

fn get_hard_drive(data: &str) -> Vec<Option<FileId>> {
    let hard_drive = 
        data.chars()
        .enumerate()
        .fold(Vec::new(), |mut output, entry| {
            let iterator = (entry.1).to_digit(10).unwrap();

            if entry.0 % 2 == 0 {
                // even entries contain data
                for _ in 0..iterator {
                    output.push(Some(FileId { id: entry.0 / 2 }));
                }
            } else {
                // odd entries are blank
                for _ in 0..iterator {
                    output.push(None);
                }
            }

            output
        });

    hard_drive
}

fn get_hard_drive_2(data: &str) -> Vec<FileWithSize> {
    let hard_drive = 
        data.chars()
        .enumerate()
        .fold(Vec::new(), |mut output, entry| {
            let size = (entry.1).to_digit(10).unwrap() as usize;

            if entry.0 % 2 == 0 {
                // even entries contain data
                output.push(FileWithSize { id: (entry.0 / 2) as i32, size, moved_once: false, empty: false });
            } else {
                // odd entries are blank
                output.push(FileWithSize { id: -1, size, moved_once: false, empty: true });
            }

            output
        });

    hard_drive
}

fn get_checksum(hard_drive: &Vec<Option<FileId>>) -> usize {
    let checksum = hard_drive
        .iter()
        .enumerate()
        .fold(0usize, | acc, entry| {
            let Some(file_id) = entry.1 else {
                return acc;
            };
            acc + (entry.0 * file_id.id)
        });

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = include_str!("../../samples/day09_sample.txt");

        let frag_result = get_checksum(&fragment_hard_drive(&example));
        let defrag_result = get_checksum(&defragment_hard_drive(&example));

        assert_eq!(frag_result, 1928);
        assert_eq!(defrag_result, 2858);
    }
}