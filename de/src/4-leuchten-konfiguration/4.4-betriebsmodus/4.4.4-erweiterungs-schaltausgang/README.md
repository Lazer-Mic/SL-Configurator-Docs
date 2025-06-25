# Erweiterungs-Schaltausgang

Der Erweiterungs- Schaltausgang ist ein zusätzlicher Ausgang zur Dimmschnittstelle welcher konfiguriert werden kann.
> ℹ **Information:**  
> info
Ein- und Aus- Modus
Der Erweiterungsschaltausgang kann mit den Modi Aus und Ein in einen dauerhaften Zustand versetzt werden. Das bedeutet, dass der Erweiterungsschaltausgang unabhängig von der Dimmschnittstelle, den Betriebszeiten oder der Umgebungshelligkeit dauerhaft ein- oder ausgeschaltet werden kann.
Modi mit Abhängigkeit zum Dimm- Interface- Ausgang:
LED Treiber Modus
Die grau gestrichelten Linien zeigen die Stellen, bei denen die Helligkeit der Dimmschnittstelle ändern. D.h. Die Lampe beginnt dort hoch- oder runterzufahren. Im LED Treiber Modus wird Erweiterungsschaltausgang immer dann eingeschaltet, wenn die Dimmschnittstelle auf mehr als 0 % Helligkeit eingestellt ist.
Dimm- Interface- Modus / Invertierter Dimm- Interface- Modus
Beide Dimm- Interface- Modi reagieren sofort zu Beginn jeder Änderung, d.h. es wird nicht abgewartet, bis die Sollhelligkeit an der Dimmschnittstelle erreicht ist. Der invertierte Dimm- Interface Modus schaltet den Ausgang gleich wie das Dimm- Interface, aber mit invertierter Logik.
24 Stunden Modus
Wenn dieser Modus verwendet wird, könnte der Ausgang theoretisch zu jeder Zeit an gehen. Die einzige Voraussetzung, welche dann noch erfüllt werden muss, sind die Limiten der definierten Zeitschritte.
Ein ausserhalb Betriebszeiten
Wenn dieser Modus verwendet wird, schaltet sich der Ausgang grundsätzlich ein falls keine anderen Betriebsbedingungen wie z.B. Zeitschritte definiert sind. Dies im Gegensatz zu anderen Betriebsmodi, die die Helligkeit auf 0 Lux setzen. Dieser Modus kann nur zusammen mit einem anderen limitierenden Faktor verwendet werden.
Nach Umgebungshelligkeit
Der Modus nach Umgebungshelligkeit kann mit Zeitschritten kombiniert werden. Dies geht aber nur, wenn gleichzeitig auch die globale Zeit mit dem nach Zeit Modus eingeschränkt wird. Wenn diese Kombination verwendet wird, wird der Erweiterungsschaltausgang nur eingeschaltet, wenn die Umgebungshelligkeit dunkel genug ist, die globale Zeit eingehalten wurde und ein Zeitschritt für den Zeitpunkt definiert ist - ansonsten bleibt der Ausgang aus.
Wenn der Modus Nach Umgebungshelligkeit für sich allein verwendet wird, muss nur der Schwellwert der Umgebungshelligkeit unterschritten werden, um das Dimm- Profil zu aktivieren.
Nach Zeit
Die Limiten von Modus nach Zeit sind global interpretierte Werte. Nur innerhalb dieser Limiten kann der Ausgang eingeschaltet werden. Nebst diesen Limiten müssen noch Zeitschritte definiert werden. Erst wenn beide Voraussetzungen erfüllt sind, wird der Ausgang an gehen.
> ℹ **Information:**  
> info
