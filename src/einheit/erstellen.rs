/*
    Hierdrin sollen alle Konstruktoren angeboten werden
*/

//importiere Einheit mit benötigten Enums
use crate::einheit::{Einheit, Rasse, Klasse, Groesse, Erfahrenheit};
use crate::Haeufigkeit;

impl Einheit {
    /*
        Diese Klassenfunktion soll zurückgegeben,
        wie viel die Rasse mit dieser Klasse an Verteidigung
        haben soll. Verteidigung ist eine Art Rüstungswert.
    */
    fn verteidigung_berechnen(rasse: &Rasse, klasse: &Klasse,
        haeufigkeit: &Haeufigkeit) -> i32{
        match rasse {
            Rasse::Orks => { //Orks tragen zwar große Waffen,
                match klasse { //aber vor allem leichte Rüstungen
                    Klasse::Nah => 1,
                    Klasse::Fern => 0,
                    _ => 2,
                }
            },
            Rasse::Menschen => {
                match haeufigkeit {
                    Haeufigkeit::SehrHaeufig => {
                        match  klasse{ //Bauern
                            Klasse::Nah => 1,
                            _ => 0
                        }
                    }
                    Haeufigkeit::Selten => { //Ritter
                        match  klasse{
                            Klasse::Nah => 5,
                            _ => 3
                        }
                    },
                    Haeufigkeit::SehrSelten => { //Erzritter
                        match  klasse{
                            Klasse::Nah => 9,
                            _ => 4
                        }
                    },
                    Haeufigkeit::Episch => { //Generäle oder Könige
                        match  klasse{
                            Klasse::Nah => 11,
                            _ => 8
                        }
                    },
                    Haeufigkeit::Legendaer => { //großes Schwert, schwache Rüstung
                        match  klasse{
                            Klasse::Nah => 4,
                            _ => 2
                        }
                    },
                    _ => {
                        match  klasse{ //Banditen
                            Klasse::Nah => 3,
                            _ => 2
                        }
                    },
                }

            },
            Rasse::Drachen => { //Drachen haben sehr starke Haut
                if let Haeufigkeit::Legendaer = haeufigkeit { //speziell legendäre Drachen
                    15
                } else{
                    if let Klasse::Nah = klasse { //Drachen sind
                        10
                    } else{
                        7
                    }
                }
            },
            Rasse::Elfen => {
                match haeufigkeit {
                    Haeufigkeit::LeichtSelten => {
                        match  klasse{ //Elf-Krieger
                            Klasse::Nah => 7,
                            _ => 6
                        }
                    }
                    Haeufigkeit::Selten => { //Elf-Meister
                        match  klasse{
                            Klasse::Nah => 10,
                            _ => 9
                        }
                    },
                    Haeufigkeit::SehrSelten => { //Elf-Weise
                        match  klasse{
                            Klasse::Nah => 11,
                            _ => 0
                        }
                    },
                    Haeufigkeit::Episch => { //Elf-König
                        match  klasse{
                            Klasse::Nah => 14,
                            _ => 10
                        }
                    },
                    Haeufigkeit::Legendaer => { //Elf-Geist
                        match  klasse{
                            Klasse::Nah => 15,
                            _ => 14
                        }
                    },
                    _ => 0, //alles andere sollte nicht vorkommen
                }

            },
        }
    }

