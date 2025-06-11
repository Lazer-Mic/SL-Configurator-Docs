# SL-Configurator Documentation

[![Documentation Status](https://img.shields.io/badge/docs-live-brightgreen)](https://lazer-mic.github.io/SL-Configurator-Docs/)
[![Built with mdBook](https://img.shields.io/badge/built%20with-mdBook-blue)](https://github.com/rust-lang/mdBook)
[![GitHub Pages](https://img.shields.io/badge/hosted-GitHub%20Pages-222222)](https://pages.github.com/)

Comprehensive documentation for the **esave SL-Configurator** software - A powerful tool for configuring and managing lighting systems with SL-Controllers.

## 🌐 Live Documentation

**📖 [View Documentation](https://lazer-mic.github.io/SL-Configurator-Docs/)**

## 📋 About

The SL-Configurator is a user-friendly Windows application for displaying, configuring, and managing lights equipped with esave SL-Controllers. This documentation provides detailed guidance for installation, configuration, and advanced usage.

## 🚀 Quick Start

### Online Access
Visit: **https://lazer-mic.github.io/SL-Configurator-Docs/**

### Local Development
```bash
# Clone the repository
git clone https://github.com/Lazer-Mic/SL-Configurator-Docs.git
cd SL-Configurator-Docs

# Serve locally
cd de
mdbook serve --open
```

## 📚 Documentation Structure

### 🎯 Erste Schritte (Getting Started)
- **0** - Einleitung (Introduction)
- **1** - Installation 
- **2** - Aktivierung (Activation)
- **3** - Einstellungen (Settings) - 5 detailed subsections

### ⚙️ Konfiguration (Configuration)  
- **4** - Leuchten Konfiguration (Luminaire Configuration) - 11 comprehensive subsections
- **5** - Gerätegruppen (Device Groups)
- **6** - Sensoren (Sensors)
- **7** - Übergeordnete Lichtschalter (Master Light Switches)

### 🛠️ Erweiterte Funktionen (Advanced Features)
- **8** - Servicemodus (Service Mode) - 6 detailed diagnostic subsections
- **9** - SLC RC Switch Konfiguration - 3 subsections  
- **10** - Datenexport (Data Export)
- **11** - Firmware Update Assistent (Firmware Update Assistant)

## 🎨 Features

- ✅ **Complete Coverage**: All SL-Configurator features documented
- ✅ **Visual Guides**: Hundreds of screenshots and step-by-step instructions
- ✅ **German Language**: Professional German technical documentation
- ✅ **Co-located Images**: Images stored alongside content for reliability
- ✅ **Mobile Responsive**: Works on desktop, tablet, and mobile devices
- ✅ **Fast Search**: Built-in search functionality
- ✅ **Auto-deploy**: Automatic deployment via GitHub Actions

## 📁 Project Structure

```
SL-Configurator-Docs/
├── README.md              # This file
├── .github/               # GitHub Actions workflows  
└── de/                    # German documentation
    ├── book.toml          # mdBook configuration
    ├── src/               # Documentation source
    │   ├── SUMMARY.md     # Table of contents
    │   ├── 0-einleitung/  # Introduction with images
    │   ├── 1-installation/ # Installation guide  
    │   ├── ...            # All documentation sections
    │   └── 11-firmware-update-assistent/
    └── book/              # Generated site (auto-deployed)
```

## 🔧 Technical Requirements

### SL-Configurator Software Requirements
- **Hardware**: esave SLC-USB stick with built-in radio module
- **OS**: Windows operating system (computer or tablet)
- **Version**: SL-Configurator Version 2.5 (Build 8441) or higher for Gen 2 controllers

### Development Requirements  
- [mdBook](https://github.com/rust-lang/mdBook) - Static site generator
- Git for version control

## 🤝 Contributing

1. **Content Updates**: Edit markdown files in `de/src/`
2. **Local Testing**: Use `mdbook serve` to preview changes
3. **Image Management**: Place images in same directory as markdown files
4. **Commit & Push**: Changes auto-deploy via GitHub Actions

## 📄 License

Documentation for proprietary esave SL-Configurator software.

## 👨‍💻 Author

**Michael Orlando** - Complete documentation creation and organization

---

*Built with ❤️ using [mdBook](https://github.com/rust-lang/mdBook) and deployed automatically via GitHub Actions* 🚀 