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
hinzu beim Angriff oder der Verteidigung,
wenn die Einheit effektiv gegen die
andere Einheit ist.
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
um darauf zu kämpfen.
Nah können im Feld 1 oder 2 platziert werden.
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
    legendaer,      // |
    episch,         // |
    sehr_selten,    // ^
    selten,         // | seltener
    leicht_selten,  // |
    haeufig,        // |
    sehr_haeufig,   // |
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
        Dies ist ein Konstruktor, welche für die
        hierbenötigte Aufgabe einen gewöhnlichen
        Ork-Krieger erstellt
    */
    fn gemeiner_Ork(hp: i32, damage: i32,
    verteidigung: i32) -> Einheit{
        Einheit {
            hp, damage, verteidigung, //übergebene Werte
            rasse: Rasse::Orks,
            klasse: Klasse::Nah,
            held: false,
            haeufigkeit: Haeufigkeit::selten //Ja, richtige Orks sind selten
        }
    }

    /*
        Dies ist ein Konstruktor, welche für die
        hierbenötigte Aufgabe einen gewöhnlichen
        Menschen-Banditen erstellt
    */
    fn gemeiner_Bandit(hp: i32, damage: i32,
    verteidigung: i32) -> Einheit{
        Einheit {
            hp, damage, verteidigung, //übergebene Werte
            rasse: Rasse::Menschen,
            klasse: Klasse::Nah,
            held: false,
            haeufigkeit: Haeufigkeit::sehr_haeufig //leider sind Banditen die Pest
        }
    }

}

/*
hier bitte ersmal alle Methoden
*/
impl Einheit {
    /*
        Einheit soll einen einfachen Angriff durchführen
        können. (Hier wird noch nicht Felder oder die Klassen betrachtet)
        Eine Einheit soll
        Schaden = Einheit:Angriff - Gegner:Verteidigung
        berechnen und falls Schaden > 0 ist, dem
        Gegner zufügen, falls Schaden < 0 ist, soll
        der Angreifer -Schaden viel HP verlieren,
        falls Schaden = 0, dann soll nichts passieren.
        In allen Fällen soll false zurückgegeben werden.
        (Später sollen Felder und Klassen berücksichtigt werden,
        weswegen dann manche Angriffe grundlegend nicht möglich
        sein sollten)
    */
    fn angriff(&mut self, gegner: &mut Einheit) -> bool {
        let schaden = self.damage - gegner.verteidigung;
        let mut error = false;
        match schaden {
            0 => println!("kein Schaden angerichtet"),
            1..=255 => gegner.hp -= schaden,
            -255..=-1 => self.hp += schaden,
            _ => error=true,
        }
        error
    }
}

fn main() {
    println!("Hello, world!");
}