    /*
        Diese Klassenfunktion soll die HP der
        Einheit einer bestimmten Rasse und Klasse
        anhand seiner Groöße berechnen.
        Leitwert: Je größer, desto mehr HP
    */
    fn hp_berechnen(rasse: &Rasse, klasse: &Klasse,
        groesse: &Groesse) -> i32{
            let hp = match groesse{
                Groesse::Winzig => 1,
                Groesse::Klein => 3,
                Groesse::Normalgross =>7,
                Groesse::Gross => 10,
                Groesse::Riesig => 20,
                Groesse::Gigantisch => 40,
                Groesse::Monstrositaet => 100,
            };

            match rasse {
                Rasse::Orks =>{
                    match klasse {
                        Klasse::Nah => {
                            hp*6 //10*6=60(Ork), bzw. 1*6=6(Goblin)
                        },
                        _ => {
                            hp*5
                        },
                    }
                },
                Rasse::Menschen =>{
                    match klasse {
                        Klasse::Nah => {
                            hp*3 //7*3=21(Bandit)
                        },
                        _ => {
                            hp*2
                        },
                    }
                },
                Rasse::Drachen =>{
                    match klasse {
                        Klasse::Nah => {
                            hp*5 //40*7=280(Drache)
                        },
                        _ => {
                            hp*4
                        },
                    }
                },
                Rasse::Elfen =>{
                    match klasse {
                        Klasse::Nah => {
                            hp*3 //10*4=40(Elfe), 20*4=80(Baumriese)
                        },
                        _ => {
                            hp*2+5 //10*3=30(Elfe)
                        },
                    }
                },
            }
        }

        /*
            Diese Klassenfunktion soll das Damage der
            Einheit einer bestimmten Rasse und Klasse
            anhand seiner Erfahrenheit berechnen.
            Leitwert: Je erfahrender, desto mehr Damage
        */
        fn damage_berechnen(rasse: &Rasse, klasse: &Klasse,
            erfahrenheit: &Erfahrenheit) -> i32{
                let damage = match erfahrenheit{
                    Erfahrenheit::Unerfahren => 1,
                    Erfahrenheit::Ausgebildet => 2,
                    Erfahrenheit::Erfahren => 4,
                    Erfahrenheit::Veteran => 6,
                    Erfahrenheit::Meister => 18,
                };

                match rasse {
                    Rasse::Orks =>{
                        match klasse {
                            Klasse::Nah =>
                                damage*5, //4*5=20
                            Klasse::Fern =>
                                damage*2, //4*2=8
                            Klasse::Magier =>
                                damage*6, //4*6=24, mächtige Schamanen
                            Klasse::Heiler =>
                                damage*4, //4*5=20
                        }
                    },
                    Rasse::Menschen =>{
                        match klasse {
                            Klasse::Nah => {
                                damage*3 //2*6=12
                            },
                            _ => {
                                damage*1 //4*1=4
                            },
                        }
                    },
                    Rasse::Drachen =>{
                        damage*4 //6*4=24
                    },
                    Rasse::Elfen =>{
                        match klasse {
                            Klasse::Nah => {
                                damage*3 //4*3=12
                            }
                            Klasse::Fern => {
                                damage*4 //4*4=16
                            },
                            Klasse::Magier => {
                                damage*5 //4*5=20
                            },
                            Klasse::Heiler => {
                                damage*5 //4*5=20
                            }
                        }
                    },
                }
            }


    /*
        Dies ist ein Konstruktor, welche für die
        hierbenötigte Aufgabe einen gewöhnlichen
        Ork-Krieger erstellt
    */
    fn gemeiner_ork(einheitsbezeichnung: String, groesse: Groesse,
        erfahrenheit: Erfahrenheit) -> Einheit{
             let rasse = Rasse::Orks;
             let klasse = Klasse::Nah;
             let haeufigkeit = Haeufigkeit::Selten; //Ja, richtige Orks sind selten
             let verteidigung = Einheit::verteidigung_berechnen(&rasse,
                 &klasse, &haeufigkeit); //berechne Rüstung
             let hp = Einheit::hp_berechnen(&rasse, &klasse, &groesse);
             let damage = Einheit::damage_berechnen(&rasse, &klasse, &erfahrenheit);
        Einheit {
            einheitsbezeichnung, hp, damage, groesse,//übergebene Werte
            verteidigung,
            rasse,
            klasse,
            held: false,
            haeufigkeit,
        }
    }

