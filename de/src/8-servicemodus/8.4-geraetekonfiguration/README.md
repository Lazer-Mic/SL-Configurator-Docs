# 8.4 Gerätekonfiguration

Der Dimm-Interface-Ausgangstyp im Controller-Ausgang (Dimm-Interface) gibt an, wie der SLC-Controller mit dem Treiber/Dimm-Level kommuniziert.
Die Optionen sind wie folgt:
Aus
Analog (0-10V)
Analog Invertiert (10-0V)
PWM
PWM Invertiert
DALI (Standard: Logarithmisch)
DALI (Linear)
Unter Dimmverhalten können die Ein- und Ausblendzeiten sowie der minimale Dimmstufenschritt eingestellt werden.
Wenn DALI als Dimm-Interface ausgewählt ist, wird für das Einblenden immer die Ausblendzeit verwendet.
Gerät:
Unter Leuchtentyp-Name wird der Leuchtentyp angezeigt. Dieser Name identifiziert die Leuchte für die SL-Control Webplattform.
Leistungsprofil:
Wenn ein DALI2-Treiber an die Lichtsteuerung angeschlossen ist, der eine Leistungsmessung ermöglicht, wird dieser automatisch vom Controller erkannt und die Leistungsdaten werden direkt vom Treiber übernommen.
Dies kann durch Deaktivieren der DALI2 EVG-Leistungsmessung (falls verfügbar) verhindert werden. In diesem Fall wird weiterhin das definierte Leistungsprofil für den Leistungsverbrauch verwendet. 