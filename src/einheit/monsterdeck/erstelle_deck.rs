
//importiere Einheit mit benötigten Enums
use crate::einheit::monsterdeck::{OrkDeck, gib_kobolde_goblins_einheit,
    gib_daemerwald_orks, gib_trolle_oger_goblins};
use crate::einheit::{Einheit};
    use std::io;
    use arr_macro::arr; //um sicher und schnell riesige Array zu initialisieren
    //(https://www.joshmcguigan.com/blog/array-initialization-rust/)
        pub fn erstelle_ork_deck() -> OrkDeck{
            loop {
                println!("Bitte gebe Kobold, Ork oder Oger ein für dein gewünschtes Deck");
                let mut auswahl = String::new();
                io::stdin().read_line(&mut auswahl).expect("Failed to read line");
                let deck = match auswahl.trim() {
                    "Kobold" =>  {
                        println!("Erstelle Kobold Deck");
                        erstelle_kobold_deck()
                    },
                    "Ork" => {
                        println!("Erstelle Orkdammerwald Deck");
                        erstelle_orkdaemmerwald_deck()
                    },
                    "Oger" => {
                        println!("Erstelle Oger Deck");
                        erstelle_oger_deck()
                    },
                    _ => {
                    println!("Fehler bei Eingabe! \nBitte gebe sie nur einer der folgenden
                    Worte ein: Kobold, Ork oder Oger ein");
                    continue;
                    },
                };
                return deck;
            }

        }

        fn erstelle_kobold_deck() -> OrkDeck {
            let mut deck : [Einheit;14] = arr![gib_kobolde_goblins_einheit(); 14];
            OrkDeck::KoboldeGoblins(deck) //hier ist das passende Deck
        }

        fn erstelle_orkdaemmerwald_deck() -> OrkDeck {
            let mut deck : [Einheit;13] = arr![gib_daemerwald_orks(); 13];
            OrkDeck::Daemmerwaldorks(deck) //hier ist das passende Deck
        }

        fn erstelle_oger_deck() -> OrkDeck {
            let mut deck : [Einheit;12] = arr![gib_trolle_oger_goblins(); 12];
            OrkDeck::TrolleOger(deck) //hier ist das passende Deck
        }
