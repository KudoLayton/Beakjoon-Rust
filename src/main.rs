use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let size_usize: Vec<_> = input.split_whitespace().map(|x| x.parse::<usize>().expect("Falied")).collect();
    let mut map = vec![0u64; size_usize[1]];
    let white_map:u64 = 0xAA55AA55AA55AA55;

    // Read map
    for i in 0usize..size_usize[0]{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let line:Vec<_> = input.chars().collect();
        for j in 0usize..size_usize[1]{
            map[i] = map[i] << 1u64;
            if let 'W' = line[j]{
                map[i] += 1u64;
            }
        }
    }

    // Scan each patch
    
    let mut min_score = 64;
    for i in 0usize..(size_usize[0] - 8 + 1){
        for j in 0usize..(size_usize[1] - 8 + 1){
            let scan_lines:u64 = (&map[i..i+8]).iter().fold(0, |acc, x| (acc << 8) + ((x >> j) & 0xFF) );
            let scan_result = scan_lines ^ white_map;
            let mut score = 0;
            for k in 0..64{
                score += (scan_result >> k) & 0x1u64;
            }
            min_score = min_score.min(score.min(64u64 - score));
        }
    }
    println!("{}", min_score);
}
