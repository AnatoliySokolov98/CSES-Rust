fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let letters: Vec<u8> = read!(it, String).into_bytes();
    let mut counts = [0; 26];
    let total = letters.len();
    for letter in letters {
        let index = letter - b'A';
        counts[index as usize] += 1;
    }
    let biggest = *counts.iter().max().unwrap();
    let rest = total - biggest;
    if rest + 1 < biggest {
        wln!(out, "-1");
        return;
    }
    let mut res = Vec::new();

    for j in 0..total {
        let mut found_max = false;
        for (i, &v) in counts.iter().enumerate() {
            let rest = total - j - v;
            if v > rest {
                res.push(b'A' + (i as u8));
                found_max = true;
                counts[i] -= 1;
                break;
            }
        }
        if found_max {
            continue;
        }
        for (i, &v) in counts.iter().enumerate() {
            let prev = res.last();
            if v > 0 && (prev.is_none() || i as u8 != (prev.unwrap() - b'A')) {
                res.push(b'A' + (i as u8));
                counts[i] -= 1;
                break;
            }
        }
    }
    let res = String::from_utf8(res).unwrap();
    wln!(out, "{}", res);
}
