

fn main() {
/*    let mut v = Vec::new();
    v.push(12);

    let v1 = vec![12;100];
    println!("v1 : {:?}", v1);

    let v2 = vec![12,100];
    println!("v2 : {:?}", v2);*/



    let mut v = Vec::new();
    v.push(12);
    println!("first element of v : {}", v[0]);

    v.extend(1..20);
    println!("first element of v : {}", v[0]);
    println!("Some elements of v : {:?}", &v[2..7]);
    println!("Some other elements of v : {:?}", &v[5..=10]);
    println!("Yet other elements of v : {:?}", &v[..4]);
    println!("Yet again other elements of v : {:?}", &v[16..]);

    for e in v.iter_mut() {
        *e += 100;
    }
    println!("v after modification : {:?}", v);

}






#[cfg(test)]
mod tests {


}



