use std::io;

fn commands(){
	println!("Veuillez taper une commande.");
	println!("Voici les commandes disponibles :");
	println!("  - voter [votant] [candidat] || permet de faire voter un votant pour un candidat");
	println!("  - votants || permet d'afficher la liste des votants");
	println!("  - scores || permet d'afficher les scores des candidats");
}

#[tokio::main]
async fn main(){

	let mut liste_votants : Vec<String> = vec![];//liste votants

	loop {
		let input_option: Option<String> = {
	        let mut input = String::new();
	        let _ = io::stdin().read_line(&mut input);
	        let trimmed = input.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
    	};

	    match input_option {
	    	None => commands(),
	    	Some(input) => {
	    		let mut words = input.split_whitespace();
	    		let first_word = words.next();
	    		let second_word = words.next();
	    		let _third_word = words.next();
                match first_word {
                    Some(word) => {
                    	if word == "voter" {
                    		match second_word {
                    			Some(word) => {
                    				if liste_votants.contains(&word.to_string()) {
                    					println!("{word} a déjà voté\n");
                    				} else {
                    					println!("{word} a voté nul\n");
                    					liste_votants.push(word.to_string());
                    				}
                    			},
                    			None => println!("Erreur\n"),
                    		}

                    	} else if word == "votants" {
                    		println!("Liste des personnes ayant votés :");
                    		for votant in liste_votants.iter() {
                    			println!("{votant}");
                    		}
                    		println!("");

                    	} else if word == "scores" {
                    		println!("Ok ! \n");

                    	} else {
                    		println!("Commande inconnue\n");
                    	}
                    },
                    None => println!("Erreur\n"),
                };
	    	},
	    };
    }
}
