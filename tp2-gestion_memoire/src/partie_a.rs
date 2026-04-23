pub fn programme1() {
    let v = vec![1, 2, 3];
    let v2 = v.clone(); // intentionally unused

    println!("Longueur : {}", v.len());
}

pub fn programme2() {
    let nombres = vec![1, 2, 3, 4, 5];

    let (s, nombres) = somme(nombres);

    println!("Somme : {}, Vecteur : {:?}", s, nombres);
}

fn somme(v: Vec<i32>) -> (i32, Vec<i32>) {
    let s = v.iter().sum();
    (s, v)
}