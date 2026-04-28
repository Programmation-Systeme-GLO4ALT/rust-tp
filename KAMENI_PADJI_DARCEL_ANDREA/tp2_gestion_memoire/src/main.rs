// Programme 1 : pourquoi ce code ne compile t il pas ?

fn first(){
    let v = vec![1,2,3];        // les vecteurs qui ont les valeurs de type i32 sont alloués sur le tas, et la variable v est un pointeur vers ce tas
    let _v2 = v.clone();             // toutes ces valeurs se sont dirigés vers v2, et v n'est plus valide
    println!("Longueur :{}", v.len());
}

// solution : faire v2 = v.clone() pour que v2 soit une copie de v et que v soit toujours valide.


// Programme 2 : Corrigez sans utiliser clone()

fn somme(v: &Vec<i32>) -> i32 {          // ca prend en parametre des vecteurs qui contiennent des i32, et retourne un i32
    v.iter().sum()
}

fn main() {
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres);                         // meme chose que dans le programme 1, les valeurs de nombres se dirigent vers s, et nombres n'est plus valide
    println!("Somme : {}, Vecteur : {:?}", s, nombres);
    first();
}

// solution : faire somme(&nombres) pour preter à s la référence de nombres, et que nombres soit toujours valide.




