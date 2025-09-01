mod data;
// 2F for / 23 for #
// TODO: add data padding,

fn main(){
    let a=data::DATA; // __START_DATA__";
    let r=13;
    let mut g=vec![vec![' ';4*r+1];2*r+1];
    let b='\u{2F}';
    for y in 0..=r-1{
        let x=((r*r-y*y)as f32).sqrt()as usize;
        for i in 2*r-2*x..=2*r+2*x{
            g[r-y][i]=b;
            g[r+y][i]=b;
        }
    }

    let mut f=g.iter().map(|r|r.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
    let v=a.chars().flat_map(|c|(c>' ').then(||char::from_u32((c as u32)-200).unwrap())).collect::<String>();
    for c in format!(" fn main(){{let a=\"{a:<470}{v}").chars(){f=f.replacen(b,&c.to_string(),1)}
    println!("{f}");
}
