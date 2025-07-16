# Datenexport

![Datenexport](datenexport-1.png)

**Datenexport - esave Configurator Betriebsdatenauswertung**

Exportieren Sie umfassende Geräte- und Gerätegruppendaten der esave Lighting Controller in standardisierten Dateiformaten für professionelle Datenanalyse und Reporting. Diese mächtigen Exportfunktionen ermöglichen detaillierte Auswertungen von Energieverbrauch, Betriebszeiten, Bewegungsstatistiken und Effizienzanalysen.

## Hauptbereiche

### 1. Gerätegruppenverwaltung

![Datenexport](datenexport-2.png)

Die linke Seitenleiste zeigt die hierarchische Gerätestruktur:
- **Geräteliste**: Vollständige Übersicht aller konfigurierten esave Devices
- **Gruppenstatus**: Live-Betriebsmodus und Zeitsteuerung für jede Gerätegruppe
- **Auswahlkontext**: Markierung der aktuell gewählten Exportquelle
- **Betriebszeiten**: Detaillierte Laufzeitinformationen pro Gerät

### 2. Messwerte-Dashboard

Das zentrale Dashboard bietet Live-Monitoring der Gruppendaten:
- **Energieverbrauch**: Totaler kWh-Verbrauch der ausgewählten Gerätegruppe
- **Aktuelle Leistung**: Momentane kW-Leistungsaufnahme
- **Energieeinsparung**: Berechnete Einsparungen gegenüber konventioneller Beleuchtung
- **Steuerungsfunktionen**: Energiezähler-Reset und Datenexport-Tools

### 3. Export-Funktionalität

Der prominente "Energiemessdaten exportieren..." Button ermöglicht:
- **Datenauswahl**: Flexible Auswahl der zu exportierenden Parameter
- **Formatoptionen**: CSV (Excel-kompatibel) und XML-Standardformate
- **Zeitraumfilterung**: Auswahl spezifischer Zeitperioden für den Export
- **Vorschaufunktion**: Live-Tabellenvorschau der Exportdaten

## Exportdialog und Konfiguration

### Format-Auswahl

![Datenexport](datenexport-3.png)

**CSV (Excel) Format:**
- **Kompatibilität**: Vollständig kompatibel mit Microsoft Excel und LibreOffice
- **Trennzeichen**: Konfigurierbare Spaltentrennzeichen (Standard: Semikolon)
- **Encoding**: UTF-8 für internationale Zeichensätze
- **Anwendung**: Ideaal für Tabellenkalkulationen und statistische Analysen

**XML Format:**
- **Strukturierung**: Hierarchische Datenorganisation mit Metadaten
- **Validierung**: XML-Schema-konforme Datenstruktur
- **Integration**: Direkte Einbindung in Datenbank- und ERP-Systeme
- **Anwendung**: Automatisierte Datenverarbeitung und Systemintegration

### Gruppierungsoptionen

**Untergruppen exportieren:** ✅ Aktiviert
- **Hierarchische Struktur**: Einbeziehung aller Untergeräte der ausgewählten Gruppe
- **Vollständigkeit**: Exportierung der kompletten Gruppenhierarchie
- **Datenintegrität**: Konsistente Darstellung der Systemarchitektur

**Gruppenname-Einrückung:** `<SPACE>-<SPACE>`
- **Formatierung**: Visuelle Hierarchiedarstellung in Exportdateien
- **Anpassbar**: Konfigurierbare Einrückungszeichen
- **Lesbarkeit**: Verbesserte Übersichtlichkeit in Tabellenformaten

**Voller Gruppenname vs. Trennzeichen:**
- **Vollständiger Name**: Komplette Gruppenpfade für eindeutige Identifikation
- **Trennzeichen-Option**: Umgekehrter Schrägstrich [\] für Pfadtrennung
- **Flexibilität**: Anpassung an unterschiedliche Datenverarbeitungsanforderungen

### Einzelgeräte-Export

**Einzelne Geräte exportieren:** ✅ Aktiviert
- **Granularität**: Detaillierte Daten für jedes individuelle esave Device
- **Gerätenamenpräfix**: `<SPACE>-<SPACE>` für strukturierte Darstellung
- **Identifikation**: Eindeutige Zuordnung aller Gerätedaten

## Datenfelder und Parameter

### Grundlegende Exportdaten

