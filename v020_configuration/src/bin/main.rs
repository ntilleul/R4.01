use clap::Parser;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io;
pub mod configuration;
use configuration::Configuration;

fn commands() {
    println!("Veuillez taper une commande.");
    println!("Voici les commandes disponibles :");
    println!("  - voter [votant] [candidat] || permet de faire voter un votant pour un candidat");
    println!("  - votants || permet d'afficher la liste des votants");
    println!("  - scores || permet d'afficher les scores des candidats");
}

#[tokio::main]
async fn main() {
    let mut candidats: BTreeMap<String, u64> = BTreeMap::new(); //liste candidats
    let mut votants = BTreeSet::new(); //liste votants

    candidats.insert("Nul".to_string(), 0);
    candidats.insert("Blanc".to_string(), 0);

    let config = Configuration::parse();

    for candidat in config.candidates {
        candidats.insert(candidat.to_string(), 0);
    }

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
                let third_word = words.next();
                match first_word {
                    Some(word) => {
                        if word == "voter" {
                            if let Some(votant) = second_word {
                                if votants.contains(&votant.to_string()) {
                                    println!("{votant} a déjà voté !\n")
                                } else {
                                    if let Some(candidat) = third_word {
                                        if candidats.contains_key(&candidat.to_string()) {
                                            println!("{votant} a voté pour {candidat}\n");
                                            if let Some(nb_vote) = candidats.get_mut(candidat) {
                                                *nb_vote += 1;
                                            }
                                        } else {
                                            println!("{votant} a voté nul\n");
                                            if let Some(nb_vote) = candidats.get_mut("Nul") {
                                                *nb_vote += 1;
                                            }
                                        }
                                    } else {
                                        println!("{votant} a voté blanc\n");
                                        if let Some(nb_vote) =
                                            candidats.get_mut(&"Blanc".to_string())
                                        {
                                            *nb_vote += 1;
                                        }
                                    }
                                    let nouveau_votant = votant.to_string();
                                    votants.insert(nouveau_votant);
                                }
                            } else {
                                println!("Il manque un nom de votant !\n");
                            }
                        } else if word == "votants" {
                            println!("Liste des personnes ayant votés :");
                            for votant in &votants {
                                println!("{votant}");
                            }
                            println!("");
                        } else if word == "scores" {
                            for (candidat, score) in &candidats {
                                println!("{candidat} : {score}")
                            }
                        } else {
                            println!("Commande inconnue\n");
                        }
                    }
                    None => println!("Erreur\n"),
                }
            }
        }
    }
}
