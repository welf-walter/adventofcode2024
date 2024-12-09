type Fileid = u32;

#[derive(Clone, Debug, PartialEq)]
struct Disk {
    sectors:Vec<Option<Fileid>>
}

impl Disk {
    fn is_used(&self, index:usize) -> bool {
        self.sectors[index].is_some()
    }

    fn checksum(&self) -> u64 {
        let mut akku = 0;
        for i in 0..self.sectors.len() {
            akku = akku + (i as u64 * self.sectors[i].unwrap_or(0) as u64 );
        }
        akku
    }

    fn find_free_block(&self, blocksize:usize) -> Option<usize> {
        for i in 0..self.sectors.len()-blocksize {
            let mut all_free = true;
            for j in 0..blocksize {
                all_free = all_free && self.sectors[i+j].is_none();
            }
            if all_free {
                return Some(i);
            }
        }
        return None;
    }
}

#[test]
fn test_find_free_block() {
    let disk = read_input("34567");
    assert_eq!(disk.find_free_block(4),Some(3));
    assert_eq!(disk.find_free_block(5),Some(3+4+5));
    assert_eq!(disk.find_free_block(6),Some(3+4+5));
    assert_eq!(disk.find_free_block(7),None);
}

fn read_input(line:&str) -> Disk {
    let mut sectors:Vec<Option<Fileid>> = Vec::new();
    let mut is_file_next = true;
    let mut fileid = 0;
    for c in line.chars() {
        let blocksize = c.to_digit(10).unwrap();
        for _i in 0..blocksize {
            if is_file_next {
                sectors.push(Some(fileid));
            }
            else
            {
                sectors.push(None);
            }
        }
        if is_file_next {
            is_file_next = false;
        }
        else
        {
            is_file_next = true;
            fileid += 1;
        }
    }
    Disk { sectors }
}

fn defrag1(before:&Disk) -> Disk {
    let mut disk = before.clone();
    let mut first_free = 0;
    let mut last_used = disk.sectors.len()-1;
    loop {
        while disk.is_used(first_free) {
            first_free += 1;
        }
        while !disk.is_used(last_used) {
            last_used -= 1;
        }
        if first_free > last_used {
            return disk;
        }
        // swap:
        disk.sectors[first_free] = disk.sectors[last_used];
        disk.sectors[last_used] = None;
    }
}

fn defrag2(before:&Disk) -> Disk {
    let mut disk = before.clone();
    let mut file_to_move:Fileid = disk.sectors.iter().max().unwrap().unwrap();
    loop {
        let file_size = disk.sectors.iter().filter( |file_id| file_id.is_some() && file_id.unwrap() == file_to_move ).count();
        let old_pos = disk.sectors.iter().position( |file_id| file_id.is_some() && file_id.unwrap() == file_to_move ).unwrap();
        let new_poso = disk.find_free_block(file_size);
        if new_poso.is_some() && new_poso.unwrap() < old_pos {
            let new_pos = new_poso.unwrap();
            for i in 0..file_size {
                disk.sectors[new_pos+i] = disk.sectors[old_pos+i];
                disk.sectors[old_pos+i] = None;
            }
        }
        if file_to_move == 0 {
            return disk;
        }
        file_to_move -= 1;
    }
}

#[test]
fn test_read_input() {
    let disk1 = read_input("12345");
    assert_eq!(disk1.sectors.len(), 1+2+3+4+5);
    assert_eq!(disk1.sectors,vec![
        Some(0),
        None, None,
        Some(1), Some(1), Some(1),
        None, None, None, None,
        Some(2), Some(2), Some(2), Some(2), Some(2)]);
    let disk1defrag = defrag1(&disk1);
    assert_eq!(disk1defrag.sectors,vec![
        Some(0), Some(2), Some(2),
        Some(1), Some(1), Some(1),
        Some(2), Some(2), Some(2),
        None, None, None,
        None, None, None]);
    let disk1defrag2 = defrag2(&disk1);
    assert_eq!(disk1, disk1defrag2);

    assert_eq!(disk1defrag.checksum(), 0*0+1*2+2*2+3*1+4*1+5*1+6*2+7*2+8*2);


    let disk2 = read_input("2333133121414131402");
    assert_eq!(disk2.sectors, vec![
        Some(0), Some(0), None, None, None, Some(1), Some(1), Some(1), None, None, None, Some(2),
        None, None, None, Some(3), Some(3), Some(3), None, Some(4), Some(4), None, Some(5), Some(5), Some(5), Some(5),
        None, Some(6), Some(6), Some(6), Some(6), None, Some(7), Some(7), Some(7), None,
        Some(8), Some(8), Some(8), Some(8), Some(9), Some(9)]);

    let disk2defrag = defrag1(&disk2);
    assert_eq!(disk2defrag.sectors,vec![
        Some(0), Some(0), Some(9), Some(9), Some(8),
        Some(1), Some(1), Some(1), Some(8), Some(8),
        Some(8), Some(2), Some(7), Some(7), Some(7),
        Some(3), Some(3), Some(3), Some(6), Some(4),
        Some(4), Some(6), Some(5), Some(5), Some(5),
        Some(5), Some(6), Some(6),
        None, None, None, None, None, None, None, None, None, None, None, None, None, None]);
    assert_eq!(disk2defrag.checksum(), 1928);

    let disk2defrag2 = defrag2(&disk2);
    assert_eq!(disk2defrag2.sectors,vec![
        Some(0), Some(0), Some(9), Some(9), Some(2),
        Some(1), Some(1), Some(1), Some(7), Some(7),
        Some(7), None, Some(4), Some(4), None,
        Some(3), Some(3), Some(3), None, None,
        None, None, Some(5), Some(5), Some(5),
        Some(5), None, Some(6), Some(6), Some(6),
        Some(6), None, None, None, None, None,
        Some(8), Some(8), Some(8), Some(8), None, None]);
    assert_eq!(disk2defrag2.checksum(), 2858);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


pub fn puzzle() {
    let file = File::open("input/day9.txt").expect("Could not open input/day9.txt");
    let reader = BufReader::new(file);

    let line:String = reader.lines().next().unwrap().unwrap();
    let disk = read_input(line.as_str());
    let disk_defrag = defrag1(&disk);
    let checksum = disk_defrag.checksum();
    println!("Day 9, Part 1: Checksum of disk is {}", checksum);

}
