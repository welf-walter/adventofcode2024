type Fileid = u32;

#[derive(Clone)]
struct Disk {
    sectors:Vec<Option<Fileid>>
}

impl Disk {
    fn is_used(&self, index:usize) -> bool {
        self.sectors[index].is_some()
    }
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

fn defrag(before:&Disk) -> Disk {
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
    let disk1defrag = defrag(&disk1);
    assert_eq!(disk1defrag.sectors,vec![
        Some(0), Some(2), Some(2),
        Some(1), Some(1), Some(1),
        Some(2), Some(2), Some(2),
        None, None, None,
        None, None, None]);

    let disk2 = read_input("2333133121414131402");
    assert_eq!(disk2.sectors, vec![
        Some(0), Some(0), None, None, None, Some(1), Some(1), Some(1), None, None, None, Some(2),
        None, None, None, Some(3), Some(3), Some(3), None, Some(4), Some(4), None, Some(5), Some(5), Some(5), Some(5),
        None, Some(6), Some(6), Some(6), Some(6), None, Some(7), Some(7), Some(7), None,
        Some(8), Some(8), Some(8), Some(8), Some(9), Some(9)]);

    let disk2defrag = defrag(&disk2);
    assert_eq!(disk2defrag.sectors,vec![
        Some(0), Some(0), Some(9), Some(9), Some(8),
        Some(1), Some(1), Some(1), Some(8), Some(8),
        Some(8), Some(2), Some(7), Some(7), Some(7),
        Some(3), Some(3), Some(3), Some(6), Some(4),
        Some(4), Some(6), Some(5), Some(5), Some(5),
        Some(5), Some(6), Some(6),
        None, None, None, None, None, None, None, None, None, None, None, None, None, None]);
}
