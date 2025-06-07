# 4.4.4 Erweiterungsschalter-Ausgang

Der Erweiterungsschalter-Ausgang ist ein zusätzlicher Ausgang zur Dimm-Schnittstelle, der konfiguriert werden kann.

> ℹ️ **Information:**  
> info

Ein-/Ausschaltmodus:
Der Erweiterungsschalter-Ausgang kann mit den Modi Aus und Ein auf einen permanenten Zustand gesetzt werden. Das bedeutet, dass der Erweiterungsschalter-Ausgang unabhängig von der Dimm-Schnittstelle, den Betriebszeiten oder der Umgebungshelligkeit permanent ein- oder ausgeschaltet werden kann.

Von der Dimm-Schnittstelle abhängige Modi:

LED-Treiber-Modus:
Die grauen gestrichelten Linien zeigen die Punkte, an denen sich die Helligkeit der Dimm-Schnittstelle ändert. Das heißt, dort beginnt die Lampe zu dimmen oder zu heller zu werden. Im LED-Treiber-Modus wird der Erweiterungsschalter-Ausgang immer eingeschaltet, wenn die Dimm-Schnittstelle auf mehr als 0% Helligkeit eingestellt ist.

Dimm-Schnittstellen-Modus / Invertierter Dimm-Schnittstellen-Modus:
Beide Dimm-Schnittstellen-Modi reagieren sofort bei Beginn einer Änderung, d.h. sie warten nicht, bis die Zielhelligkeit an der Dimm-Schnittstelle erreicht ist. Der invertierte Dimm-Schnittstellen-Modus schaltet den Ausgang auf die gleiche Weise wie die Dimm-Schnittstelle, aber mit invertierter Logik.

24-Stunden-Modus:
Wenn dieser Modus verwendet wird, könnte der Ausgang theoretisch zu jeder Zeit eingeschaltet werden. Die einzige verbleibende Anforderung sind die Grenzen der definierten Zeitschritte.

Außerhalb der Betriebszeiten:
Wenn dieser Modus verwendet wird, wird der Ausgang grundsätzlich eingeschaltet, wenn keine anderen Betriebsbedingungen wie Zeitschritte definiert sind. Dies steht im Gegensatz zu anderen Betriebsmodi, die die Helligkeit auf 0 Lux setzen. Dieser Modus kann nur zusammen mit einem anderen begrenzenden Faktor verwendet werden.

Nach Umgebungshelligkeit:
Der Modus nach Umgebungshelligkeit kann mit Zeitschritten kombiniert werden. Dies funktioniert jedoch nur, wenn die globale Zeit auch mit dem Modus nach Zeit eingeschränkt ist. Bei dieser Kombination wird der Erweiterungsschalter-Ausgang nur eingeschaltet, wenn die Umgebungshelligkeit dunkel genug ist, die globale Zeit erfüllt ist und ein Zeitschritt für die Zeit definiert ist - ansonsten bleibt der Ausgang aus.
Wenn der Modus nach Umgebungshelligkeit allein verwendet wird, muss nur der Schwellenwert der Umgebungshelligkeit unterschritten werden, um das Dimmprofil zu aktivieren.

Nach Zeit:
Die Grenzen des Modus nach Zeit werden global interpretiert. Der Ausgang kann nur innerhalb dieser Grenzen eingeschaltet werden. Zusätzlich zu diesen Grenzen müssen auch Zeitschritte definiert werden. Nur wenn beide Anforderungen erfüllt sind, wird der Ausgang eingeschaltet.

> ℹ️ **Information:**  
> info 