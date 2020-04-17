use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let cases = line.trim_end().parse::<i32>()?;

    for _ in 0..cases {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let v: Vec<&str> = line.trim_end().split(" ").collect();
        let r = v[0].parse::<i64>()?;
        let e = v[1].parse::<i64>()?;
        let c = v[2].parse::<i64>()?;

        let t = e - c;
        if t > r {
            println!("advertise");
        }
        else if t < r {
            println!("do not advertise");
        }
        else {
            println!("does not matter");
        }

    }

    Ok(())
}