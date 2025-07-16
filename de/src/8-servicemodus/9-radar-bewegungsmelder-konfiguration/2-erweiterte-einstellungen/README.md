# Erweiterte Einstellungen

![Erweiterte-Einstellungen](erweiterte-einstellungen.png)

**Erweiterte Einstellungen - Radar-Bewegungssensor-Konfiguration**

Konfigurieren Sie spezialisierte Parameter des Radar-Bewegungssensors für optimale Leistung in verschiedenen Umgebungen. Diese erweiterten Einstellungen ermöglichen die Feinabstimmung von Klassifizierungsalgorithmen, Detektionsparametern und installationsabhängigen Faktoren.

*Der Servicemodus kann unter dem Menüpunkt Anzeige > Service Mode aktiviert werden.*

## Hauptbereiche

### 1. Konfigurationsnavigation

Die linke Navigationsleiste bietet Zugang zu allen Konfigurationsbereichen:
- **Allgemein**: Grundlegende Detektionseinstellungen
- **Erweiterte Einstellungen**: Spezialisierte Parameter für Experten (aktuell aktiv)
- **Detektionsrichtungen**: Richtungsabhängige Erkennungseinstellungen
- **Automatische Rauschunterdrückung**: Störungsfilterung und Umgebungsanpassung
- **Klassifizierungslimiten**: Schwellenwerte für Objektklassifizierung

### 2. Erweiterte Konfiguration

Der Hauptbereich zeigt spezialisierte Einstellungen:
- **Klassierungsmodus**: Algorithmus-Auswahl für Objekterkennung
- **Amplitude**: Signalstärke-Parameter für Detektionsempfindlichkeit
- **Sensorhöhe**: Installationshöhe für geometrische Berechnungen

## Klassierungsmodus

### Berechne Länge nach Zeit (Aktuell ausgewählt)

**Dropdown-Auswahl:** "Berechne Länge nach Zeit"
- **Funktionsweise**: Objektlänge wird durch Geschwindigkeit und Verweilzeit berechnet
- **Formel**: Länge = Geschwindigkeit × Durchgangszeit
- **Vorteile**: Präzise Längenbestimmung bei bekannter Geschwindigkeit
- **Anwendung**: Optimal für Verkehrssituationen mit konstanten Geschwindigkeiten

**Berechnungsprinzip:**
- **Zeitmessung**: Erfassung der Zeit, die ein Objekt im Detektionsbereich verbringt
- **Geschwindigkeitskorrelation**: Kombination mit gemessener Doppler-Geschwindigkeit
- **Längenextrapolation**: Mathematische Ableitung der Objektlänge
- **Geometrische Korrektur**: Berücksichtigung des Durchgangsitswinkels

### Alternative Klassierungsmodi

**Weitere verfügbare Modi (Dropdown-Optionen):**
- **Berechne Länge nach Amplitude**: Objektlänge basierend auf Radar-Querschnitt
- **Statische Klassifizierung**: Feste Zuordnung basierend auf Geschwindigkeitsbereichen
- **Machine Learning Modus**: KI-basierte Objekterkennung
- **Hybrid-Klassifizierung**: Kombination verschiedener Erkennungsalgorithmen

**Auswahlkriterien:**
- **Verkehrsdichte**: Hohe Dichte → Zeit-basiert, niedrige Dichte → Amplitude-basiert
- **Geschwindigkeitsvariation**: Konstante Geschwindigkeit → Zeit-basiert
- **Objektvielfalt**: Vielfältige Objekte → Machine Learning
- **Installationsumgebung**: Komplex → Hybrid-Klassifizierung

## Amplitude

### Signalstärke-Konfiguration

**Aktueller Wert:** 2000 (Einstellbereich sichtbar)
- **Parameter**: Radar-Amplituden-Schwellenwert für Objekterkennung
- **Einheit**: Relative Einheiten (0-4095)
- **Standardwert**: 2000 (mittlere Empfindlichkeit)
- **Anpassungsbereich**: Grüner Slider für einfache Einstellung

**Funktionsweise:**
- **Detektionsschwelle**: Minimale Signalstärke für Objekterkennung
- **Empfindlichkeitssteuerung**: Höhere Werte = weniger empfindlich
- **Störungsfilterung**: Ausblendung schwacher Reflexionen
- **Reichweitenoptimierung**: Anpassung an gewünschte Detektionsdistanz

