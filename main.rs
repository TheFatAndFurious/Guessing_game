// librairie pour recuperer les inputs de l'utilisateur
use std::io;
// librairie qui permet de comparer la proposition de l'utilisateur avec le nombre secret
use std::cmp::Ordering;
// librairie pour generer un nombre random et choisir son range
use rand::Rng;


fn main() {
    println!("Guess the number!");
    
    // on cree une variable pour contenir un nombre secret entre 1 et 100
    let secret_number = rand::thread_rng().gen_range(1..=100);


    //boucle pour recommencer apres chaque tentative infructueuse
    loop {
        println!("Please input your guess.");

        // on cree une variable mutable (mut) et vide (String::new()) pour recevoir la proposition de nombre de l'utilisateur
        let mut guess = String::new();

        // on appelle la fonction stdin de la librairie IO pour nous permettre de manipuler les entrees de l'utilisateur
        io::stdin()
        // on appelle read_line pour recevoir la proposition de l'utilisateur, en passant 'mut guess' comme argument pour lui indiquer dans quelle string stocker l'input utilisateur 
        // le & avant mut indique qu'il s'agit d'une 'reference' qui permet a plusieurs parties du programme d'acceder a son contenu sans la copier plusieurs fois en memoire.
        // retourne une valeur Result (une 'enum') qui est un type qui possede plusieurs etats (chaque etat est appele une 'variante') Dans le cas de Result les deux etats sont 'Ok' ou 'Err'
            .read_line(&mut guess)
        //recupere une erreur eventuelle et fait crasher le programme (note:ressemble un peu au try/catch en JS)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        
        //on compare le nombre saisi par l'utilisateur avec le nombre secret puis on affiche un message en fonction du resultat de la comparaison
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
        }
    }
}
}
