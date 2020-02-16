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
u8: Hp,u8: Damage,u8: Verteidigung,
Enum Rasse: Rasse,Enum Klasse: Klasse,
Bool: Held, Enum Haeufigkeit: Haeufigkeit
*/

/*
Es gibt 4 Rassen, wobei Orks und Drachen
als böse und brutal gelten, wohingegen
Menschen und Elfen gut und naturnah
*/
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
enum Klasse {
    Nah, //Krieger kämpfen in vorderster Reihe
    Fern, //Bogenschützen direkt dahinter
    Magier, //Magier fast unantastbar
    Heiler, //ganz hinten, als Versorgung
}

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
u8: Hp,u8: Damage,u8: Verteidigung,
Enum Rasse: Rasse,Enum Klasse: Klasse,
Bool: Held, Enum Haeufigkeit: Haeufigkeit

Wobei Damage=Heilungsrate ist, falls Klasse
Heiler.

Helden müssen als erste Einheit ihrer Klasse
auf ihrem Feld besiegt werden.

Haeufigkeit ist ein Richtwert, wie selten
diese Einheit als Karte zu finden ist
*/
struct Einheit {
    hp: u8, //0-255 HP möglich
    damage: u8, //0-255 Damage/Heilung möglich
    verteidigung: u8, //0-255 Verteidigung möglich
    rasse: Rasse, //Rasse der Einheit
    klasse: Klasse, //Klasse der Einheit
    held: bool, //ist Held: true; kein Held: false
    haeufigkeit: Haeufigkeit, //wie haeufig zu finden
}

fn main() {
    println!("Hello, world!");
}
