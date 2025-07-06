class ZKVMDetailPage {
    constructor(zkvmName) {
        this.zkvmName = zkvmName;
        this.analytics = null;
        this.allTests = [];
        this.filteredTests = [];
        this.loadingOverlay = document.getElementById('loading-overlay');
        this.init();
    }

    async init() {
        try {
            await this.loadData();
            this.populatePage();
            this.setupFilters();
            this.hideLoading();
        } catch (error) {
            console.error('Failed to load zkVM data:', error);
            this.showError(`Failed to load ${this.zkvmName.toUpperCase()} data. Please check that the analytics files are available.`);
        }
    }

    async loadData() {
        try {
            const response = await fetch(`../analytics_output/${this.zkvmName}_analytics.json`);
            this.analytics = await response.json();
            this.allTests = this.analytics.individual_tests;
            this.filteredTests = [...this.allTests];
        } catch (error) {
            throw new Error(`Failed to load ${this.zkvmName} analytics`);
        }
    }

    populatePage() {
        this.populateHeroStats();
        this.populatePerformanceSummary();
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

        // Create category performance cards
        Object.entries(this.analytics.categories).forEach(([category, stats]) => {
            const card = this.createCategoryCard(category, stats);
            container.appendChild(card);
        });

        // Create fastest/slowest tests card if we have performance data
        if (this.analytics.performance.has_timing_data) {
            const performanceCard = this.createPerformanceRankingsCard();
            container.appendChild(performanceCard);
        }
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
                ${stats.performance.test_count > 0 ? `
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
        card.className = 'category-card fade-in';

        const rankings = this.analytics.rankings;
        
        card.innerHTML = `
            <div class="category-header">
                <h4 class="category-title">Performance Rankings</h4>
            </div>
            <div class="rankings-content">
                <div class="ranking-section">
                    <h5 style="color: var(--secondary-color); margin-bottom: 0.5rem;">🚀 Fastest Tests</h5>
                    ${rankings.fastest.slice(0, 3).map(test => `
                        <div class="ranking-item">
                            <span class="test-name">${this.truncateTestName(test.name)}</span>
                            <span class="test-time">${test.proving_time_seconds.toFixed(1)}s</span>
                        </div>
                    `).join('')}
                </div>
                <div class="ranking-section">
                    <h5 style="color: var(--error-color); margin-bottom: 0.5rem;">🐌 Slowest Tests</h5>
                    ${rankings.slowest.slice(0, 3).map(test => `
                        <div class="ranking-item">
                            <span class="test-name">${this.truncateTestName(test.name)}</span>
                            <span class="test-time">${test.proving_time_seconds.toFixed(1)}s</span>
                        </div>
                    `).join('')}
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

        card.innerHTML = `
            <div class="test-header">
                <div class="test-info">
                    <h4 class="test-name">${test.name}</h4>
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

    truncateTestName(name, maxLength = 30) {
        if (name.length <= maxLength) return name;
        return name.substring(0, maxLength) + '...';
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
                <button onclick="location.reload()" style="
                    margin-top: 1.5rem; 
                    padding: 0.75rem 1.5rem; 
                    background: var(--primary-color); 
                    color: white; 
                    border: none; 
                    border-radius: var(--radius-md); 
                    cursor: pointer;
                    font-weight: 500;
                ">
                    Retry
                </button>
            </div>
        `;
    }
} 