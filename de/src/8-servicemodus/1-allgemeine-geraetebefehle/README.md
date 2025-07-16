# Allgemeine Gerätebefehle

![Allgemeine Gerätebefehle](allgemeine-geraetebefehle.png)

**Allgemeine Gerätebefehle - esave Configurator Servicemodus**

Verwalten Sie kritische Systemfunktionen des esave Lighting Controllers mit professionellen Werkzeugen für Werksrücksetzung und Systemneustarts. Diese essentiellen Gerätebefehle ermöglichen die sichere Systemwartung und Problembehandlung mit umfassenden Sicherheitsmaßnahmen.

*Der Servicemodus kann unter dem Menüpunkt Anzeige > Service Mode aktiviert werden.*

## Hauptbereiche

### 1. Service-Tab-Navigation

Die obere Tab-Leiste bietet Zugang zu allen Servicemodus-Bereichen:
- **Allgemeine Geräte-Kommandos**: Grundlegende Systemoperationen (aktuell aktiv)
- **Gerätestatus**: Aktueller Systemzustand und Betriebsinformationen
- **Empfang**: Signalstärke und Konnektivitätsstatus
- **DALI-2 Diagnosedaten**: DALI-Bus-Diagnose und Analysefunktionen
- **Gerätekonfiguration**: Systemparameter und Betriebsmoduseinstellungen
- **Lichtstromkorrektur (LFC)**: LED-Alterungskomp nsation und Kalibrierung
- **Passwortfinder**: Distributor-Zugangstools für Autorisierung

### 2. Kritische Systemkommandos

Der Hauptbereich zeigt zwei essentiielle Gerätebefehle:
- **Werksreset**: Vollständige Rücksetzung auf Auslieferungszustand
- **Soft-Reset**: Kontrollierter Systemneumstart ohne Datenverlust
- **Visueller Status**: Farbcodierte Icons für Operationstyp-Unterscheidung

## Auf Werkseinstellungen zurücksetzen

### Factory Reset Kommando

**Status-Icon:** 🔧 Rotes Zahnrad (Kritische Operation)
- **Funktion**: "Setzt alle Geräteeinstellungen auf die Werkseinstellungen zurück"
- **Operationstyp**: Vollständige Systemrücksetzung (Factory Reset)
- **Datenbehandlung**: Löscht alle benutzerdefinierten Konfigurationen
- **Sicherheitsstufe**: Höchste Sicherheitsstufe mit Mehrfachbestätigung

**Rücksetzungsumfang:**
- **Netzwerkkonfiguration**: IP-Adressen, WLAN-Einstellungen, Mobilfunkparameter
- **Beleuchtungsprofile**: Dimming-Kurven, Zeitprogramme, Szenario-Einstellungen
- **Sensorkonfiguration**: Radar-Parameter, Empfindlichkeitseinstellungen, Klassifizierungslimiten
- **Benutzerdaten**: Passwörter, Benutzerkonten, Session-Tokens
- **Betriebsstatistiken**: Zähler, Laufzeiten, Verbrauchsdaten (optional)

**Beibehaltene Systemdaten:**
- **Firmware-Version**: Aktuelle Betriebssoftware bleibt erhalten
- **Hardware-Kalibrierung**: Werksseitige Sensor- und LED-Kalibrierung
- **Geräte-Identifikation**: Seriennummer, MAC-Adressen, Zertifikate
- **Lizenzen**: Software-Lizenzen und Freischaltungen

### Sicherheitsmaßnahmen

**Mehrstufiger Bestätigungsprozess:**
1. **Primärbestätigung**: Klick auf Werksreset-Button
2. **Warnhinweis**: Detaillierte Information über Datenverlust
3. **Sicherheitsabfrage**: "Sind Sie sicher?" Dialog
4. **Finale Bestätigung**: "Wirklich alle Daten löschen?" Abfrage

**Automatische Sicherungen:**
- **Konfigurations-Backup**: Automatische Erstellung einer Sicherungskopie
- **System-Snapshot**: Vollständige Systemzustandssicherung
- **Recovery-Point**: Wiederherstellungspunkt für Notfall-Rollback
- **Audit-Log**: Protokollierung der Reset-Operation mit Zeitstempel

## Gerät neu starten

### Soft-Reset Kommando

**Status-Icon:** 🔄 Grüner Refresh-Pfeil (Sichere Operation)
- **Funktion**: "Startet das Gerät neu (Soft-Reset)"
- **Operationstyp**: Kontrollierter Systemneumstart ohne Datenverlust
- **Konfigurationserhaltung**: Alle Einstellungen bleiben vollständig erhalten
- **Sicherheitsstufe**: Standard-Sicherheit mit einfacher Bestätigung

