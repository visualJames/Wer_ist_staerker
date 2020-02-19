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
    Dies soll die Groesse einer Einheit
    darstellen.
    Ab Gigantisch soll Flächenschaden der Fall sein,
    also es werden alle Einheiten des Feldes angegriffen
*/
#[derive(Debug)]
enum Groesse {
    Winzig, //Goblin
    Klein, //Kobold
    Normalgross, //Menschen
    Gross, //Orks und Elfen
    Riesig, //Trolle und Steingolems
    Gigantisch, //Baumriesen und Drachen
    Monstrositaet, //Antike Steinriesen, die so groß wie Berge sind
}

enum Erfahrenheit {
    Unerfahren, //Anfänger, jung
    Ausgebildet, //hat zumindest ein wenig Erfahrung
    Erfahren, //viele Schlachten
    Veteran, //eben ein Veteran
    Meister, //beherscht seine Kunst perfekt
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
    groesse: Groesse, //wie groß ist die Einheit
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

//___________________________
//Verschiedene Einheiten
    //Orks
    fn erstelle_blutschlinger_ork() -> Einheit {
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

    //Menschen
    fn erstelle_rudolf_die_silberklinge() -> Einheit {
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

    //Elfen
    fn erstelle_darion_der_geschickte() -> Einheit {
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
/*
    println!("Erstes Szenario\n");
    test_kampf_getrennt();

    println!("\nZweitest Szenario\n");
    test_kampf_gemeinsam();
*/
    println!("\nDrittes Szenario");
    test_zwei_gegen_einen();
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
    kampf(&mut mensch,&mut ork);
    println!("\n-----Ende Kampf-----\n");

    println!("Oh, Nein!!! Ein Elf taucht auf...");
    let mut elf = erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf(&mut elf,&mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn test_zwei_gegen_einen(){
    let mut ork = erstelle_blutschlinger_ork();
    let mut mensch = erstelle_rudolf_die_silberklinge();
    let mut elf = erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf_zwei_gegen_einen(&mut mensch,&mut elf, &mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn test_kampf_mensch_vs_ork() {
    let mut ork = erstelle_blutschlinger_ork();
    let mut mensch = erstelle_rudolf_die_silberklinge();

    println!("\n----Starte Kampf----\n");
    kampf(&mut mensch,&mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn test_kampf_elf_vs_ork() {
    let mut ork = erstelle_blutschlinger_ork();
    let mut elf = erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf(&mut elf,&mut ork);
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

//Kampf bis zum Tode
fn kampf_zwei_gegen_einen (ork: &mut Einheit, goblin: &mut Einheit, mensch: &mut Einheit){
    //kämpfe solange bis einer stirbt
    loop {
        if ork.hp >0{
            let error = ork.angriff(mensch);
            match check_kampf(error, ork, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    println!("Kampf vorbei");
                    break;
                },
            }
        }
        if goblin.hp >0 {
            let error = goblin.angriff(mensch);
            match check_kampf(error, goblin, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    println!("Kampf vorbei");
                    break;
                },
            }
        }
        if ork.hp >0 && ((!mensch.ist_effektiv(goblin) && mensch.ist_effektiv(ork))
         || goblin.hp<=0) {
            println!("{}: {}",goblin.einheitsbezeichnung, mensch.ist_effektiv(goblin));
            println!("{}: {}",ork.einheitsbezeichnung, mensch.ist_effektiv(ork));
            let error = mensch.angriff(ork);
            match check_kampf(error, ork, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    if error||mensch.hp<=0||goblin.hp <=0 {
                        println!("Kampf vorbei");
                        break;
                    }else{
                        println!("Endlich ist {} tot mit {} viel HP",
                        ork.einheitsbezeichnung, ork.hp);
                    }
                },
            }
        } else {
            let error = mensch.angriff(goblin);
            match check_kampf(error, goblin, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    if error||mensch.hp<=0||ork.hp <=0 {
                        println!("Kampf vorbei");
                        break;
                    }else{
                        println!("Endlich ist {} tot mit {} viel HP",
                        goblin.einheitsbezeichnung, goblin.hp);
                    }
                },
            }
        }

    }
    println!("Der Sieger stolziert anmutig vom Schlachfeld");
}

//todo! Die Klasse Einheit mit ihren Methoden und die Tests mit print trennen
//todo! Richtige Tests schreiben, ob die Logik so funktioniert wie gewollt
//todo! Mehr Einheiten für Ork erzeugen. Ork-Einheiten sollen schon ein wenig erstellt werden
