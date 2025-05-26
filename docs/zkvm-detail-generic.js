class GenericZKVMDetailPage {
    constructor() {
        this.zkvmName = this.getZKVMFromURL();
        this.analytics = null;
        this.allTests = [];
        this.filteredTests = [];
        this.allZKVMs = [];
        this.loadingOverlay = document.getElementById('loading-overlay');
        this.init();
    }

    getZKVMFromURL() {
        const urlParams = new URLSearchParams(window.location.search);
        const zkvm = urlParams.get('zkvm');
        
        if (!zkvm) {
            // Fallback: redirect to main dashboard if no zkVM specified
            window.location.href = 'index.html';
            return null;
        }
        
        return zkvm.toLowerCase();
    }

    async init() {
        if (!this.zkvmName) return;
        
        try {
            await this.loadAllZKVMs();
            await this.loadZKVMData();
            this.updatePageTitles();
            this.populatePage();
            this.setupFilters();
            this.hideLoading();
        } catch (error) {
            console.error('Failed to load zkVM data:', error);
            this.showError(`Failed to load ${this.zkvmName.toUpperCase()} data. Please check that the analytics files are available.`);
        }
    }

    async loadAllZKVMs() {
        try {
            const response = await fetch('./analytics_output/summary_analytics.json');
            const summary = await response.json();
            this.allZKVMs = summary.zkvms;
        } catch (error) {
            console.warn('Could not load zkVM list for navigation');
            this.allZKVMs = [];
        }
    }

    async loadZKVMData() {
        try {
            const response = await fetch(`./analytics_output/${this.zkvmName}_analytics.json`);
            if (!response.ok) {
                throw new Error(`zkVM "${this.zkvmName}" not found`);
            }
            this.analytics = await response.json();
            this.allTests = this.analytics.individual_tests;
            this.filteredTests = [...this.allTests];
        } catch (error) {
            throw new Error(`Failed to load ${this.zkvmName} analytics: ${error.message}`);
        }
    }

    updatePageTitles() {
        const displayName = this.zkvmName.toUpperCase();
        
        // Update page title
        document.getElementById('page-title').textContent = `${displayName} zkEVM - Detailed Results`;
        
        // Update navigation title
        document.getElementById('nav-title').textContent = `${displayName} zkEVM`;
        
        // Update hero section
        document.getElementById('hero-title').textContent = `${displayName} zkEVM`;
        document.getElementById('hero-description').textContent = 
            `Detailed test results and performance analysis for the ${displayName} Zero-Knowledge Virtual Machine.`;
        
        // Update test section description
        document.getElementById('tests-description').textContent = 
            `Detailed results for every test executed on ${displayName}`;
        
        // Update footer
        document.getElementById('footer-zkvm').textContent = displayName;
        
        // Update loading message
        document.getElementById('loading-message').textContent = `Loading ${displayName} test data...`;
        
        // Update navigation with other zkVMs
        this.updateNavigationLinks();
    }

    updateNavigationLinks() {
        const container = document.getElementById('other-zkvms-nav');
        container.innerHTML = '';
        
        // Add links to other zkVMs
        this.allZKVMs
            .filter(zkvm => zkvm.toLowerCase() !== this.zkvmName)
            .forEach(zkvm => {
                const link = document.createElement('a');
                link.href = `zkvm.html?zkvm=${zkvm.toLowerCase()}`;
                link.className = 'nav-link';
                link.textContent = `${zkvm.toUpperCase()} Results`;
                container.appendChild(link);
            });
    }

    populatePage() {
        this.populateHeroStats();
        this.populatePerformanceSummary();
        this.setupDynamicFilters();
        this.populateTestResults();
        this.updateLastUpdated();
    }

    populateHeroStats() {
        const summary = this.analytics.summary;
        const performance = this.analytics.performance;

        document.getElementById('total-tests').textContent = summary.total_tests;
        document.getElementById('successful-tests').textContent = summary.successful_tests;
        document.getElementById('success-rate').textContent = `${summary.success_rate_percent.toFixed(1)}%`;
        
        if (performance.has_timing_data) {
            document.getElementById('avg-time').textContent = `${performance.proving_time_seconds.mean.toFixed(1)}s`;
        } else {
            document.getElementById('avg-time').textContent = 'N/A';
        }
    }

    populatePerformanceSummary() {
        const container = document.getElementById('performance-grid');
        container.innerHTML = '';

        // Only show fastest/slowest tests card if we have performance data
        if (this.analytics.performance.has_timing_data && this.analytics.rankings) {
            const performanceCard = this.createPerformanceRankingsCard();
            container.appendChild(performanceCard);
        } else {
            // Show a message if no performance data is available
            container.innerHTML = `
                <div class="no-performance-data">
                    <p style="text-align: center; color: var(--text-secondary); font-style: italic;">
                        No performance timing data available for this zkVM
                    </p>
                </div>
            `;
        }
    }

    setupDynamicFilters() {
        const filtersContainer = document.getElementById('test-filters');
        
        // Get unique categories from the tests
        const categories = [...new Set(this.allTests.map(test => test.test_category))];
        
        // Add category filters after the existing status filters
        categories.forEach(category => {
            const button = document.createElement('button');
            button.className = 'filter-btn';
            button.dataset.filter = category;
            button.textContent = category.replace('_', ' ');
            filtersContainer.appendChild(button);
        });
    }

    createCategoryCard(category, stats) {
        const card = document.createElement('div');
        card.className = 'category-card fade-in';

        const successRate = stats.success_rate_percent;
        const successColor = this.getSuccessColor(successRate);

        card.innerHTML = `
            <div class="category-header">
                <h4 class="category-title">${category.replace('_', ' ')}</h4>
                <span class="success-badge" style="background-color: ${successColor};">
                    ${successRate.toFixed(1)}%
                </span>
            </div>
            <div class="category-metrics">
                <div class="metric">
                    <div class="metric-value">${stats.total}</div>
                    <div class="metric-label">Total Tests</div>
                </div>
                <div class="metric">
                    <div class="metric-value">${stats.successful}</div>
                    <div class="metric-label">Successful</div>
                </div>
                <div class="metric">
                    <div class="metric-value">${stats.failed}</div>
                    <div class="metric-label">Failed</div>
                </div>
                ${stats.performance && stats.performance.test_count > 0 ? `
                <div class="metric">
                    <div class="metric-value">${(stats.performance.mean_ms / 1000).toFixed(1)}s</div>
                    <div class="metric-label">Avg Time</div>
                </div>
                ` : ''}
            </div>
        `;

        return card;
    }

    createPerformanceRankingsCard() {
        const card = document.createElement('div');
        card.className = 'performance-leaderboard fade-in';

        const rankings = this.analytics.rankings;
        
        // Get top 5 fastest and slowest tests for the leaderboard
        const fastestTests = rankings.fastest.slice(0, 5);
        const slowestTests = rankings.slowest.slice(0, 5);
        
        card.innerHTML = `
            <div class="leaderboard-header">
                <h4 class="leaderboard-title">Performance Leaderboard</h4>
                <p class="leaderboard-subtitle">Fastest and slowest test execution times</p>
            </div>
            
            <div class="leaderboard-content">
                <div class="leaderboard-section fastest">
                    <div class="section-header">
                        <h5>🚀 Fastest Tests</h5>
                    </div>
                    <div class="leaderboard-list">
                        ${fastestTests.map((test, index) => `
                            <div class="leaderboard-item rank-${index + 1}">
                                <div class="rank-badge">${index + 1}</div>
                                <div class="test-info">
                                    <div class="test-name-full">${this.formatTestName(test.name)}</div>
                                    <div class="test-category-tag">${test.test_category?.replace('_', ' ') || 'Unknown'}</div>
                                </div>
                                <div class="test-time-large">${test.proving_time_seconds.toFixed(2)}s</div>
                            </div>
                        `).join('')}
                    </div>
                </div>
                
                <div class="leaderboard-section slowest">
                    <div class="section-header">
                        <h5>🐌 Slowest Tests</h5>
                    </div>
                    <div class="leaderboard-list">
                        ${slowestTests.map((test, index) => `
                            <div class="leaderboard-item rank-${index + 1}">
                                <div class="rank-badge">${index + 1}</div>
                                <div class="test-info">
                                    <div class="test-name-full">${this.formatTestName(test.name)}</div>
                                    <div class="test-category-tag">${test.test_category?.replace('_', ' ') || 'Unknown'}</div>
                                </div>
                                <div class="test-time-large">${test.proving_time_seconds.toFixed(2)}s</div>
                            </div>
                        `).join('')}
                    </div>
                </div>
            </div>
        `;

        return card;
    }

    populateTestResults() {
        const container = document.getElementById('tests-container');
        container.innerHTML = '';

        this.filteredTests.forEach(test => {
            const testCard = this.createTestCard(test);
            container.appendChild(testCard);
        });

        // Update test count in filters
        this.updateFilterCounts();
    }

    createTestCard(test) {
        const card = document.createElement('div');
        card.className = `test-card ${test.status} fade-in`;
        card.dataset.category = test.test_category;
        card.dataset.status = test.status;

        const statusBadge = test.status === 'success' ? 
            `<span class="status-badge success">✓ Success</span>` :
            `<span class="status-badge crashed">✗ Crashed</span>`;

        const timeInfo = test.proving_time_ms ? 
            `<div class="test-time">${(test.proving_time_ms / 1000).toFixed(1)}s</div>` :
            '';

        const errorInfo = test.error_reason ? 
            `<div class="error-reason">
                <strong>Error:</strong> ${test.error_reason}
            </div>` : '';

        // Use the full test name for cards (let CSS handle wrapping)
        const displayName = this.formatTestName(test.name);

        card.innerHTML = `
            <div class="test-header">
                <div class="test-info">
                    <h4 class="test-name">${displayName}</h4>
                    <div class="test-meta">
                        <span class="test-category">${test.test_category.replace('_', ' ')}</span>
                        ${statusBadge}
                    </div>
                </div>
                ${timeInfo}
            </div>
            ${errorInfo}
        `;

        return card;
    }

    setupFilters() {
        const filterButtons = document.querySelectorAll('.filter-btn');
        
        filterButtons.forEach(button => {
            button.addEventListener('click', (e) => {
                // Remove active class from all buttons
                filterButtons.forEach(btn => btn.classList.remove('active'));
                
                // Add active class to clicked button
                e.target.classList.add('active');
                
                // Apply filter
                const filter = e.target.dataset.filter;
                this.applyFilter(filter);
            });
        });
    }

    applyFilter(filter) {
        if (filter === 'all') {
            this.filteredTests = [...this.allTests];
        } else if (filter === 'success' || filter === 'crashed') {
            this.filteredTests = this.allTests.filter(test => test.status === filter);
        } else {
            // Category filter
            this.filteredTests = this.allTests.filter(test => test.test_category === filter);
        }

        this.populateTestResults();
    }

    updateFilterCounts() {
        const filterButtons = document.querySelectorAll('.filter-btn');
        
        filterButtons.forEach(button => {
            const filter = button.dataset.filter;
            let count;
            
            if (filter === 'all') {
                count = this.allTests.length;
            } else if (filter === 'success' || filter === 'crashed') {
                count = this.allTests.filter(test => test.status === filter).length;
            } else {
                count = this.allTests.filter(test => test.test_category === filter).length;
            }
            
            // Update button text with count
            const originalText = button.textContent.split(' (')[0];
            button.textContent = `${originalText} (${count})`;
        });
    }

    formatTestName(name) {
        // Clean up the test name for better display
        // Remove common prefixes and make it more readable
        return name
            .replace(/^tests?\//, '')           // Remove leading "test/" or "tests/"
            .replace(/zkevm\//, '')             // Remove "zkevm/" from path
            .replace(/_/g, ' ')                 // Replace underscores with spaces
            .replace(/\.rs$/, '')               // Remove .rs extension
            .trim();
    }

    truncateTestName(name, maxLength = 50) {
        // Increased max length and better truncation
        const formatted = this.formatTestName(name);
        if (formatted.length <= maxLength) return formatted;
        return formatted.substring(0, maxLength - 3) + '...';
    }

    getSuccessColor(percentage) {
        if (percentage >= 70) return 'var(--secondary-color)';
        if (percentage >= 40) return 'var(--warning-color)';
        return 'var(--error-color)';
    }

    updateLastUpdated() {
        const timestamp = this.analytics.generated_at;
        const date = new Date(timestamp);
        const formatted = date.toLocaleString();
        document.getElementById('last-updated').textContent = formatted;
    }

    hideLoading() {
        this.loadingOverlay.classList.add('hidden');
        setTimeout(() => {
            this.loadingOverlay.style.display = 'none';
        }, 300);
    }

    showError(message) {
        this.loadingOverlay.innerHTML = `
            <div style="text-align: center; color: var(--error-color);">
                <h3>Error Loading Data</h3>
                <p style="margin-top: 1rem; max-width: 400px;">${message}</p>
                <div style="margin-top: 1.5rem;">
                    <button onclick="location.href='index.html'" style="
                        margin-right: 1rem;
                        padding: 0.75rem 1.5rem; 
                        background: var(--primary-color); 
                        color: white; 
                        border: none; 
                        border-radius: var(--radius-md); 
                        cursor: pointer;
                        font-weight: 500;
                    ">
                        Back to Dashboard
                    </button>
                    <button onclick="location.reload()" style="
                        padding: 0.75rem 1.5rem; 
                        background: transparent; 
                        color: var(--primary-color); 
                        border: 1px solid var(--primary-color); 
                        border-radius: var(--radius-md); 
                        cursor: pointer;
                        font-weight: 500;
                    ">
                        Retry
                    </button>
                </div>
            </div>
        `;
    }
}

// Initialize the page
document.addEventListener('DOMContentLoaded', () => {
    new GenericZKVMDetailPage();
}); 