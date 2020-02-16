# Wer_ist_staerker
Grundlage für ein Kartenspiel

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
Orks, Menschen, Drachen, Elfen

Orks -effektiv_gegen-> Menschen
-effektiv_gegen-> Drachen
-effektiv_gegen-> Elfen
-effektiv_gegen-> Orks


Dies soll die Grundlage für ein größeres
Kartenspiel sein. Deswegen hat eine Einheit
folgende Attribute:
u8: Hp,u8: Damage,u8: Verteidigung,
Enum Rasse: Rasse,Enum Klasse: Klasse,
Bool: Held, Enum Haeufigkeit: Haeufigkeit
