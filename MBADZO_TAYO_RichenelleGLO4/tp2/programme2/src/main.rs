fn somme(v: Vec<i32>) -> (i32, Vec<i32>) {
    let s = v.iter().sum();
    (s, v)
}

fn main() {
    let nombres = vec![1, 2, 3, 4, 5];
    let (s, nombres) = somme(nombres);
    println!("Somme : {}, Vecteur : {:?}", s, nombres);
}