**Energieverbrauch [kWh]:** ✅ Aktiviert
- **Funktion**: Gesamtenergieverbrauch seit Inbetriebnahme oder Reset
- **Einheit**: Kilowattstunden (kWh) mit 3 Dezimalstellen Genauigkeit
- **Anwendung**: Energiekostenanalyse und Verbrauchsoptimierung
- **Datenquelle**: Integrierte Leistungsmessung der esave Controller

**Aktuelle Leistungsaufnahme [kW]:** ❌ Deaktiviert
- **Funktion**: Momentane Leistungsaufnahme zum Exportzeitpunkt
- **Einheit**: Kilowatt (kW) als Momentaufnahme
- **Anwendung**: Lastanalyse und Spitzenlastüberwachung
- **Verfügbarkeit**: Optional aktivierbar für detaillierte Leistungsanalysen

**Gesparte Energie [kWh]:** ✅ Aktiviert
- **Funktion**: Berechnete Energieeinsparung gegenüber konventioneller Beleuchtung
- **Berechnung**: Differenz zwischen theoretischem Vollbetrieb und tatsächlichem Verbrauch
- **Einheit**: Kilowattstunden (kWh) Einsparung
- **ROI-Analyse**: Basis für Return-on-Investment Berechnungen

**Gesparte Energie [%]:** ✅ Aktiviert
- **Funktion**: Prozentuale Energieeinsparung als relative Effizienzkenngröße
- **Berechnung**: (Gesparte Energie / Theoretischer Vollverbrauch) × 100
- **Darstellung**: Prozentsatz mit 1-2 Dezimalstellen
- **Benchmarking**: Vergleichskennzahl für Effizienzoptimierung

**Datensatztyp:** ✅ Aktiviert
- **Funktion**: Klassifikation des Datensatzes (Gerätegruppe/Einzelgerät)
- **Werte**: "Gerätegruppe" oder spezifischer Gerätetyp
- **Strukturierung**: Organisatorische Zuordnung in Exportdateien
- **Filterung**: Basis für nachgelagerte Datenfilterung und -analyse

**Bewegungszähler:** ✅ Aktiviert
- **Funktion**: Anzahl detektierter Bewegungen seit Inbetriebnahme oder Reset
- **Datentyp**: Integer-Wert (Ganzzahl)
- **Sensor-Integration**: Daten von integrierten Radar- und PIR-Sensoren
- **Traffic-Analyse**: Grundlage für Verkehrsaufkommen-Analysen

### Gerätespezifische Datenfelder

**Gruppenpezifische Spalten:**

**Anzahl Geräte:** ❌ Deaktiviert
- **Funktion**: Anzahl der Geräte in jeder Gruppe
- **Anwendung**: Übersicht der Systemgröße und -komplexität
- **Verfügbarkeit**: Optional aktivierbar für Inventarverwaltung

**Gerätespezifische Spalten:**

**Gerätetyp:** ❌ Deaktiviert
- **Funktion**: Spezifikation des esave Controller-Modells
- **Beispiele**: "esave LUX", "esave DALI", "esave RADAR"
- **Anwendung**: Hardware-Inventar und Kompatibilitätsanalysen

**Geräteposition:** ❌ Deaktiviert
- **Funktion**: GPS-Koordinaten oder Installationsort
- **Format**: Latitude/Longitude oder Textbeschreibung
- **Anwendung**: Geografische Verteilungsanalysen und Wartungsplanung

**Adresse (Eindeutige ID):** ✅ Aktiviert
- **Funktion**: Eindeutige Geräteidentifikation (MAC-Adresse oder Geräte-ID)
- **Format**: Hexadezimale Adresse oder alphanumerische ID
- **Anwendung**: Eindeutige Zuordnung für Wartung und Support
- **Eindeutigkeit**: Garantiert unique identification für jedes Device

**Betriebszeit [h]:** ✅ Aktiviert
- **Funktion**: Gesamtbetriebsstunden seit Inbetriebnahme
- **Einheit**: Stunden (h) mit Dezimalstellen für Minuten-Genauigkeit
- **Anwendung**: Wartungsplanung und Lebensdaueranalyse
- **Zählung**: Kumulierte Einschaltzeit der Beleuchtung

