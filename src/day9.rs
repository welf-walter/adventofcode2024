type Fileid = u32;

struct Disk {
    sectors:Vec<Option<Fileid>>
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
}