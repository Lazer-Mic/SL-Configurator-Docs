# Radar-Bewegungsmelder-Konfiguration

![Radar-Bewegungsmelder-Konfiguration](radar-bewegungsmelder-konfiguration.png)

**Radar-Bewegungsmelder-Konfiguration**

Konfigurieren Sie erweiterte Radar-Bewegungsmelder für intelligente Beleuchtungssteuerung mit präziser Bewegungserkennung und Objektklassifizierung. Diese professionelle Lösung ermöglicht die Anpassung von Detektionsparametern, Klassifizierungseinstellungen und Störungsunterdrückung für optimale Sensorfunktionalität.

*Der Servicemodus kann unter dem Menüpunkt Anzeige > Service Mode aktiviert werden.*

## Hauptbereiche

### 1. Radar-Bewegungsmelder-Dashboard

- Diese Seite zeigt den iFux LightRadar (Seriennummer: 0102030405060708) im Hauptdashboard
- Anzeige aller Erkennungsstatistiken und aktueller Klassifizierungswerte
- Zugang zu allen Konfigurationsbereichen des Radarsensors

### 2. Konfigurationsbereiche

- **Allgemein**: Grundlegende Radareinstellungen und Parameter
- **Erweiterte Einstellungen**: Spezialisierte Konfigurationsoptionen
- **Detektionsrichtungen**: Richtungsabhängige Erkennungseinstellungen
- **Automatische Rauschunterdrückung**: Störungserkennung und -filterung
- **Klassifizierungslimiten**: Objektklassifizierungs-Schwellenwerte

## Erkennungsstatistiken

### Zähler-Übersicht

**Annäherungs-Erkennung:**
- **Undefiniert Annähernd**: 293 Erkennungen
- **Undefiniert Entfernend**: 302 Erkennungen
- **Person Annähernd**: 22.231 Erkennungen
- **Person Entfernend**: 16.263 Erkennungen
- **Fahrrad Annähernd**: 1.987 Erkennungen
- **Fahrrad Entfernend**: 2.068 Erkennungen

**Fahrzeug-Erkennung:**
- **Motorrad Annähernd**: 7.429 Erkennungen
- **Motorrad Entfernend**: 8.109 Erkennungen
- **Auto Annähernd**: 8.233 Erkennungen
- **Auto Entfernend**: 11.912 Erkennungen
- **Bus/LKW Annähernd**: 2.302 Erkennungen
- **Bus/LKW Entfernend**: 2.238 Erkennungen

### Aktuelle Klassifizierung

**Letztes Ereignis:**
- **Klassifizierung**: Fahrrad
- **Richtung & Geschwindigkeit**: 4 km/h
- **Energie**: 815

**Geschwindigkeitsmessung:**
- **Ø Annäherungsgeschw.**: 0 km/h
- **Ø Entfernungsgeschw.**: 4 km/h
- **Länge**: 0,7 m

## Funktionsbereiche

### Bewegungszähler zurücksetzen

- **Counter Reset**: Hier klicken um den Bewegungszähler zurückzusetzen
- Setzt alle Erkennungsstatistiken auf Null zurück
- Nützlich für neue Messperioden oder Kalibrierung

### Radar-Bewegungszähler zurücksetzen

- **Radar Counter Reset**: Hier klicken um die Bewegungszähler des Radarsensors zurückzusetzen
- Spezifischer Reset für Radar-spezifische Zähler
- Unabhängig von allgemeinen Bewegungszählern

### Radar-Bewegungssensor-Konfiguration

- **Sensor Configuration**: Hier klicken um die Radarsensor-Konfiguration anzupassen
- Zugang zu detaillierten Konfigurationsoptionen
- Umfassende Parameter-Anpassung möglich

### Firmware-Update

- **Firmware-Version**: 3.1.0
- **Hardware-Version**: 2.0
- **Seriennummer**: 1814
- **Update-Option**: Hier klicken um die Radarsensor-Firmware zu aktualisieren