### Amplitude-Optimierung

**Niedrige Amplitude (500-1500):**
- **Anwendung**: Erkennung kleiner Objekte (Personen, Tiere)
- **Vorteil**: Hohe Empfindlichkeit für schwache Reflexionen
- **Nachteil**: Mögliche Störungen durch Umwelteinflüsse
- **Empfehlung**: Fußgängerbereiche, ruhige Umgebungen

**Mittlere Amplitude (1500-2500):**
- **Anwendung**: Ausgewogene Erkennung verschiedener Objekttypen
- **Vorteil**: Guter Kompromiss zwischen Empfindlichkeit und Störresistenz
- **Standard**: Für die meisten Anwendungen geeignet
- **Empfehlung**: Mischverkehr, städtische Bereiche

**Hohe Amplitude (2500-4000):**
- **Anwendung**: Fokus auf große Objekte (Fahrzeuge, LKW)
- **Vorteil**: Minimale Störungen, stabile Detektion
- **Nachteil**: Kleine Objekte werden möglicherweise übersehen
- **Empfehlung**: Autobahnen, Industriegebiete

## Sensorhöhe

### Installationsparameter

**Aktueller Wert:** 8 m (Installationshöhe)
- **Parameter**: Höhe des Radarsensors über dem Boden
- **Einheit**: Meter (m)
- **Standardwert**: 8 m (typische Straßenbeleuchtungshöhe)
- **Anpassungsbereich**: 2-15 m (je nach Installation)

**Bedeutung für die Detektion:**
- **Geometrische Berechnungen**: Korrektur der Entfernungsmessung
- **Winkelkompensation**: Anpassung der Erfassungsgeometrie
- **Reichweitenoptimierung**: Maximale Detektionsdistanz
- **Klassifizierungsgenauigkeit**: Präzise Objektgrößenberechnung

### Höhen-Optimierung

**Niedrige Installation (2-4 m):**
- **Anwendung**: Fußgängerbereiche, Parkplätze, Eingangsbereiche
- **Vorteil**: Hohe Detektionsgenauigkeit für kleine Objekte
- **Geometrie**: Steiler Erfassungswinkel, kleinerer Bereich
- **Empfehlung**: Präzise Personenerkennung in begrenzten Bereichen

**Mittlere Installation (4-8 m):**
- **Anwendung**: Stadtstraßen, Wohngebiete, Gewerbeflächen
- **Vorteil**: Ausgewogene Erfassung verschiedener Objekttypen
- **Geometrie**: Optimaler Kompromiss zwischen Bereich und Genauigkeit
- **Standard**: Für die meisten städtischen Anwendungen

**Hohe Installation (8-15 m):**
- **Anwendung**: Autobahnen, Hauptverkehrsstraßen, große Freiflächen
- **Vorteil**: Große Erfassungsbereiche, Überblick über weite Flächen
- **Geometrie**: Flacher Winkel, erweiterte Reichweite
- **Empfehlung**: Verkehrsüberwachung, große Parkplätze

## Erweiterte Konfigurationsstrategien

### Umgebungsanpassung

**Städtische Umgebung:**
- **Klassierungsmodus**: Berechne Länge nach Zeit
- **Amplitude**: 1800-2200 (mittlere Empfindlichkeit)
- **Sensorhöhe**: 6-8 m (Standardmastenhöhe)
- **Optimierung**: Ausgewogene Erkennung aller Verkehrsteilnehmer

**Ländliche Umgebung:**
- **Klassierungsmodus**: Berechne Länge nach Amplitude
- **Amplitude**: 1200-1800 (höhere Empfindlichkeit)
- **Sensorhöhe**: 8-10 m (erhöhte Reichweite)
- **Optimierung**: Erfassung auch entfernter oder kleiner Objekte

**Industrieumgebung:**
- **Klassierungsmodus**: Statische Klassifizierung
- **Amplitude**: 2200-3000 (reduzierte Störanfälligkeit)
- **Sensorhöhe**: 10-12 m (Überblick über große Flächen)
- **Optimierung**: Robuste Erkennung bei elektromagnetischen Störungen

### Performance-Optimierung

