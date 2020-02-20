/*
    Die wichtigsten Aktionen sollen öffentlich sein.
    Unteraktionen, die Teil einer übergeordnenten Aktion sind,
    sollen private bleiben
*/

//importiere Einheit mit benötigten Enums
use crate::einheit::{Einheit, Rasse, Klasse, Groesse, Erfahrenheit};
use crate::Haeufigkeit;

impl Einheit {
    //bestimmt den zusätzlichen Schaden, wenn die Einheit effektiv ist
    fn zuesatz_schaden(&self) -> i32{
        match self.rasse {
            Rasse::Orks => 5, //Ausgleich dafür, dass sie kaum Rüstung haben
            Rasse::Menschen => 2,
            Rasse::Drachen => 0,//Angriff schon so mächtig
            Rasse::Elfen => 4, //Elfen wissen besonders Orks gut zu schaden
        }
    }

    //bestimmt ob die Einheit gegen die Feindeinheit effektiv ist
    pub fn ist_effektiv(&mut self, gegner: &Einheit) -> bool {
        let ist = match self.rasse {
            Rasse::Orks => {
                    if let Rasse::Menschen = gegner.rasse{
                        true
                    } else {
                        false
                    }
            },
            Rasse::Menschen => {
                    if let Rasse::Drachen = gegner.rasse{
                        true
                    } else {
                        false
                    }
            },
            Rasse::Drachen => {
                    if let Rasse::Elfen = gegner.rasse{
                        true
                    } else {
                        false
                    }
            },
            Rasse::Elfen => {
                    if let Rasse::Orks = gegner.rasse{
                        true
                    } else {
                        false
                    }
            },
        };
        ist //ist er effektiv: Ja oder Nein?
    }

    /*
        Einheit soll einen einfachen Angriff durchführen
        können. (Hier wird noch nicht Felder oder die Klassen betrachtet)
        Eine Einheit soll
        Schaden = Einheit:Angriff - Gegner:Verteidigung
        berechnen und falls Schaden > 0 ist, dem
        Gegner zufügen, falls Schaden < 0 ist, soll
        der Angreifer -Schaden viel HP verlieren,
        falls Schaden = 0, dann soll nichts passieren.
        Effektivität der Einheit gegen den Feind wird berücksichtigt.
        In allen Fällen soll false zurückgegeben werden.
        (Später sollen Felder und Klassen berücksichtigt werden,
        weswegen dann manche Angriffe grundlegend nicht möglich
        sein sollten)
    */
    pub fn angriff(&mut self, gegner: &mut Einheit) -> bool {
        println!("{} greift mutig {} an",
        self.einheitsbezeichnung, gegner.einheitsbezeichnung);
        //bestimme die Verteidigung des Gegners und den Angriff der eigenen Einheit
        let mut vert = gegner.verteidigung;
        let angriff = if self.ist_effektiv(&gegner) {
            vert = 0; //Gegnerische Rüstung dann wertlos
            self.damage + self.zuesatz_schaden() //und Zusatsschaden
        } else {
            self.damage //kein Zusatsschaden
        };
        println!("Der Angriff erfolgt mit {} Angriffschaden von {}
        und {} Verteidigung von {}", angriff, self.einheitsbezeichnung,
        vert, gegner.einheitsbezeichnung);

        //füge den Gegner Schaden zu
        let schaden = angriff - vert;
        let mut error = false;
        match schaden {
            0 => println!("kein Schaden angerichtet"),
            1..=255 => {
                gegner.hp -= schaden;
                println!("Der gegnerischen Einheit {}
                wurde {} viel Schaden zugefügt!",
                gegner.einheitsbezeichnung, schaden);
                println!("-->{} Leben {}",gegner.einheitsbezeichnung, gegner.hp);
            },
            -255..=-1 => {
                self.hp += schaden;
                println!("Deine Einheit ist zu swach!!!\n
                Du erleidest im Kampf gegen {} {} viel Schaden.",
                gegner.einheitsbezeichnung, -schaden);
                println!("-->{} Leben {}",self.einheitsbezeichnung, self.hp);
            },
            _ => {
                error=true;
                println!("Illegale Anzahl an Schaden ist
                         ermittelt worden: {}",schaden);
                println!("Deine Einheit: {:?}",self);
                println!("Gegnerische Einheit: {:?}",gegner);
            },
        }
        error
    }
}
