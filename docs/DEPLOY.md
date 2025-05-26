# 🚀 Deploy to GitHub Pages

This guide will help you deploy the zkEVM benchmark website to GitHub Pages for free hosting.

## ✅ What's Ready

Your website is now **fully static** and ready for GitHub Pages! Here's what we've prepared:

- ✅ All analytics data copied locally (`analytics_output/` directory)
- ✅ JavaScript updated to use local paths (`./analytics_output/`)
- ✅ No server dependencies - pure static files
- ✅ Modern responsive design
- ✅ Performance leaderboard with horizontal layout

## 🗂️ Files Ready for Deployment

```
website/
├── index.html              # Main dashboard
├── zkvm.html               # zkVM detail page  
├── styles.css              # All styling
├── app.js                  # Dashboard logic
├── zkvm-detail-generic.js  # Detail page logic
├── analytics_output/       # Analytics data
│   ├── summary_analytics.json
│   ├── risc0_analytics.json
│   └── sp1_analytics.json
└── README.md               # Documentation
```

## 📋 Deployment Steps

### Option 1: Root Directory Deployment

1. **Copy website files to your repository root:**
   ```bash
   cp -r website/* /path/to/your/repository/
   ```

2. **Enable GitHub Pages:**
   - Go to your repository on GitHub.com
   - Click **Settings** → **Pages**
   - Under **Source**, select **Deploy from a branch**
   - Choose **main** branch and **/ (root)** folder
   - Click **Save**

3. **Access your site:**
   ```
   https://yourusername.github.io/yourrepository/
   ```

### Option 2: Docs Directory Deployment

1. **Copy website files to docs directory:**
   ```bash
   mkdir docs
   cp -r website/* docs/
   ```

2. **Enable GitHub Pages:**
   - Go to your repository on GitHub.com
   - Click **Settings** → **Pages**
   - Under **Source**, select **Deploy from a branch**
   - Choose **main** branch and **/docs** folder
   - Click **Save**

3. **Access your site:**
   ```
   https://yourusername.github.io/yourrepository/
   ```

## 🔗 Available Pages

Once deployed, your site will have:

- **Main Dashboard**: `https://yourusername.github.io/yourrepository/`
- **RISC0 Details**: `https://yourusername.github.io/yourrepository/zkvm.html?zkvm=risc0`
- **SP1 Details**: `https://yourusername.github.io/yourrepository/zkvm.html?zkvm=sp1`

## 🔄 Updating Data

To update the website with new benchmark data:

1. **Generate new analytics** (in your main project):
   ```bash
   python3 generate_analytics.py  # or your analytics script
   ```

2. **Copy new data to website:**
   ```bash
   cp analytics_output/*.json website/analytics_output/
   ```

3. **Commit and push:**
   ```bash
   git add .
   git commit -m "Update benchmark data"
   git push
   ```

4. **GitHub Pages will automatically redeploy** (usually takes 1-2 minutes)

## ⚡ Local Testing

Before deploying, test locally:

```bash
cd website
python3 serve.py
```

Then visit `http://localhost:8080` to verify everything works.

## 🛠️ Troubleshooting

### Site shows 404
- Check that `index.html` is in the root of your selected folder
- Verify GitHub Pages is enabled and source is set correctly

### Analytics not loading
- Check that `analytics_output/` directory is committed to git
- Verify JSON files are valid (use `python -m json.tool file.json`)

### Mobile layout issues
- The design is responsive and should work on all devices
- Clear your browser cache if styles seem outdated

## 📈 Next Steps

Your static website is production-ready! Consider:

- Setting up a custom domain
- Adding Google Analytics for visitor tracking
- Setting up automated data updates with GitHub Actions
- Adding more zkVMs as your benchmark suite grows

---

**🎉 Congratulations!** Your zkEVM benchmark website is ready for the world to see! 