**LED Betriebszeit [h]:** ❌ Deaktiviert
- **Funktion**: Spezifische Betriebsstunden der LED-Module
- **Unterscheidung**: Separate Zählung für LED-Komponenten vs. Gesamtsystem
- **Anwendung**: LED-spezifische Wartungsplanung und Degradationsanalyse
- **Verfügbarkeit**: Optional für detaillierte LED-Lebensdauer-Tracking

## Exportdatei-Optionen

### Dateibehandlung

**Spaltenüberschriften exportieren:** ✅ Aktiviert
- **Funktion**: Erste Zeile enthält beschreibende Spaltenheader
- **Vorteil**: Selbsterklärende Exportdateien ohne zusätzliche Dokumentation
- **Kompatibilität**: Optimiert für automatische Datenimport-Routinen
- **Lokalisierung**: Deutsche Spaltenüberschriften für lokale Anwendungen

**Spaltentrennzeichen:** Semikolon [;]
- **Standard**: Deutsches CSV-Format mit Semikolon-Trennung
- **Kompatibilität**: Optimiert für deutsche Excel-Versionen
- **Alternativen**: Umschaltbar auf Komma, Tabulator oder andere Trennzeichen
- **Lokalisierung**: Angepasst an regionale CSV-Konventionen

**Datei nach dem Export öffnen:** ✅ Aktiviert
- **Funktion**: Automatisches Öffnen der Exportdatei mit Standard-Anwendung
- **Benutzerfreundlichkeit**: Sofortige Verfügbarkeit der exportierten Daten
- **Anwendungsintegration**: Direkter Start von Excel oder Standard-CSV-Editor
- **Zeitersparnis**: Eliminierung manueller Datei-Navigation

### Vorschau-Funktionalität

**Live-Datenvorschau:**
Das untere Vorschaufenster zeigt die Exportstruktur in Echtzeit:

**Tabellenstruktur:**
- **Name**: Geräte- oder Gruppenbezeichnung
- **Datentyp**: Klassifikation (Gerätegruppe/Einzelgerät)
- **Geräteadresse**: Eindeutige Identifikation
- **Betriebszeit [h]**: Akkumulierte Betriebsstunden
- **Verbrauchte Energie [kWh]**: Totaler Energieverbrauch
- **Gesparte Energie [kWh]**: Berechnete Einsparungen

**Beispieldaten:**
- **"Neue Gruppe"**: Gerätegruppe mit 0,000h Betriebszeit und 0,000 kWh Verbrauch
- **Live-Update**: Vorschau aktualisiert sich automatisch bei Optionsänderungen
- **Datenvalidierung**: Sofortige Erkennung von Formatierungs- oder Strukturproblemen

## Anwendungsszenarien

### Energieverbrauchsanalyse

**Verbrauchsoptimierung:**
- **Baseline-Analyse**: Erfassung des aktuellen Energieverbrauchs pro Leuchte/Gruppe
- **Effizienzvergleiche**: Vergleich verschiedener Installationen oder Zeiträume
- **Kostenkalkulation**: Umrechnung kWh-Verbrauch in Energiekosten
- **ROI-Berechnung**: Amortisationsanalyse für esave-Investitionen

**Reporting und Compliance:**
- **Energieaudit**: Dokumentation für Energieeffizienz-Zertifizierungen
- **Nachhaltigkeitsreporting**: Daten für CSR-Berichte und Umweltzertifikate
- **Regulatorische Anforderungen**: Compliance mit Energieeffizienz-Vorschriften
- **Benchmarking**: Vergleich mit Industriestandards und Best Practices

### Wartungsplanung

**Preventive Maintenance:**
- **Betriebsstunden-Tracking**: Wartungsintervalle basierend auf tatsächlicher Nutzung
- **LED-Degradationsanalyse**: Vorhersage von LED-Austauschzyklen
- **Störungsfrüherkennung**: Identifikation anomaler Verbrauchsmuster
- **Kapazitätsplanung**: Vorhersage von Wartungsressourcen-Bedarf

**Lifecycle Management:**
- **Asset-Tracking**: Vollständige Gerätelebensdauer-Dokumentation
- **Ersatzteilplanung**: Prognose von Ersatzteilbedarfen
- **Upgrade-Planung**: Identifikation optimaler Modernisierungszeitpunkte
- **Budget-Planung**: Wartungskosten-Prognosen basierend auf Verbrauchstrends

### Verkehrs- und Nutzungsanalyse

