# Gerätekonfiguration

Der Dimming Interface Ausgangstyp im Bereich Kontrollerausgang (Dimming- Interface) gibt an, auf welche Art und Weise der SLC- Controller mit dem Treiber/Dimm- Level kommuniziert.
Die Auswahlmöglichkeiten sind wie folgt:
Aus
Analog (0- 10V)
Analog Invertiert (10- 0V)
PWM
PWM Invertiert
DALI (Standard: Logarithmisch)
DALI (Linear)
Unter Dim- Verhalten können die Einblend- und Ausblendzeiten sowie der minimale Dimm- Schritt eingestellt werden.
Wenn DALI als Dimmschnittstelle ausgewählt ist, wird für das Einblenden immer die Ausblendzeit verwendet.
Gerät:
Unter Leuchtentypenname wird der Leuchtentyp angezeigt. Dieser Name identifiziert die Leuchte für die SL- Control Webplattform.
Leistungsprofil:
Ist an der Lichtsteuerung ein DALI2- Treiber angeschlossen, der über eine Wirkleistungsmessung verfügt, wird dieser vom Controller automatisch erkannt und die Leistungsdaten werden direkt vom Treiber übernommen.
Dies kann durch Deaktivieren der Funktion DALI2 EVG Leistungsmessung übernehmen (sofern vorhanden) verhindert werden. In diesem Fall wird weiterhin das definierte Leistungsprofil für den Stromverbrauch verwendet.