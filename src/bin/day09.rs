use std::io::{stdin, Read};

#[cfg(test)]
static TEST_INPUT: &str = "2333133121414131402";

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

fn map_to_disk(disk_map: &Vec<usize>) -> Vec<Option<usize>> {
    let mut disk = vec![];
    for i in (0..disk_map.len()).step_by(2) {
        let id = i / 2;
        disk.append(&mut vec![Some(id); disk_map[i]]);
        if i + 1 < disk_map.len() {
            disk.append(&mut vec![None; disk_map[i + 1]]);
        }
    }
    disk
}

#[test]
fn test_map_to_disk() {
    assert_eq!(
        map_to_disk(&parse("12345")),
        vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2)
        ]
    );
}

fn part1(input: &str) -> usize {
    let disk_map = parse(input);
    println!("{}", disk_map.len());
    let mut disk = map_to_disk(&disk_map);
    println!("{}", disk.len());
    let mut i = 0;
    while i < disk.len() {
        if disk[i].is_some() {
            i += 1;
            continue;
        }
        while disk[disk.len() - 1] == None {
            disk.pop();
        }
        if i < disk.len() {
            disk[i] = disk.pop().unwrap();
        }
        i += 1;
    }
    println!("{}", disk.len());
    disk.iter()
        .enumerate()
        .map(|(i, block)| i * block.unwrap())
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 1928);
}

#[derive(Debug, Clone, Copy)]
enum Extent {
    File { id: usize, size: usize },
    Free(usize),
}

fn parse2(input: &str) -> Vec<Extent> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Extent::File {
                    id: i / 2,
                    size: c.to_digit(10).unwrap() as usize,
                }
            } else {
                Extent::Free(c.to_digit(10).unwrap() as usize)
            }
        })
        .collect()
}

fn part2(input: &str) -> usize {
    let mut disk = parse2(input);
    let mut cur_pos = disk.len() - 1;
    let Extent::File {
        id: mut cur_id,
        size: mut cur_size,
    } = disk[cur_pos]
    else {
        panic!("last extent is not a file")
    };

    while cur_id > 0 && cur_pos > 0 {
        for i in 0..cur_pos {
            if let Extent::Free(free) = disk[i] {
                if free >= cur_size {
                    disk[cur_pos] = Extent::Free(cur_size);
                    // we don't need to merge adjacent free extents because nothing can be moved to
                    // the right of the pointer
                    disk[i] = Extent::File {
                        id: cur_id,
                        size: cur_size,
                    };
                    if free > cur_size {
                        disk.insert(i + 1, Extent::Free(free - cur_size));
                    }
                    break;
                }
            }
        }
        while cur_pos > 0 {
            if let Extent::File { id, size } = disk[cur_pos] {
                if id + 1 == cur_id {
                    cur_id = id;
                    cur_size = size;
                    break;
                }
            }
            cur_pos -= 1;
        }
    }

    let mut result = 0;
    let mut pos = 0;
    for block in disk {
        match block {
            Extent::Free(size) => {
                pos += size;
            }
            Extent::File { id, size } => {
                let block_sum = id * (pos..pos + size).sum::<usize>();
                result += block_sum;
                pos += size;
            }
        }
    }
    result
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 2858);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