    /*
        Dies ist ein Konstruktor, welche für die
        hierbenötigte Aufgabe einen gewöhnlichen
        Menschen-Banditen erstellt
    */
    fn gemeiner_bandit(einheitsbezeichnung: String,groesse: Groesse,
        erfahrenheit: Erfahrenheit)-> Einheit{
             let rasse =  Rasse::Menschen;
             let klasse = Klasse::Nah;
             let haeufigkeit = Haeufigkeit::Haeufig; //leider sind Banditen die Pest
             let verteidigung = Einheit::verteidigung_berechnen(&rasse,
                 &klasse, &haeufigkeit); //berechne Rüstung
             let hp = Einheit::hp_berechnen(&rasse, &klasse, &groesse);
             let damage = Einheit::damage_berechnen(&rasse, &klasse, &erfahrenheit);
        Einheit {
            einheitsbezeichnung, hp, damage, groesse,//übergebene Werte
            verteidigung,
            rasse,
            klasse,
            held: false,
            haeufigkeit,
        }
    }

    /*
        Dies ist ein Konstruktor, welche für
        die Erstellung eines seltenen Elfes ist
    */
    fn elf_bogenschuetze(einheitsbezeichnung: String, groesse: Groesse,
        erfahrenheit: Erfahrenheit)-> Einheit{
             let rasse = Rasse::Elfen;
             let klasse = Klasse::Fern;
             let haeufigkeit = Haeufigkeit::LeichtSelten; //Elfen waren früher nur
                                               //Legenden,jetzt entschwinden sie
                                               //immer mehr den Ruf der Fabelwesen
             let verteidigung = Einheit::verteidigung_berechnen(&rasse,
                 &klasse, &haeufigkeit); //berechne Rüstung
             let hp = Einheit::hp_berechnen(&rasse, &klasse, &groesse);
             let damage = Einheit::damage_berechnen(&rasse, &klasse, &erfahrenheit);
        Einheit {
            einheitsbezeichnung, hp, damage, groesse, //übergebene Werte
            verteidigung,
            rasse,
            klasse,
            held: false,
            haeufigkeit,
        }
    }

}

//___________________________ alles hierdrüber privat
//public Konstukturen für Verschiedene Einheiten einer bestimmten Rasse
    //Konstruktoren für die Rasse Orks
    pub fn erstelle_blutschlinger_ork() -> Einheit {
        let name = String::from("Blutschlinger-Ork");
        let groesse = Groesse::Gross;
        let erfahrenheit = Erfahrenheit::Erfahren;
        println!("Erzeuge Ork: {}", name);
        let ork = Einheit::gemeiner_ork(name,
             groesse, erfahrenheit);
        println!("Ork mit folgenden Werten erzeugt:\n
        {:?}", ork);
        ork
    }

    //Konstruktoren für die Rasse Menschen
    pub fn erstelle_rudolf_die_silberklinge() -> Einheit {
        let name = String::from("Rudolf die Silberklinge");
        let groesse = Groesse::Normalgross;
        let erfahrenheit = Erfahrenheit::Ausgebildet;
        println!("Erzeuge Mensch: {}", name);
        let mensch = Einheit::gemeiner_bandit(name,
             groesse, erfahrenheit);
        println!("Bandit mit folgenden Werten erzeugt:\n
        {:?}", mensch);
        mensch
    }

    //Konstruktoren für die Rasse Elfen
    pub fn erstelle_darion_der_geschickte() -> Einheit {
        let name = String::from("Darion der Geschickte");
        let groesse = Groesse::Gross;
        let erfahrenheit = Erfahrenheit::Erfahren;
        println!("Erzeuge Elf: {}", name);
        let elf = Einheit::elf_bogenschuetze(name,
             groesse, erfahrenheit);
        println!("Elf mit folgenden Werten erzeugt:\n
             {:?}", elf);
             elf
    }
