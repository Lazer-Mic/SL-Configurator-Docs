# 4.10 Leistungsprofil

Das Leistungsprofil gibt Auskunft über die angeschlossene Last der Leuchte bei einer bestimmten Dimmstufe.
Das Leistungsprofil wird zur Berechnung des aktuellen Energieverbrauchs und des Leistungsmessers verwendet. Es wird vom Leuchtenhersteller vorgegeben und sollte im Feld nicht geändert werden.
Um auf die Leistungsprofil-Einstellungen zuzugreifen, muss zuerst der Servicemodus unter Ansicht > Servicemodus aktiviert werden.
Dann kann das Leistungsprofil im Bereich Leistung und Energie unter Leistungsprofil anpassen eingestellt werden.
Bei DALI 2.0 Treibern mit integrierter Energiemessung wird das Leistungsprofil nicht mehr berücksichtigt. Die Leistungsdaten werden vom Treiber abgefragt, sofern dies nicht deaktiviert ist. Dies sollte nur deaktiviert werden, wenn mehrere DALI 2.0 Treiber an einen Controller angeschlossen sind.
Da keine DALI-Adressierung stattfindet und alle DALI-Befehle als Broadcast gesendet werden, können die Werte bei mehreren Treibern nicht korrekt ausgelesen werden. 