mod data;

fn main(){
    let a=data::DATA; // __START_DATA__";
    let r=13;
    let mut g=vec![vec![' ';4*r+1];2*r+1];
    let b='\u{23}';

    for y in 0..=r{
        let x=((r*r-y*y)as f32).sqrt()as usize;
        for i in 2*r-2*x..=2*r+2*x{
            g[r-y][i]=b;
            g[r+y][i]=b;
        }
    }

    let mut f=g.iter().map(|r|r.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
    let s=" fn main(){let a=\"";
    let mut d=|s:&str|{f=f.replacen(b,s,1)};
    (s.to_string()+a).chars().for_each(|c|d(&c.to_string()));
    a.chars().for_each(|c|if!c.is_whitespace(){d(&char::from_u32((c as u32)-200).unwrap().to_string())});
    println!("{f}");
}
