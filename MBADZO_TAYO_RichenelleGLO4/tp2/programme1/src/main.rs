fn main() {
    println!("Le programme 1 ne compilait pas car les types comme Vec (vecteurs) ne possèdent pas le trait Copy. 
    Lorsqu'on écrit let v2 = v;, la propriété (ownership) des données sur le tas (heap) est transférée de v à v2.
     On appelle cela un Move. Dès lors, v devient invalide et ne peut plus être utilisé !");
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("Longueur : {}", v2.len()); 
}