**Neustart-Sequenz:**
1. **Graceful Shutdown**: Kontrolliertes Herunterfahren aller Services
2. **Speicher-Clearing**: Bereinigung des RAM und Cache-Speicher
3. **Hardware-Reset**: Neuinitialisierung der Hardware-Komponenten
4. **Service-Restart**: Systematischer Neustart aller Systemdienste
5. **Konfiguration-Reload**: Laden der gespeicherten Konfiguration
6. **System-Validation**: Überprüfung der Systemintegrität

**Beibehaltene Daten:**
- **Komplette Konfiguration**: Alle Benutzereinstellungen bleiben erhalten
- **Netzwerkverbindungen**: Automatische Wiederherstellung aller Verbindungen
- **Betriebsstatistiken**: Zähler, Laufzeiten, historische Daten
- **Session-Daten**: Aktive Sitzungen werden nach Neustart wiederhergestellt

### Neustart-Anwendungen

**Systemwartung:**
- **Speicher-Optimierung**: Bereinigung von Memory-Leaks und Fragmentierung
- **Cache-Clearing**: Entfernung veralteter oder korrupter Cache-Daten
- **Service-Refresh**: Neuinitialisierung hängender oder instabiler Services
- **Performance-Reset**: Wiederherstellung optimaler Systemleistung

**Problembehandlung:**
- **Netzwerk-Probleme**: Neuaufbau gestörter Verbindungen
- **Sensor-Kalibrierung**: Neukalibrierung nach Umgebungsveränderungen
- **Software-Instabilität**: Behebung temporärer Software-Probleme
- **Hardware-Synchronisation**: Neusynchronisation der Hardware-Komponenten

## Bedienelemente und Navigation

### Service-Tab-Übersicht

Die Servicemodus-Navigation zeigt alle verfügbaren Diagnosebereiche:

**Aktiver Tab - Allgemeine Geräte-Kommandos:**
- **Status**: Aktuell ausgewählt (highlighted)
- **Inhalt**: Kritische Systemoperationen
- **Zugriff**: Factory Reset und Soft Reset Funktionen

**Weitere Service-Tabs:**
- **Gerätestatus**: Live-Systemüberwachung und Zustandsanzeigen
- **Empfang**: Mobilfunk-, WLAN- und Konnektivitätsinformationen
- **DALI-2 Diagnosedaten**: DALI-Bus-Analyse und Geräte-Diagnose
- **Gerätekonfiguration**: Hardware-Parameter und Betriebsmoduseinstellungen
- **Lichtstromkorrektur (LFC)**: LED-Aging-Kompensation und Kalibrierung
- **Passwortfinder**: Distributor-Autorisierung und Zugangsverwaltung

### Geräteauswahl und Context

**Linke Seitenleiste - Gerätehierarchie:**
- **esave USB-Stick**: Angeschlossenes Konfigurationstool
- **Verfügbare Geräte**: Expandierbare Liste aller erkannten esave Devices
- **Aktuelles Gerät**: "esave USB-Stick" (blau markiert, aktuell ausgewählt)
- **Gerätestatus**: Verbindungsindikator und Kommunikationsstatus

**Hauptmenü-Navigation:**
- **slConfigurator**: Hauptkonfigurationstool mit vollständiger Geräte-Integration
- **Einstellungen, Anzeige, Werkzeuge**: Zusätzliche Konfigurationsbereiche
- **Gerätelist, Leuchte, Hilfe**: Erweiterte Verwaltungs- und Support-Funktionen

## Kommando-Ausführung

### Ausführungsprozess

**Benutzerinteraktion:**
1. **Tab-Auswahl**: Navigation zu "Allgemeine Geräte-Kommandos"
2. **Kommando-Wahl**: Auswahl zwischen Werksreset oder Neustart
3. **Sicherheitsvalidierung**: Durchlaufen der Bestätigungsdialoge
4. **Ausführungsstart**: Initiation der gewählten Operation
5. **Progress-Monitoring**: Überwachung des Ausführungsfortschritts
6. **Completion-Bestätigung**: Erfolgsmeldung und Systemvalidierung

**Systemverhalten:**
- **Immediate Feedback**: Sofortige Rückmeldung bei Kommando-Auswahl
- **Progress Indication**: Fortschrittsbalken und Status-Updates
- **Error Handling**: Automatische Fehlerbehandlung und Recovery
- **Success Notification**: Bestätigung der erfolgreichen Kommando-Ausführung

### Sicherheitsfeatures

**Zugriffskontrolle:**
- **Administratorrechte**: Beschränkung auf autorisierte Benutzer
- **Session-Validation**: Überprüfung der aktuellen Sitzung
- **Device-Authentication**: Bestätigung der Geräteberechtigung
- **Operation-Authorization**: Rollenbasierte Kommando-Freigabe

