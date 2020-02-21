//Copyright: 2020 André Hodapp

/*
    Hierdrin ist die Einheit mit ihren Aktionen und Konstruktoren zu finden.
    Eine Einheit ist ein Lebewesen einer bestimmten Rasse/Volkes, die
    für den Kampf eingesetzt wird.
*/

pub mod aktionen; //Aktionen, also angreifen, heilen etc.
pub mod erstellen; //Konstruktoren, die ich anbiete (z.B. erstelle_blutschlinger_ork())
pub mod monsterdeck; //Deck bestehend aus Einheiten

use crate::Haeufigkeit; //für struct Einheit

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
pub struct Einheit {
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


//Hier sollen immutable getter
//inspiriert: https://stackoverflow.com/questions/35390615/writing-getter-setter-properties-in-rust
impl Einheit {
    pub fn einheitsbezeichnung(&self) -> &String {
        &self.einheitsbezeichnung
    }
    pub fn hp(&self) -> &i32 {
        &self.hp
    }
}
