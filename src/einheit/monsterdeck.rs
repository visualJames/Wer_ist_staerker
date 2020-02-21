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

//Die verschiedenen Ork-Decks
enum OrkDeck {
    KoboldeGoblins,
    Daemmerwaldorks,
    TrolleOger,
}

//gibt eine Einheit der Kobolde oder Goblins vom Orkstamm zurück
fn kobolde_goblins_einheit() -> Einheit {
    //bestimme ob Kobold oder Goblin
    let kategorie = rand::thread_rng().gen_range(0, 1);
    let einheit :Einheit =
    match kategorie {
        0 => { //Kobold (zwar leicht selten, desw. hat nur dieses Deck sie)
            erstellen::erstelle_kobold_messerwerfer() //1/2
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
                    0 => erstellen::erstelle_goblin_Arbeiter(), //3.5/20
                    _ => erstellen::erstelle_goblin_wassertraeger(), //3.5/20
                }
            }
        },
    };
    einheit //Die Einheit wird letzlich zurückgegeben
}

//gibt eine Einheit der Dämmerwaldorks und Goblins vom Orkstamm zurück
fn daemerwald_orks() -> Einheit {
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
                    0 => erstellen::erstelle_goblin_Arbeiter(), //14/50
                    _ => erstellen::erstelle_goblin_wassertraeger(), //14/50
                }
            }
        },
    };
    einheit //Die Einheit wird letzlich zurückgegeben
}
