# Brightstuff Label Printer App - Production Guide

## 🔧 Installation & Setup

### Prerequisites
1. **Label Server**: Ensure the label server is running and accessible
2. **Network Configuration**: Verify connectivity to `label-server.bstuff:6500`
3. **Printer Setup**: Configure printers and add them to the server's `printers.json`

### Installation Steps
1. **Download**: Get the latest release from the GitHub repository
2. **Install**: Run the installer for your operating system
3. **Launch**: Start the application
4. **Verify Connection**: Check that the app connects to the label server

### Configuration Files
The application relies on server-side configuration files:
- `printers.json`: List of available printers and their settings
- `labels.json`: Available label formats and types

## 🎮 User Interface Guide

### Main Interface Layout

#### 1. **Header Section**
- **Logo**: Brightstuff branding

#### 2. **Printer Configuration**
- **Printer Selection**: Dropdown to select available printers
- **TSC Double Sticker Toggle**: Special option for TSC printers (appears only when TSC printer is selected)

#### 3. **Label Configuration**
- **Label Selection**: Multi-select dropdown for choosing label types and quantities
- **File Upload**: PDF template upload (appears when "dataplate" label is selected)

#### 4. **Data Input**
- **QR Code Input**: Text area for scanning QR codes or manual data entry
- **Auto-print**: Automatically prints when QR code is scanned (500ms delay)

#### 5. **Action Buttons**
- **Print Sticker**: Main printing action
- **Print Previous**: Reprint the last printed label (if available)

## 🖨️ Printing Workflow

### Step-by-Step Process

#### 1. **Printer Setup**
```
1. Select your printer from the dropdown
2. For TSC printers: Toggle "Double Sticker" if needed
3. Verify printer connection
```

#### 2. **Label Configuration**
```
1. Select label types from the multi-select dropdown
2. Set quantities for each label type
3. If using "dataplate" labels: Upload PDF template
```

#### 3. **Data Input**
```
1. Scan QR code or manually enter data
2. Data format is automatically detected (JSON or string)
3. Auto-print triggers after 500ms delay
```

#### 4. **Printing**
```
1. Click "Print Sticker" or wait for auto-print
2. Monitor printing status
3. Success/error notifications appear
```

### Supported Data Formats

#### **String Format**
```
Sample text for Vindicator:
Example: "1,,373734470B30211C4D3B252C,,,,06/25/2025,2024_0177P,Vindicator,,,,vindicator,1.2.0,,1.0.0.3,"
```

#### **JSON Format**
```
Structured data:
Example: {"product": "Widget", "serial": "12345", "date": "2024-01-01"}
```

## 🚨 Troubleshooting

### Common Issues

#### **Connection Problems**
```
Problem: App cannot connect to label server
Solution:
1. Verify server is running on label-server.bstuff:6500
2. Check network connectivity
3. Verify firewall settings
4. Restart the application
```

#### **Printer Not Found**
```
Problem: Printer not appearing in dropdown
Solution:
1. Check printers.json configuration on server
2. Verify printer is powered on and connected
3. Check network connectivity to printer
4. Restart label server
```

#### **Print Failures**
```
Problem: Labels not printing
Solution:
1. Check printer status and paper
2. Verify label format compatibility
3. Check printer IP address in configuration
4. Review error messages in notifications
```

#### **File Upload Issues**
```
Problem: PDF templates not uploading
Solution:
1. Verify file is valid PDF format
2. Check server storage permissions
3. Verify network connectivity
4. Try smaller file size
```

### **Error Messages**

| Error | Meaning | Action |
|-------|---------|--------|
| "Error fetching data" | Cannot connect to server | Check server status and network |
| "File upload failed" | PDF upload unsuccessful | Verify file format and permissions |
| "Error printing sticker" | Printer communication failed | Check printer status and configuration |
| "File already exists" | Template already uploaded | Use existing template or rename file |

**Version**: 1.2.2
**Last Updated**: January 2024
**Author**: João Batista
**License**: MIT License
