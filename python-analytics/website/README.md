# zkEVM Benchmark Website

A sleek, modern dashboard for visualizing zkEVM benchmark results with detailed performance analysis and test results.

## 🌟 Features

- **Interactive Dashboard**: Overview of all zkVMs with key metrics
- **Performance Leaderboards**: Fastest and slowest test execution times
- **Detailed Test Results**: Individual test cards with success/failure status and error details
- **Responsive Design**: Works on desktop, tablet, and mobile devices
- **Static Hosting Ready**: Fully client-side, perfect for GitHub Pages

## 🚀 Quick Start

### Local Development

1. **Start the local server:**
   ```bash
   cd website
   python3 serve.py
   ```

2. **Open in browser:**
   - Main Dashboard: http://localhost:8080/index.html
   - RISC0 Details: http://localhost:8080/zkvm.html?zkvm=risc0
   - SP1 Details: http://localhost:8080/zkvm.html?zkvm=sp1

### Deploy to GitHub Pages

1. **Copy this `website/` directory to your repository**

2. **Enable GitHub Pages:**
   - Go to repository Settings → Pages
   - Set Source to "Deploy from a branch"
   - Choose `main` branch and `/docs` folder (or rename `website/` to `docs/`)
   - Or choose `main` branch and `/` root if you put website files in the root

3. **Your site will be available at:**
   ```
   https://yourusername.github.io/yourrepository/
   ```

## 📁 File Structure

```
website/
├── index.html              # Main dashboard page
├── zkvm.html               # Generic zkVM detail page
├── styles.css              # All styling
├── app.js                  # Main dashboard JavaScript
├── zkvm-detail-generic.js  # zkVM detail page JavaScript
├── serve.py                # Local development server
├── analytics_output/       # Analytics data (JSON files)
│   ├── summary_analytics.json
│   ├── risc0_analytics.json
│   └── sp1_analytics.json
└── README.md               # This file
```

## 🎨 Design Features

- **Modern UI**: Clean, professional design with subtle animations
- **Performance First**: Optimized for fast loading and smooth interactions
- **Accessible**: Proper contrast ratios and focus indicators
- **Mobile Responsive**: Adaptive layout for all screen sizes

## 🔧 Customization

### Adding New zkVMs

The system automatically detects new zkVMs from the analytics data. Just add:
1. `{zkvm_name}_analytics.json` to `analytics_output/`
2. Update `summary_analytics.json` to include the new zkVM
3. The website will automatically show the new zkVM

### Updating Analytics Data

Replace the JSON files in `analytics_output/` with new data. The website will automatically load the updated information.

## 📊 Analytics Data Format

The website expects these JSON files in `analytics_output/`:

- `summary_analytics.json`: Overview of all zkVMs
- `{zkvm}_analytics.json`: Detailed analytics for each zkVM

## 🛠️ Development

### Local Testing
```bash
# Start development server
python3 serve.py

# Test different zkVMs
open "http://localhost:8080/zkvm.html?zkvm=risc0"
open "http://localhost:8080/zkvm.html?zkvm=sp1"
```

### Browser Compatibility
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari, Chrome Mobile)

## 📝 License

This website is part of the zkEVM benchmark project.