**Datenintegrität:**
- **Pre-Operation Backup**: Automatische Sicherung vor kritischen Operationen
- **Integrity Checks**: Validierung der Systemintegrität
- **Rollback Capability**: Wiederherstellungsmöglichkeit bei Problemen
- **Consistency Validation**: Überprüfung der Datenkonsistenz

## Technische Spezifikationen

### Kommando-Performance

- **Ausführungszeit Factory Reset**: 30-60 Sekunden (abhängig von Datenmenge)
- **Ausführungszeit Soft Reset**: 15-30 Sekunden (Standard-Neustart)
- **Backup-Erstellung**: 5-10 Sekunden (automatisch vor Reset)
- **Validierungszeit**: 3-5 Sekunden (Systemintegritätsprüfung)

### Systemanforderungen

- **Arbeitsspeicher**: 64MB RAM für Backup-Erstellung
- **Storage**: 16MB für temporäre Backup-Dateien
- **Prozessor**: ARM Cortex-M4 für Kommando-Verarbeitung
- **Netzwerk**: Aufrechte Verbindung für Remote-Operationen

### Kompatibilität

- **esave Controller**: Alle esave Lighting Controller Generationen
- **Firmware-Versionen**: Ab Firmware v2.0 (Factory Reset ab v2.5)
- **Protokolle**: USB, DALI-2, MQTT, HTTP/HTTPS
- **Betriebssysteme**: Windows, macOS, Linux (slConfigurator)

## Anwendungsszenarien

### Inbetriebnahme

**Neue Installation:**
- **Werksreset**: Sauberer Ausgangszustand für Erstkonfiguration
- **Baseline-Setup**: Definierter Startzustand für alle Parameter
- **Konfigurationsvorbereitung**: Bereintigung für projektspezifische Einstellungen
- **Qualitätssicherung**: Konsistente Ausgangsbasis für alle Installationen

### Wartung und Service

**Planmäßige Wartung:**
- **Soft Reset**: Monatliche Systemreinigung und Memory-Optimierung
- **Cache-Clearing**: Entfernung alter Daten und temporärer Dateien
- **Service-Refresh**: Neustart nach Firmware-Updates oder Konfigurationsänderungen
- **Preventive Maintenance**: Vorbeugende Systemoptimierung

**Störungsbeseitigung:**
- **Problem-Isolation**: Werksreset zur Identifikation von Konfigurationsproblemen
- **System-Recovery**: Soft Reset bei temporären Systeminstabilitäten
- **Network-Reset**: Neustart zur Behebung von Konnektivitätsproblemen
- **Performance-Optimization**: Systemreinigung bei Leistungsproblemen

### Support und Diagnose

**Remote-Support:**
- **Standardisierte Ausgangslage**: Werksreset für konsistente Support-Basis
- **Problem-Reproduktion**: Soft Reset zur Reproduktion von Fehlerzuständen
- **Diagnose-Preparation**: Systemreinigung vor erweiterten Diagnoseverfahren
- **Solution-Validation**: Reset zur Bestätigung von Problemlösungen

## Vorteile der Allgemeinen Gerätebefehle

### Systemzuverlässigkeit

- **Robuste Operationen**: Sichere Ausführung kritischer Systemkommandos
- **Fehlerresistenz**: Automatische Wiederherstellung bei Ausführungsproblemen
- **Datenschutz**: Umfassende Backup-Strategien vor kritischen Operationen
- **Konsistente Zuständde**: Definierte und reproduzierbare Systemzustände

### Wartungseffizienz

- **Schnelle Problemlösung**: Effiziente Tools für häufige Wartungsaufgaben
- **Automatisierte Prozesse**: Reduzierung manueller Eingriffe bei Routineoperationen
- **Standardisierte Verfahren**: Einheitliche Vorgehensweise für alle Installationen
- **Dokumentierte Operationen**: Vollständige Protokollierung für Nachverfolgung

### Professionelle Systemverwaltung

- **Expertentools**: Spezialisierte Funktionen für Fachkräfte und Techniker
- **Sichere Ausführung**: Mehrstufige Sicherheitsmaßnahmen für kritische Operationen
- **Umfassende Kontrolle**: Vollständige Systemkontrolle über intuitive Bedienoberfläche
- **Skalierbare Verwaltung**: Effiziente Verwaltung einzelner Geräte bis hin zu Großinstallationen

Diese allgemeinen Gerätebefehle bieten eine professionelle und sichere Grundlage für die Systemverwaltung von esave Lighting Controllern mit robusten Sicherheitsmaßnahmen und effizienten Wartungsoperationen.