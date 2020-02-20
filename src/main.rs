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

mod einheit; //Wir wollen mit Einheiten arbeiten

use einheit::Einheit; //struct Einheit direkt benutzbar



/*
    Die Häufigkeit einer Karte
*/
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
    let mut ork = crate::einheit::erstellen::erstelle_blutschlinger_ork();
    let mut mensch = crate::einheit::erstellen::erstelle_rudolf_die_silberklinge();

    println!("\n----Starte Kampf----\n");
    kampf(&mut mensch,&mut ork);
    println!("\n-----Ende Kampf-----\n");

    println!("Oh, Nein!!! Ein Elf taucht auf...");
    let mut elf = crate::einheit::erstellen::erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf(&mut elf,&mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn test_zwei_gegen_einen(){
    let mut ork = crate::einheit::erstellen::erstelle_blutschlinger_ork();
    let mut mensch = crate::einheit::erstellen::erstelle_rudolf_die_silberklinge();
    let mut elf = crate::einheit::erstellen::erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf_zwei_gegen_einen(&mut mensch,&mut elf, &mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn test_kampf_mensch_vs_ork() {
    let mut ork = crate::einheit::erstellen::erstelle_blutschlinger_ork();
    let mut mensch = crate::einheit::erstellen::erstelle_rudolf_die_silberklinge();

    println!("\n----Starte Kampf----\n");
    kampf(&mut mensch,&mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn test_kampf_elf_vs_ork() {
    let mut ork = crate::einheit::erstellen::erstelle_blutschlinger_ork();
    let mut elf = crate::einheit::erstellen::erstelle_darion_der_geschickte();

    println!("\n----Starte Kampf----\n");
    kampf(&mut elf,&mut ork);
    println!("\n-----Ende Kampf-----\n");
}

fn check_kampf(error: bool, ork: &Einheit, mensch: &Einheit) -> bool{
    if error||*ork.hp() <1||*mensch.hp() <1 {
        if error {
            println!("Fehler!");
        } else if *ork.hp()<1 {
            println!("Die Einheit: {} hat gegen {} gewonnen",
             mensch.einheitsbezeichnung(), ork.einheitsbezeichnung());
        } else if *mensch.hp()<1 {
            println!("Die Einheit: {} hat gegen {} gewonnen",
             ork.einheitsbezeichnung(),mensch.einheitsbezeichnung());
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
        if *ork.hp() >0{
            let error = ork.angriff(mensch);
            match check_kampf(error, ork, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    println!("Kampf vorbei");
                    break;
                },
            }
        }
        if *goblin.hp() >0 {
            let error = goblin.angriff(mensch);
            match check_kampf(error, goblin, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    println!("Kampf vorbei");
                    break;
                },
            }
        }
        if *ork.hp() >0 && ((!mensch.ist_effektiv(goblin) && mensch.ist_effektiv(ork))
         || *goblin.hp()<=0) {
            println!("{}: {}",goblin.einheitsbezeichnung(), mensch.ist_effektiv(goblin));
            println!("{}: {}",ork.einheitsbezeichnung(), mensch.ist_effektiv(ork));
            let error = mensch.angriff(ork);
            match check_kampf(error, ork, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    if error||*mensch.hp()<=0||*goblin.hp() <=0 {
                        println!("Kampf vorbei");
                        break;
                    }else{
                        println!("Endlich ist {} tot mit {} viel HP",
                        ork.einheitsbezeichnung(), *ork.hp());
                    }
                },
            }
        } else {
            let error = mensch.angriff(goblin);
            match check_kampf(error, goblin, mensch) {
                false => println!("Beide stehen noch immer wacker"),
                true => {
                    if error||*mensch.hp()<=0||*ork.hp() <=0 {
                        println!("Kampf vorbei");
                        break;
                    }else{
                        println!("Endlich ist {} tot mit {} viel HP",
                        goblin.einheitsbezeichnung(), *goblin.hp());
                    }
                },
            }
        }

    }
    println!("Der Sieger stolziert anmutig vom Schlachfeld");
}

//todo! Richtige Tests schreiben, ob die Logik so funktioniert wie gewollt
//todo! Mehr Einheiten für Ork erzeugen. Ork-Einheiten sollen schon ein wenig erstellt werden
