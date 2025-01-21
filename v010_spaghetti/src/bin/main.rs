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
	    	Some(input) => println!("{input}"),
	    }

    }
}
