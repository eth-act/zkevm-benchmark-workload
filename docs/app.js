class ZKVMDashboard {
    constructor() {
        this.analytics = {};
        this.summary = {};
        this.loadingOverlay = document.getElementById('loading-overlay');
        this.init();
    }

    async init() {
        try {
            await this.loadData();
            this.populateDashboard();
            this.hideLoading();
        } catch (error) {
            console.error('Failed to load dashboard data:', error);
            this.showError('Failed to load benchmark data. Please check that the analytics files are available.');
        }
    }

    async loadData() {
        try {
            // Load summary data
            const summaryResponse = await fetch('./analytics_output/summary_analytics.json');
            if (!summaryResponse.ok) {
                throw new Error('Failed to load summary analytics');
            }
            this.summary = await summaryResponse.json();
        } catch (error) {
            throw new Error('Failed to load summary analytics');
        }

        // Load individual zkVM analytics
        for (const zkvm of this.summary.zkvms) {
            try {
                const response = await fetch(`./analytics_output/${zkvm}_analytics.json`);
                if (response.ok) {
                    this.analytics[zkvm] = await response.json();
                }
            } catch (error) {
                console.warn(`Failed to load analytics for ${zkvm}:`, error);
            }
        }
    }

    populateDashboard() {
        this.populateHeroStats();
        this.populateZKVMCards();
        this.populateCategoriesAnalysis();
        this.populateErrorsAnalysis();
        this.updateLastUpdated();
    }

    populateHeroStats() {
        const totalZKVMs = this.summary.zkvms.length;
        const totalTests = Object.values(this.summary.comparison)
            .reduce((sum, zkvm) => sum + zkvm.total_tests, 0);
        const avgSuccessRate = Object.values(this.summary.comparison)
            .reduce((sum, zkvm) => sum + zkvm.success_rate_percent, 0) / totalZKVMs;

        document.getElementById('total-zkvms').textContent = totalZKVMs;
        document.getElementById('total-tests').textContent = totalTests.toLocaleString();
        document.getElementById('avg-success-rate').textContent = `${avgSuccessRate.toFixed(1)}%`;
    }

    populateZKVMCards() {
        const container = document.getElementById('zkvm-grid');
        container.innerHTML = '';

        for (const zkvm of this.summary.zkvms) {
            const analytics = this.analytics[zkvm];
            if (!analytics) continue;

            const card = this.createZKVMCard(analytics);
            container.appendChild(card);
        }
    }

    createZKVMCard(analytics) {
        const successRate = analytics.summary.success_rate_percent;
        const successClass = successRate >= 70 ? 'success-high' : 
                           successRate >= 40 ? 'success-medium' : 'success-low';

        const card = document.createElement('div');
        card.className = 'zkvm-card fade-in';
        card.style.cursor = 'pointer';
        
        // Add click handler to navigate to detailed page
        card.addEventListener('click', () => {
            window.location.href = `zkvm.html?zkvm=${analytics.zkvm_name.toLowerCase()}`;
        });
        
        const performanceData = analytics.performance.has_timing_data ? 
            analytics.performance.proving_time_seconds : null;

        card.innerHTML = `
            <div class="zkvm-header">
                <h3 class="zkvm-name">${analytics.zkvm_name}</h3>
                <span class="success-badge ${successClass}">
                    ${successRate.toFixed(1)}% Success
                </span>
            </div>
            
            <div class="zkvm-metrics">
                <div class="metric">
                    <div class="metric-value">${analytics.summary.total_tests}</div>
                    <div class="metric-label">Total Tests</div>
                </div>
                <div class="metric">
                    <div class="metric-value">${analytics.summary.successful_tests}</div>
                    <div class="metric-label">Successful</div>
                </div>
                <div class="metric">
                    <div class="metric-value">${analytics.summary.failed_tests}</div>
                    <div class="metric-label">Failed</div>
                </div>
                <div class="metric">
                    <div class="metric-value">
                        ${performanceData ? `${performanceData.mean.toFixed(1)}s` : 'N/A'}
                    </div>
                    <div class="metric-label">Avg Time</div>
                </div>
            </div>

            ${this.createProgressBar(successRate)}

            <div class="categories-breakdown">
                <h4 style="margin-bottom: 0.75rem; font-size: 0.875rem; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.05em;">
                    Test Categories
                </h4>
                ${Object.entries(analytics.categories)
                    .map(([category, stats]) => `
                        <div class="category-item">
                            <span class="category-name">${category.replace('_', ' ')}</span>
                            <span class="category-success" style="color: ${this.getSuccessColor(stats.success_rate_percent)};">
                                ${stats.successful}/${stats.total} (${stats.success_rate_percent.toFixed(1)}%)
                            </span>
                        </div>
                    `).join('')}
            </div>
            
            <div class="view-details-btn">
                <span style="font-size: 0.875rem; color: var(--primary-color); font-weight: 500;">
                    Click to view detailed results →
                </span>
            </div>
        `;

        return card;
    }

    createProgressBar(percentage) {
        const progressClass = percentage >= 70 ? 'progress-success' : 
                            percentage >= 40 ? 'progress-warning' : 'progress-error';
        
        return `
            <div class="progress-bar">
                <div class="progress-fill ${progressClass}" style="width: ${percentage}%"></div>
            </div>
        `;
    }

    getSuccessColor(percentage) {
        if (percentage >= 70) return 'var(--secondary-color)';
        if (percentage >= 40) return 'var(--warning-color)';
        return 'var(--error-color)';
    }

    populateCategoriesAnalysis() {
        const container = document.getElementById('categories-container');
        container.innerHTML = '';

        // Collect all categories across zkVMs
        const categories = new Set();
        Object.values(this.analytics).forEach(analytics => {
            Object.keys(analytics.categories).forEach(category => categories.add(category));
        });

        categories.forEach(category => {
            const card = this.createCategoryCard(category);
            container.appendChild(card);
        });
    }

    createCategoryCard(category) {
        const card = document.createElement('div');
        card.className = 'category-card fade-in';

        const zkrmData = this.summary.zkvms.map(zkvm => {
            const analytics = this.analytics[zkvm];
            const categoryStats = analytics?.categories[category];
            
            if (!categoryStats) return null;

            return {
                zkvm,
                ...categoryStats
            };
        }).filter(Boolean);

        const totalTests = zkrmData.reduce((sum, data) => sum + data.total, 0);
        const totalSuccessful = zkrmData.reduce((sum, data) => sum + data.successful, 0);
        const avgSuccessRate = totalTests > 0 ? (totalSuccessful / totalTests * 100) : 0;

        card.innerHTML = `
            <div class="category-header">
                <h4 class="category-title">${category.replace('_', ' ')}</h4>
                <div style="display: flex; justify-content: space-between; margin-bottom: 1rem;">
                    <span style="font-size: 0.875rem; color: var(--text-secondary);">
                        ${totalTests} total tests
                    </span>
                    <span style="font-size: 0.875rem; font-weight: 600; color: ${this.getSuccessColor(avgSuccessRate)};">
                        ${avgSuccessRate.toFixed(1)}% avg success
                    </span>
                </div>
            </div>

            <div class="category-zkvm-list">
                ${zkrmData.map(data => `
                    <div class="category-zkvm-item">
                        <span style="font-weight: 500; text-transform: uppercase; font-size: 0.875rem;">
                            ${data.zkvm}
                        </span>
                        <div style="text-align: right;">
                            <div style="font-weight: 600; color: ${this.getSuccessColor(data.success_rate_percent)};">
                                ${data.success_rate_percent.toFixed(1)}%
                            </div>
                            <div style="font-size: 0.75rem; color: var(--text-muted);">
                                ${data.successful}/${data.total}
                            </div>
                        </div>
                    </div>
                `).join('')}
            </div>
        `;

        return card;
    }

    populateErrorsAnalysis() {
        const container = document.getElementById('errors-container');
        container.innerHTML = '';

        // Collect all error patterns across zkVMs
        const errorPatterns = new Map();

        Object.values(this.analytics).forEach(analytics => {
            if (analytics.errors.error_distribution) {
                analytics.errors.error_distribution.forEach(error => {
                    if (errorPatterns.has(error.pattern)) {
                        errorPatterns.set(error.pattern, 
                            errorPatterns.get(error.pattern) + error.count
                        );
                    } else {
                        errorPatterns.set(error.pattern, error.count);
                    }
                });
            }
        });

        // Sort by count and display top error patterns
        const sortedErrors = Array.from(errorPatterns.entries())
            .sort((a, b) => b[1] - a[1])
            .slice(0, 6);

        const totalErrors = Array.from(errorPatterns.values()).reduce((sum, count) => sum + count, 0);

        sortedErrors.forEach(([pattern, count]) => {
            const percentage = totalErrors > 0 ? (count / totalErrors * 100) : 0;
            const card = this.createErrorCard(pattern, count, percentage);
            container.appendChild(card);
        });
    }

    createErrorCard(pattern, count, percentage) {
        const card = document.createElement('div');
        card.className = 'error-card fade-in';

        card.innerHTML = `
            <div class="error-pattern">${pattern}</div>
            <div class="error-count">${count}</div>
            <div class="error-percentage">${percentage.toFixed(1)}% of all errors</div>
            <div class="progress-bar" style="margin-top: 1rem;">
                <div class="progress-fill progress-error" style="width: ${percentage}%"></div>
            </div>
        `;

        return card;
    }

    updateLastUpdated() {
        const timestamp = this.summary.generated_at;
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

// Smooth scrolling for navigation links
document.addEventListener('DOMContentLoaded', () => {
    // Initialize dashboard
    new ZKVMDashboard();

    // Add smooth scrolling for navigation
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });

    // Add intersection observer for animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('fade-in');
            }
        });
    }, observerOptions);

    // Observe all sections for animation
    document.querySelectorAll('section').forEach(section => {
        observer.observe(section);
    });
}); 