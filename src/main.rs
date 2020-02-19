//Copyright: 2020 André Hodapp
/*
Wer ist stärker, soll ein Vergleich sein
zwischen zwei Einheiten, die gegeneinander
kämpfen, wer wen wie viel Schaden zufügt.
Wenn eine Einheit angreift, dann soll
sein Angriff - die Verteidigung des Gegners
gerechnet werden, wie viel der Gegner Schaden
bekommen soll.
Falls der E:Angriff - G:Verteidigung < 0,
dann soll die angreifende Einheit genau
so viel Schaden bekommen.
Es wird abwechselnd angegriffen.
In der Erweiterung kommt noch ein Bonus
hinzu beim Angriff,
wenn die Einheit effektiv gegen die
andere Einheit ist.
Außerdem wenn die Einheit effektiv gegen die
andere ist, ist hat sie keine Verteidigung
und der Damage geht komplett auf die HP.
Es gibt vier Rassen:
Orks, Menschen, Drachen, Elf

Orks -effektiv_gegen-> Menschen
Menschen -effektiv_gegen-> Drachen
Drachen -effektiv_gegen-> Elfen
Elfen -effektiv_gegen-> Orks


Dies soll die Grundlage für ein größeres
Kartenspiel sein. Deswegen hat eine Einheit
folgende Attribute:
i32: Hp,i32: Damage,i32: Verteidigung,
Enum Rasse: Rasse,Enum Klasse: Klasse,
Bool: Held, Enum Haeufigkeit: Haeufigkeit
*/

/*
Es gibt 4 Rassen, wobei Orks und Drachen
als böse und brutal gelten, wohingegen
Menschen und Elfen gut und naturnah
*/
#[derive(Debug)]
enum Rasse {
    Orks, //-effektiv_gegen-> Menschen
    Menschen, //-effektiv_gegen-> Drachen
    Drachen, //-effektiv_gegen-> Elfen
    Elfen, //-effektiv_gegen-> Orks
}

/*
Es gibt zwei Felder (Schlachtfelder),
Nah können im Feld 1 oder 2 platziert werden.
um darauf zu kämpfen.
Fern, Magier und Heiler sind zwischen den
beiden Feldern zu finden.

Fern kann immer Nah angreifen;
Nah können nur andere Nah angreifen,
außer alle feindlichen Nah in dem Feld sind
tot, dann können sie auch Fern angreifen.
Falls auf beiden Feldern alle Nah des
Gegners tot sind, dann können Nah endlich
auch den Magier, bzw. den Heiler angreifen

Spielfeld-Aufbau:
             |
  [Nah, Nah] | [Nah, Nah]
        [Fern, Fern]
      [Magier/Heiler]
*/
#[derive(Debug)]
enum Klasse {
    Nah, //Krieger kämpfen in vorderster Reihe
    Fern, //Bogenschützen direkt dahinter
    Magier, //Magier fast unantastbar
    Heiler, //ganz hinten, als Versorgung
}

#[derive(Debug)]
enum Haeufigkeit {  // ^
    Legendaer,      // |
    Episch,         // |
    SehrSelten,    // ^
    Selten,         // | seltener
    LeichtSelten,  // |
    Haeufig,        // |
    SehrHaeufig,   // |
}

/*
Eineheit hat folgende Attribute:
i32: Hp,i32: Damage,i32: Verteidigung,
Enum Rasse: Rasse,Enum Klasse: Klasse,
Bool: Held, Enum Haeufigkeit: Haeufigkeit

Wobei Damage=Heilungsrate ist, falls Klasse
Heiler.

Helden müssen als erste Einheit ihrer Klasse
auf ihrem Feld besiegt werden.

Haeufigkeit ist ein Richtwert, wie selten
diese Einheit als Karte zu finden ist
*/
#[derive(Debug)]
struct Einheit {
    einheitsbezeichnung: String, //Die Bezeichnung der Einheit
    hp: i32, //logisch soll nur 1-255 HP möglich sein
    damage: i32, //logisch soll nur 0-255 Damage/Heilung möglich sein
    verteidigung: i32, //logisch soll nur 0-255 Verteidigung möglich sein
    rasse: Rasse, //Rasse der Einheit
    klasse: Klasse, //Klasse der Einheit
    held: bool, //ist Held: true; kein Held: false
    haeufigkeit: Haeufigkeit, //wie haeufig zu finden
}


