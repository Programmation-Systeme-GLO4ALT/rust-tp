// Programme 1 : Pourquoi ce code ne compile-t-il pas ?



// Bon ici le vecteur v est déplacé vers v2, ce qui signifie que v n'est plus accessible
// pour que le code compile, il faut utiliser un emprunt au lieu d'un déplacement
fn main() {
    let v = vec![1, 2, 3];
    let v2 = &v;  // Emprunt au lieu de déplacement
    println!("Longueur : {}", v.len());
}

// Programme 2 : Corrigez sans utiliser clone()

// nombres est déplacé dans somme()
// Modifier somme() pour prendre une référence : &Vec<i32>
fn somme(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres);
    println!("Somme : {}, Vecteur : {:?}", s, nombres);
}