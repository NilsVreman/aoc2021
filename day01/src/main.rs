use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use itertools::izip;

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    // Call br to get all lines.
    // Call map on each line and then parse each value.
    // map error if it is not same as all other
    br.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn cmp_cons(vec: &Vec<i64>) -> i64 {
    return vec.iter().zip(vec.iter().skip(1))
        .fold(0, |acc, (&x, &y)| { 
            if y > x { acc + 1 } else { acc } 
        });
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input.txt")?)?;

    let i = cmp_cons(&vec);
    let vec3 = izip!(vec.iter(), vec.iter().skip(1), vec.iter().skip(2))
        .map(|(&x, &y, &z)| x+y+z)
        .collect();
    let j = cmp_cons(&vec3);
    println!("Number increasing consecutive: {}", i);
    println!("Number increasing triple: {}", j);
    Ok(())
}
