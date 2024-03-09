fn main() {
    // Message d'accueil
    println!("Bienvenue sur votre compte bancaire !");

    // Initialisation du compte
    let mut balance = 1000.0;
    println!("Votre solde initial est de : {}€", balance);

    // Depot sur le compte
    loop {
        println!("Combien souhaitez-vous ajouter au solde ?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
        
            //convertir un string vers f64
            let deposit: f64 = match input.trim().parse() {
                Ok(num) if num >= 0.0 => num,
                _ => {
                    println!("Veuillez entrer un nombre positif valide.");
                    continue;
                }
            };
            //gérer le depot en faisant une addition
            balance += deposit;
            println!("Votre solde modifié est de : {}€", balance);
            break;
    }
    


    // Retrait sur le compte
    loop {
        println!("Combien souhaitez-vous retirer au solde ?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
    
        let withdrawal: f64 = match input.trim().parse() {
            Ok(num) if num >= 0.0 => num,
            _ => {
                println!("Veuillez entrer un nombre positif valide.");
                continue;
            }
        };
    

        if balance - withdrawal <= 0.0 {
            println!("Impossible, solde est insufisant !");
            continue;
        } else {
            balance -= withdrawal;
        }
       
        println!("Votre solde modifié est de : {}€", balance);
        break;
    }
   

    // Affichage du solde (géré: par emprunt/reference)
    let borrowed_balance = &balance;
    println!("Votre solde avant frais de gestion est de : {}€", borrowed_balance);


    // Application des frais de gestion, affiche du solde final via le shadowing
    let withdrawal_fees = 10.0;
    let balance = balance - withdrawal_fees;
    println!("Votre solde final est de : {}€", balance);

}
