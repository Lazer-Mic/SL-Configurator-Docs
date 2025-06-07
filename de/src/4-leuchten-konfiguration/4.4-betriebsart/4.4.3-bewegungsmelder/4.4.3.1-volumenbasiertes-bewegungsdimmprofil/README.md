# 4.4.3.1 Volumenbasiertes Bewegungsdimmprofil

Ein volumenbasiertes Bewegungsdimmprofil wird typischerweise auf Straßenabschnitten verwendet, auf denen der Verkehr fast kontinuierlich ist und es daher nicht praktikabel ist, auf einzelne Bewegungen zu reagieren.

Ein Standort wird definiert, an dem das Verkehrsaufkommen (Anzahl der Fahrzeuge pro Stunde) gezählt und an alle Leuchten im relevanten Abschnitt weitergegeben wird (Mastersensor). Die Dimmung hängt ausschließlich von der Menge der erkannten Verkehrsbewegungen ab.

Folgende Einstellungen können konfiguriert werden:

Mastersensor:
Wenn dieses Kontrollkästchen aktiviert ist, wird das Verkehrsaufkommen an dieser Leuchte ermittelt und die Werte innerhalb des entsprechenden Beleuchtungsabschnitts verteilt.

Aktualisierungsintervall [min]:
Dies legt das Zeitintervall (in Minuten) fest, in dem das Verkehrsaufkommen ausgewertet werden soll.

Mastersensor-ID:
Die Mastersensor-ID definiert die Gruppe von Leuchten, die einem bestimmten Master zugeordnet werden sollen. Die ID muss innerhalb des Beleuchtungssystems eindeutig sein; eine bestimmte ID kann nur einmal pro Netzwerk verwendet werden.
Der Master sendet das Verkehrsaufkommen und seine ID als Broadcast an das Mesh-Netzwerk. Jede Leuchte mit der gleichen ID erkennt dies und verwendet den Wert.

Basishelligkeit:
Die Basishelligkeit definiert die Helligkeit der Leuchte bei 0 Fahrzeugen pro Stunde bis zum nächsten Schwellenwert.

Dimmstufen:
Die aktuell eingestellten Dimmstufen werden im Feld unter der Grafik links angezeigt.
Durch Klicken auf das grüne "+"-Symbol wird eine neue Dimmstufe hinzugefügt.
Der Wert "Bewegungsvolumen [1/h]" definiert immer die Untergrenze der aktuell ausgewählten Dimmstufe. 