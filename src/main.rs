mod bytes;

fn main(){
    let a:&[u8]=bytes::BYTES; // __START_DATA__];
    let r=21;
    let mut g=vec![vec![' ';4*r+1];2*r+1];

    for y in 0..=r{
        let x=((r*r-y*y)as f32).sqrt()as usize;
        for i in 2*r-2*x..=2*r+2*x{
            g[r-y][i]='#';
            g[r+y][i]='#';
        }
    }

    let mut f=g.iter().map(|r|r.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
    let s=" fn main(){{let q:&[u8]=&[";
    let mut d=|c:char|{f=f.replacen('#',&c.to_string(),1)};
    s.chars().for_each(&mut d);
    a.iter().fold("".to_string(),|a,b|a+&format!("{:02},",b)).chars().for_each(&mut d);
    a.iter().map(|b|{*b as char}).for_each(d);

    println!("{f}");
}