**Traffic-Monitoring:**
- **Bewegungsstatistiken**: Analyse von Verkehrsaufkommen und -mustern
- **Stoßzeiten-Identifikation**: Erkennung von Peak-Traffic-Perioden
- **Saisonale Analysen**: Vergleich verschiedener Jahreszeiten oder Monate
- **Nutzungsoptimierung**: Anpassung der Beleuchtungssteuerung an tatsächliche Nutzung

**Sicherheitsanalysen:**
- **Aktivitätsmuster**: Identifikation ungewöhnlicher Bewegungsmuster
- **Sicherheitszone-Monitoring**: Überwachung kritischer Bereiche
- **Incident-Correlation**: Korrelation von Bewegungsdaten mit Sicherheitsereignissen
- **Präventive Sicherheitsmaßnahmen**: Datenbasierte Sicherheitsoptimierung

## Technische Spezifikationen

### Datenformat-Details

**CSV-Export-Spezifikationen:**
- **Encoding**: UTF-8 with BOM für Windows-Kompatibilität
- **Zeilenendings**: Windows CRLF (\r\n) für optimale Excel-Integration
- **Dezimaltrennzeichen**: Komma (,) entsprechend deutscher Lokalisation
- **Datumsformat**: DD.MM.YYYY HH:MM:SS für Zeitstempel

**XML-Export-Spezifikationen:**
- **Schema**: Proprietäres esave XML-Schema mit Validierung
- **Namespace**: esave.com/configurator/export/v1.0
- **Struktur**: Hierarchische Darstellung von Gruppen und Geräten
- **Metadaten**: Umfassende Geräteinformationen und Exportkontext

### Performance-Parameter

- **Exportgeschwindigkeit**: 1000 Geräte pro Sekunde (CSV), 500 Geräte pro Sekunde (XML)
- **Maximale Datenmenge**: 100.000 Geräte pro Exportvorgang
- **Speicherbedarf**: 1MB RAM pro 1000 exportierte Datensätze
- **Dateigröße**: ~50KB pro 1000 Geräte (CSV), ~100KB pro 1000 Geräte (XML)

### System-Integration

**Import-Kompatibilität:**
- **Microsoft Excel**: Alle Versionen ab Excel 2010
- **LibreOffice Calc**: Vollständige Kompatibilität
- **Google Sheets**: Direkte CSV-Import-Unterstützung
- **Datenbank-Systeme**: SQL Server, MySQL, PostgreSQL (via CSV/XML)

**API-Integration:**
- **REST-API**: Programmatischer Zugriff auf Exportfunktionen
- **Bulk-Export**: Automatisierte Massenexporte für große Installationen
- **Scheduled Exports**: Zeitgesteuerte automatische Datenexporte
- **Webhook-Integration**: Event-basierte Datenübertragung

## Vorteile des Datenexports

### Umfassende Transparenz

- **Vollständige Datenabdeckung**: Export aller relevanten Betriebs- und Verbrauchsdaten
- **Hierarchische Struktur**: Darstellung der kompletten Geräte- und Gruppenhierarchie
- **Historische Datenanalyse**: Langzeit-Tracking von Effizienz- und Verbrauchstrends
- **Multi-Format-Unterstützung**: Flexible Datenverwendung in verschiedenen Anwendungen

### Professionelle Datenanalyse

- **Standardisierte Formate**: CSV und XML für universelle Kompatibilität
- **Konfigurierbare Ausgabe**: Anpassbare Datenfelder und Formatierungsoptionen
- **Vorschaufunktion**: Validierung der Exportstruktur vor Dateierstellung
- **Automatisierungsfreundlich**: Geeignet für automatisierte Datenverarbeitungsprozesse

### Betriebsoptimierung

- **Evidenzbasierte Entscheidungen**: Datengetriebene Optimierung von Beleuchtungsanlagen
- **Kosteneffizienzmessung**: Quantifizierung von Energieeinsparungen und ROI
- **Wartungsoptimierung**: Präventive Wartung basierend auf tatsächlichen Betriebsdaten
- **Kontinuierliche Verbesserung**: Langfrist-Tracking für systematische Optimierungen

Diese Datenexport-Funktionalität bietet eine professionelle und umfassende Lösung für die Auswertung und Analyse von esave Lighting Controller Betriebsdaten mit flexiblen Exportoptionen und standardisierten Datenformaten. 