/*
In diesem Block nur Konstruktoren für Einheiten
*/
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
        Dies ist ein Konstruktor, welche für die
        hierbenötigte Aufgabe einen gewöhnlichen
        Ork-Krieger erstellt
    */
    fn gemeiner_ork(einheitsbezeichnung: String,
        hp: i32, damage: i32)
         -> Einheit{
             let rasse = Rasse::Orks;
             let klasse = Klasse::Nah;
             let haeufigkeit = Haeufigkeit::Selten; //Ja, richtige Orks sind selten
             let verteidigung = Einheit::verteidigung_berechnen(&rasse,
                 &klasse, &haeufigkeit); //berechne Rüstung
        Einheit {
            einheitsbezeichnung, hp, damage, //übergebene Werte
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
    fn gemeiner_bandit(einheitsbezeichnung: String,
        hp: i32, damage: i32)
         -> Einheit{
             let rasse =  Rasse::Menschen;
             let klasse = Klasse::Nah;
             let haeufigkeit = Haeufigkeit::Haeufig; //leider sind Banditen die Pest
             let verteidigung = Einheit::verteidigung_berechnen(&rasse,
                 &klasse, &haeufigkeit); //berechne Rüstung
        Einheit {
            einheitsbezeichnung, hp, damage,
            verteidigung, //übergebene Werte
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
    fn elf_bogenschuetze(einheitsbezeichnung: String,
        hp: i32, damage: i32)
         -> Einheit{
             let rasse = Rasse::Elfen;
             let klasse = Klasse::Fern;
             let haeufigkeit = Haeufigkeit::LeichtSelten; //Elfen waren früher nur
                                               //Legenden,jetzt entschwinden sie
                                               //immer mehr den Ruf der Fabelwesen
             let verteidigung = Einheit::verteidigung_berechnen(&rasse,
                 &klasse, &haeufigkeit); //berechne Rüstung
        Einheit {
            einheitsbezeichnung, hp, damage,
            verteidigung, //übergebene Werte
            rasse,
            klasse,
            held: false,
            haeufigkeit,
        }
    }

}

//___________________________
//Verschiedene Einheiten
    //Orks
    fn erstelle_blutschlinger_ork() -> Einheit {
        let ork_name = String::from("Blutschlinger-Ork");
        let ork_hp = 50;
        let ork_damage = 20;
        println!("Erzeuge Ork: {}", ork_name);
        let ork = Einheit::gemeiner_ork(ork_name,
            ork_hp, ork_damage);
        println!("Ork mit folgenden Werten erzeugt:\n
        {:?}", ork);
        ork
    }

    //Menschen
    fn erstelle_rudolf_die_silberklinge() -> Einheit {
        let mensch_name = String::from("Rudolf die Silberklinge");
        let mensch_hp = 40;
        let mensch_damage = 16;
        println!("Erzeuge Mensch: {}", mensch_name);
        let mensch = Einheit::gemeiner_bandit(mensch_name,
            mensch_hp, mensch_damage);
        println!("Bandit mit folgenden Werten erzeugt:\n
        {:?}", mensch);
        mensch
    }

    //Elfen
    fn erstelle_darion_der_geschickte() -> Einheit {
        let elfen_name = String::from("Darion der Geschickte");
        let elfen_hp = 20;
        let elfen_damage = 18;
        println!("Erzeuge Elf: {}", elfen_name);
        let elf = Einheit::elf_bogenschuetze(elfen_name,
             elfen_hp,elfen_damage);
        println!("Elf mit folgenden Werten erzeugt:\n
             {:?}", elf);
             elf
    }

//___________________________

/*
hier bitte ersmal alle Methoden
*/
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
    fn ist_effektiv(&mut self, gegner: &Einheit) -> bool {
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
    fn angriff(&mut self, gegner: &mut Einheit) -> bool {
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

fn main() {
    test_kaempfe();
}

fn test_kaempfe(){
    println!("Erstes Szenario\n");
    test_kampf_getrennt();

    println!("\nZweitest Szenario\n");
    test_kampf_gemeinsam();
}

fn test_kampf_getrennt(){
    test_kampf_mensch_vs_ork();
    println!("\nAm nächsten Tag trifft der Ork auf einen Elfen...\n");
    test_kampf_elf_vs_ork();
}

fn test_kampf_gemeinsam(){
    let mut ork = erstelle_blutschlinger_ork();
    let mut mensch = erstelle_rudolf_die_silberklinge();

    println!("\n----Starte Kampf----\n");
    kampf(&mut ork,&mut mensch);
    println!("\n-----Ende Kampf-----\n");

    println!("Oh, Nein!!! Ein Elf taucht auf...");
    let mut elf = erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf(&mut ork,&mut elf);
    println!("\n-----Ende Kampf-----\n");
}

fn test_kampf_mensch_vs_ork() {
    let mut ork = erstelle_blutschlinger_ork();
    let mut mensch = erstelle_rudolf_die_silberklinge();

    println!("\n----Starte Kampf----\n");
    kampf(&mut ork,&mut mensch);
    println!("\n-----Ende Kampf-----\n");
}

fn test_kampf_elf_vs_ork() {
    let mut ork = erstelle_blutschlinger_ork();
    let mut elf = erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf(&mut ork,&mut elf);
    println!("\n-----Ende Kampf-----\n");
}

fn check_kampf(error: bool, ork: &Einheit, mensch: &Einheit) -> bool{
    if error||ork.hp <1||mensch.hp <1 {
        if error {
            println!("Fehler!");
        } else if ork.hp<1 {
            println!("Die Einheit: {} hat gegen {} gewonnen",
             mensch.einheitsbezeichnung, ork.einheitsbezeichnung);
        } else if mensch.hp<1 {
            println!("Die Einheit: {} hat gegen {} gewonnen",
             ork.einheitsbezeichnung,mensch.einheitsbezeichnung);
        }
        return true; //Spiel wird beendet
    }
    return false; //Spiel läuft weiter
}

//Kampf bis zum Tode
fn kampf (ork: &mut Einheit, mensch: &mut Einheit){
    //kämpfe solange bis einer stirbt
    loop {
        let error = ork.angriff(mensch);
        match check_kampf(error, ork, mensch) {
            false => println!("Beide stehen noch immer wacker"),
            true => {
                println!("Kampf vorbei");
                break;
            },
        }
        let error = mensch.angriff(ork);
        match check_kampf(error, ork, mensch) {
            false => println!("Beide stehen noch immer wacker"),
            true => {
                println!("Kampf vorbei");
                break;
            },
        }
    }
    println!("Der Sieger stolziert anmutig vom Schlachfeld");
}
