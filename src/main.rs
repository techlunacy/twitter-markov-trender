fn main() {
    let chain = ["this is a string", "this is another string", "is this a string?"];
    let keys = chain.iter().flat_map(|s| s.split_whitespace().next()).collect();


    for i in keys.iter() {
        println!("{}", i);
    }

        println!("~~~~~~~~~~~~");
    for i in chain.iter() {
        for j in i.split_whitespace() {
            println!("{}", j);
        }
    }
}