## Detaillierte Konfigurationsbereiche

### [1. Allgemein](1-allgemein/README.md)

Grundlegende Radareinstellungen und Betriebsparameter:
- Detektionsreichweite und Empfindlichkeit
- Grundlegende Erkennungsparameter
- Betriebsmodi und Energieverwaltung
- Installationseinstellungen

### [2. Erweiterte Einstellungen](2-erweiterte-einstellungen/README.md)

Spezialisierte Konfigurationsoptionen für Experten:
- Erweiterte Algorithmus-Parameter
- Feineinstellung der Signalverarbeitung
- Anpassung für spezielle Umgebungsbedingungen
- Performance-Optimierung

### [3. Detektionsrichtungen](3-detektionsrichtungen/README.md)

Richtungsabhängige Erkennungseinstellungen:
- Annäherungs- und Entfernungsrichtungen
- Asymmetrische Erkennungsparameter
- Richtungsspezifische Schwellenwerte
- Geometrische Anpassungen

### [4. Automatische Rauschunterdrückung](4-automatische-rauschunterdrueckung/README.md)

Störungserkennung und intelligente Filterung:
- Umgebungsrausch-Analyse
- Adaptive Störungsunterdrückung
- Falschauslösung-Minimierung
- Umweltanpassung

### [5. Klassifizierungslimiten](5-klassifizierungslimiten/README.md)

Objektklassifizierungs-Schwellenwerte und -Parameter:
- Größenbasierte Klassifizierung
- Geschwindigkeits-Schwellenwerte
- Energie-Limiten für verschiedene Objekttypen
- Machine-Learning-Parameter

## Objektklassifizierung

### Unterstützte Objekttypen

**Personen:**
- **Person**: Fußgänger mit typischen Bewegungsmustern
- **Geschwindigkeitsbereich**: 1-8 km/h
- **Größenklassifizierung**: Kleine bis mittlere Radar-Signatur
- **Bewegungsmuster**: Unregelmäßige, organische Bewegungen

**Fahrzeuge:**
- **Fahrrad**: Zweiräder mit charakteristischen Geschwindigkeiten
- **Motorrad**: Schnelle Zweiräder mit höherer Energie-Signatur
- **Auto**: Standardfahrzeuge mit mittlerer Größe und Geschwindigkeit
- **Bus/LKW**: Große Fahrzeuge mit hoher Radar-Signatur

**Undefiniert:**
- **Undefinierte Objekte**: Nicht klassifizierbare Bewegungen
- **Störungen**: Umwelteinflüsse und Fehlerkennungen
- **Unbekannte Muster**: Neue oder ungewöhnliche Objekttypen

### Klassifizierungsparameter

**Größe/Energie:**
- **Radar-Querschnitt**: Objektgröße basierend auf Radar-Reflexion
- **Energie-Schwellenwerte**: Minimale und maximale Energiewerte
- **Signatur-Analyse**: Charakteristische Radar-Fingerprints
- **Größenverteilung**: Statistische Größenanalyse

**Geschwindigkeit:**
- **Doppler-Analyse**: Geschwindigkeitsmessung über Frequenzverschiebung
- **Beschleunigungsmuster**: Charakteristische Beschleunigungs-/Bremsverhalten
- **Geschwindigkeitsverteilung**: Typische Geschwindigkeitsbereiche pro Objekttyp
- **Bewegungsrichtung**: Annäherung vs. Entfernung

## Anwendungsbereiche

### Verkehrsüberwachung

- **Straßenbeleuchtung**: Adaptive Beleuchtung basierend auf Verkehrsaufkommen
- **Kreuzungssteuerung**: Verkehrsabhängige Beleuchtungsanpassung
- **Fußgängerüberwege**: Sichere Überquerung durch gezielte Beleuchtung
- **Radwege**: Spezielle Erkennung und Beleuchtung für Radfahrer

### Smart City Anwendungen

