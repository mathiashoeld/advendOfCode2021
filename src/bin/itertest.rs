use core::slice::Iter;
fn main() {
    println!("Testing some stuff");

    let a = (1..100).collect::<Vec<u32>>();
    println!("{:?}", a);

    let mut i1 = a.iter();
    let mut i2 = a.iter();
    let t1 = vec![i1, i2];
    let _ = i2.next(); // 1
                       // now i1 and i2 are one element apart
                       // iterate until i2 is at the end
    for s in i2 {
        let v = i1.next().unwrap();
        println!("{}, {}, {}", v, s, v + s);
    }
}

fn sliding_window(input_vector: &Vec<u32>, window_size: u32) -> Vec<u32> {
    let mut asdf: Vec<Iter<u32>> = vec![];
    for i in 1..window_size {
        asdf.push(input_vector.iter());
    }

    Vec::from([1, 2, 3])
}