**Maximale Genauigkeit:**
- **Präzise Kalibrierung**: Exakte Sensorhöhen-Eingabe
- **Optimierte Amplitude**: Anpassung an lokale Gegebenheiten
- **Algorithmus-Wahl**: Situationsgerechte Klassifizierungsmethode
- **Testphase**: Überwachung und Feinabstimmung über mehrere Wochen

**Maximale Reichweite:**
- **Erhöhte Installation**: Optimale Sensorhöhe für gewünschte Abdeckung
- **Reduzierte Amplitude**: Erhöhte Empfindlichkeit für entfernte Objekte
- **Zeit-basierte Klassifizierung**: Präzise Erkennung auch bei schwachen Signalen
- **Umweltanpassung**: Berücksichtigung lokaler Störfaktoren

**Minimale Störungen:**
- **Erhöhte Amplitude**: Ausblendung schwacher Störsignale
- **Amplitude-basierte Klassifizierung**: Fokus auf starke, eindeutige Signale
- **Geometrische Optimierung**: Anpassung der Erfassungsgeometrie
- **Adaptive Algorithmen**: Selbstlernende Störungsunterdrückung

## Technische Spezifikationen

### Klassierungsalgorithmen

**Zeit-basierte Berechnung:**
- **Genauigkeit**: ±10% bei konstanten Geschwindigkeiten
- **Verarbeitungszeit**: 50-200ms je nach Objektgröße
- **Mindestgeschwindigkeit**: 1 km/h für zuverlässige Berechnung
- **Optimaler Bereich**: 2-50 km/h

**Amplitude-basierte Berechnung:**
- **Genauigkeit**: ±15% abhängig von Objektreflektivität
- **Verarbeitungszeit**: <50ms (schnellste Methode)
- **Unabhängig von Geschwindigkeit**: Auch für stehende Objekte
- **Optimaler Bereich**: Alle Geschwindigkeiten

### Amplituden-Spezifikationen

- **Auflösung**: 12-bit (0-4095 Stufen)
- **Dynamikbereich**: >60 dB
- **Rauschpegel**: <50 (bei optimalen Bedingungen)
- **Kalibrierungsgenauigkeit**: ±1% der Einstellung

### Höhen-Kalibrierung

- **Messgenauigkeit**: ±0,1 m Eingabeauflösung
- **Geometrische Korrektur**: Automatische Winkelberechnung
- **Reichweitenoptimierung**: Maximale Detektionsdistanz = 2,5 × Installationshöhe
- **Mindesthöhe**: 2 m (für zuverlässige Funktion)

## Vorteile der Erweiterten Einstellungen

### Präzision

- **Algorithmus-Optimierung**: Auswahl der optimal geeigneten Erkennungsmethode
- **Umgebungsanpassung**: Feinabstimmung auf spezifische Installationsbedingungen
- **Geometrische Korrekturen**: Präzise Berücksichtigung der Installationsgeometrie
- **Performance-Tuning**: Optimierung für spezifische Leistungsanforderungen

### Flexibilität

- **Anpassbare Algorithmen**: Verschiedene Klassifizierungsansätze verfügbar
- **Skalierbare Empfindlichkeit**: Anpassung an verschiedene Objektgrößen
- **Installationsflexibilität**: Unterstützung verschiedener Montagehöhen
- **Umgebungstoleranz**: Anpassung an verschiedene Einsatzgebiete

### Professionalität

- **Experteneinstellungen**: Zugang zu erweiterten Konfigurationsparametern
- **Wissenschaftliche Basis**: Fundierte Algorithmen für präzise Erkennung
- **Kalibrierungsmöglichkeiten**: Exakte Anpassung an lokale Gegebenheiten
- **Dokumentierte Parameter**: Nachvollziehbare und reproduzierbare Einstellungen

Diese erweiterten Einstellungen bieten Experten die Möglichkeit, den Radar-Bewegungssensor präzise an spezifische Anforderungen und Umgebungsbedingungen anzupassen für optimale Detektionsleistung und Klassifizierungsgenauigkeit.

## Untere Bedienelemente

- **OK**: Alle Konfigurationsänderungen bestätigen und anwenden
- **Abbrechen**: Alle Änderungen verwerfen und Konfigurationsdialog schließen
- **Übernehmen**: Änderungen anwenden ohne den Dialog zu schließen
