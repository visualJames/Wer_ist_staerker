//Copyright: 2020 André Hodapp

/*
    Hierdrin ist ein Deck bestehend aus Einheiten zu finden.
    (Eine Einheit ist ein Lebewesen einer bestimmten Rasse/Volkes, die
    für den Kampf eingesetzt wird.)
    Es gibt für jeweils jede der 4 Rassen (Orks, Menschen, Drachen, Elfen)
    ein oder mehrere Decks.
    Diese Decks beinhalten nur Einheiten.
    Für Ausrüstungs- und/oder Heilungskarten soll es ein seperates Deck
    zum Ziehen geben.
*/

//importiere Einheit mit benötigten Enums
use crate::einheit::{Einheit, Rasse, Klasse, Groesse, Erfahrenheit};
use crate::Haeufigkeit;


use rand::Rng; //Um eine Zufallszahl zu bekommen

use crate::einheit::erstellen; //Um Einheiten zu erstellen

pub mod erstelle_deck;

use std::iter::{Iterator, Enumerate}; //um zu iterieren über Vektoren

//Die verschiedenen Ork-Decks
pub enum OrkDeck {
    KoboldeGoblins(Vec<Einheit>),     // ^
    Daemmerwaldorks(Vec<Einheit>),    // | Größe der Decks, also Anzahl an Karten
    TrolleOger(Vec<Einheit>),         // |
}

impl OrkDeck {
    //Soll das ganze Deck ausgeben
    pub fn gib_ganzes_deck_aus(&self){
        //nimm dieses Orkdeck und packe es in einem Vector, sodass die bel.
        //Größe egal ist und man gut damit arbeiten kann
        let deck = match self {
            OrkDeck::KoboldeGoblins(deck) => deck,
            OrkDeck::Daemmerwaldorks(deck) => deck,
            OrkDeck::TrolleOger(deck) => deck,
        };
        for (i,vec) in deck.iter().enumerate() {
            println!("{}.te Einheit: {:?}", i, vec);
        }
    }

    //Soll das oberste Element des Vektors im Deck poppen
    pub fn pop(&mut self) -> Einheit{
        //nimm dieses Orkdeck und packe es in einem Vector, sodass die bel.
        //Größe egal ist und man gut damit arbeiten kann
        let mut deck = match self {
            OrkDeck::KoboldeGoblins(deck) => deck,
            OrkDeck::Daemmerwaldorks(deck) => deck,
            OrkDeck::TrolleOger(deck) => deck,
        };
        deck.pop().expect("Deck ist leer")
    }
}


//gibt eine Einheit der Kobolde oder Goblins vom Orkstamm zurück
fn gib_kobolde_goblins_einheit() -> Einheit {
    //bestimme ob Kobold oder Goblin
    let kategorie = rand::thread_rng().gen_range(0, 2);
    let einheit :Einheit =
    match kategorie {
        0 => {
            let wen = rand::thread_rng().gen_range(0,30);
            match wen {
                0 => erstellen::erstelle_blutschlinger_ork_Bluttrinker(), //1/60
                1 => erstellen::erstelle_blutschlinger_ork(), //3/60
                2 => erstellen::erstelle_blutschlinger_ork(),
                3 => erstellen::erstelle_blutschlinger_ork(),
                _ => //Kobold (zwar leicht selten, desw. hat nur dieses Deck sie)
                    erstellen::erstelle_kobold_messerwerfer(), //26/60
            }
        },
        _ => { //Goblin (kommt auch in einem anderen Deck vor, da sie häufig sind)
            //bestimme welchen Goblin genau
            let welche_seltenheit = rand::thread_rng().gen_range(0, 10);
            if welche_seltenheit == 0 { //1/20
                erstellen::erstelle_goblin_truppenanfuehrer() //leicht selten
            } else if welche_seltenheit < 3 { //2/20
                erstellen::erstelle_goblin_bombenwerfer()
            } else { //7/20
                let welche_einheit = rand::thread_rng().gen_range(0,1);
                match welche_einheit {
                    0 => erstellen::erstelle_goblin_arbeiter(), //3.5/20
                    _ => erstellen::erstelle_goblin_wassertraeger(), //3.5/20
                }
            }
        },
    };
    einheit //Die Einheit wird letzlich zurückgegeben
}

//gibt eine Einheit der Dämmerwaldorks und Goblins vom Orkstamm zurück
fn gib_daemerwald_orks() -> Einheit {
    //bestimme ob Kobold oder Goblin
    let kategorie = rand::thread_rng().gen_range(0, 5);
    let einheit :Einheit =
    match kategorie {
        0 => { //Dämmerwald-Orks (selten aber stark)
            let welche_seltenheit = rand::thread_rng().gen_range(0,10);
            match welche_seltenheit {
                0 => erstellen::erstelle_daemmerwald_ork_schaedelspalter(), //1/50
                1 => erstellen::erstelle_daemmerwald_ork_Schamane(), //2/50
                2 => erstellen::erstelle_daemmerwald_ork_Schamane(),
                _ => erstellen::erstelle_daemmerwald_ork_Krieger(), //7/50
            }
        },
        _ => { //Goblin (kommt auch in einem anderen Deck vor, da sie häufig sind)
            //bestimme welchen Goblin genau
            let welche_seltenheit = rand::thread_rng().gen_range(0, 10);
            if welche_seltenheit == 0 { //1*4/50=4/50
                erstellen::erstelle_goblin_truppenanfuehrer() //leicht selten
            } else if welche_seltenheit < 3 { //2*4/50=8/50
                erstellen::erstelle_goblin_bombenwerfer()
            } else { //7*4/50=28/50
                let welche_einheit = rand::thread_rng().gen_range(0,2);
                match welche_einheit {
                    0 => erstellen::erstelle_goblin_arbeiter(), //14/50
                    _ => erstellen::erstelle_goblin_wassertraeger(), //14/50
                }
            }
        },
    };
    einheit //Die Einheit wird letzlich zurückgegeben
}


//gibt eine Einheit der Trolle, Oger und Goblins vom Orkstamm zurück
fn gib_trolle_oger_goblins() -> Einheit {
    //bestimme ob Kobold oder Goblin
    let kategorie = rand::thread_rng().gen_range(0, 7);
    let einheit :Einheit =
    match kategorie {
        0 => { //Dämmerwald-Orks (selten aber stark)
            let welche_seltenheit = rand::thread_rng().gen_range(0,10);
            match welche_seltenheit {
                0 => erstellen::erstelle_troll(), //3/60
                1 => erstellen::erstelle_troll(),
                2 => erstellen::erstelle_troll(),
                3 => erstellen::erstelle_oger_menschenfresser(), //1/60
                _ => erstellen::erstelle_oger() //7/60
            }
        },
        _ => { //Goblin (kommt auch in einem anderen Deck vor, da sie häufig sind)
            //bestimme welchen Goblin genau
            let welche_seltenheit = rand::thread_rng().gen_range(0, 20);
            if welche_seltenheit == 0 { //1*6/120=3/60
                erstellen::erstelle_goblin_truppenanfuehrer() //leicht selten
            } else if welche_seltenheit < 3 { //2*6/120=6/60
                erstellen::erstelle_goblin_bombenwerfer()
            } else { //17*6/120=17*3/60=34/60
                let welche_einheit = rand::thread_rng().gen_range(0,2);
                match welche_einheit {
                    0 => erstellen::erstelle_goblin_arbeiter(), //17/60
                    _ => erstellen::erstelle_goblin_wassertraeger(), //17/60
                }
            }
        },
    };
    einheit //Die Einheit wird letzlich zurückgegeben
}