- **Öffentliche Plätze**: Personenerkennung für adaptive Platzbeleuchtung
- **Parks und Grünanlagen**: Sicherheitsbeleuchtung mit Bewegungserkennung
- **Bushaltestellen**: Fahrzeug- und Personenerkennung für Komfort
- **Parkplätze**: Fahrzeugerkennung für effiziente Beleuchtung

### Industrielle Anwendungen

- **Fabrikhallen**: Personenerkennung für arbeitsplatzspezifische Beleuchtung
- **Lagerhäuser**: Fahrzeug- und Personenerkennung für Logistikoptimierung
- **Sicherheitsbereiche**: Präzise Objektklassifizierung für Sicherheitssysteme
- **Produktionslinien**: Bewegungserkennung für prozessoptimierte Beleuchtung

### Kommerzielle Installationen

- **Einzelhandel**: Kundenerkennung für Verkaufsflächenbeleuchtung
- **Bürogebäude**: Mitarbeitererkennung für energieeffiziente Beleuchtung
- **Hotels**: Gasterkennung für komfortable Beleuchtungssteuerung
- **Restaurants**: Personenstrom-Analyse für Ambientebeleuchtung

## Technische Spezifikationen

### Radar-Technologie

- **Frequenzbereich**: 24 GHz ISM-Band
- **Detektionsreichweite**: Bis zu 20 Meter (konfigurierbar)
- **Winkelabdeckung**: 120° horizontaler Erfassungsbereich
- **Auflösung**: Geschwindigkeit ±0,1 km/h, Entfernung ±0,1 m
- **Update-Rate**: 10-50 Hz (konfigurierbar)

### Klassifizierungsgenauigkeit

- **Personenerkennung**: >95% Genauigkeit
- **Fahrzeugerkennung**: >90% Genauigkeit (nach Typ)
- **Falschauslösungen**: <1% bei optimaler Konfiguration
- **Reaktionszeit**: <100ms für Standarderkennungen

### Umweltspezifikationen

- **Betriebstemperatur**: -40°C bis +85°C
- **Luftfeuchtigkeit**: 0-100% RH (nicht kondensierend)
- **IP-Schutzart**: IP65 (staub- und wasserdicht)
- **Vibrationsresistenz**: IEC 60068-2-6 konform

## Vorteile der Radar-Bewegungserkennung

### Präzision

- **Objektklassifizierung**: Unterscheidung zwischen verschiedenen Objekttypen
- **Geschwindigkeitsmessung**: Präzise Doppler-basierte Geschwindigkeitserfassung
- **Richtungserkennung**: Unterscheidung zwischen Annäherung und Entfernung
- **Störungsresistenz**: Robuste Funktion bei verschiedenen Wetterbedingungen

### Intelligenz

- **Adaptive Algorithmen**: Selbstlernende Erkennungsmuster
- **Umweltanpassung**: Automatische Anpassung an Installationsumgebung
- **Störungsfilterung**: Intelligente Unterdrückung von Fehlauslösungen
- **Statistik-Funktionen**: Umfassende Datensammlung und -analyse

### Effizienz

- **Energieoptimierung**: Bedarfsgerechte Beleuchtungssteuerung
- **Wartungsreduzierung**: Robuste, wartungsarme Technologie
- **Kosteneffizienz**: Optimierter Energieverbrauch durch präzise Erkennung
- **Langlebigkeit**: Langzeitstabile Radartechnologie

### Flexibilität

- **Konfigurierbarkeit**: Umfassende Anpassungsmöglichkeiten
- **Skalierbarkeit**: Einsatz von Einzelinstallationen bis zu großen Netzwerken
- **Integration**: Nahtlose Integration in bestehende Beleuchtungssysteme
- **Update-Fähigkeit**: Firmware-Updates für neue Funktionen

Diese Radar-Bewegungsmelder-Konfiguration bietet eine hochentwickelte Lösung für intelligente Beleuchtungssteuerung mit präziser Objekterkennung, umfassender Klassifizierung und flexiblen Konfigurationsmöglichkeiten für verschiedenste Anwendungsbereiche.
