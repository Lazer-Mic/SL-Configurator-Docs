# Leistungsprofil

Das Leistungsprofil gibt Auskunft über die Anschlussleistung der Leuchte bei einer entsprechenden Dimmstufe.
Das Leistungsprofil wird für die Berechnung des aktuellen Energieverbrauchs und des Leistungsmessers verwendet. Es wird vom Leuchtenhersteller vorgegeben und sollte im Feld nicht verändert werden.
Um auf die Einstellungen des Leistungsprofils zugreifen zu können, muss zunächst der Service Modus unter Anzeige > Service Mode aktiviert werden.
Dann kann das Leistungsprofil im Bereich Leistung und Energie unter Leistungsprofil anpassen eingestellt werden.
Bei DALI- 2.0- Treibern mit integrierter Energiemessung wird das Leistungsprofil nicht mehr berücksichtigt. Die Leistungsdaten werden vom Treiber abgefragt, sofern dies nicht ausgeschaltet ist.  Dies sollte nur abgeschaltet werden, wenn mehrere DALI- 2.0- Treiber an einem Controller angeschlossen sind.
Da keine DALI- Adressierung stattfindet und alle DALI- Befehle als Broadcast gesendet werden, können die Werte bei mehreren Treibern nicht korrekt ausgelesen werden.