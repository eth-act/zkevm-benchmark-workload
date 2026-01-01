import type { Plugin } from 'vite';
import { defineConfig } from 'vocs';

// Plugin to serve public assets at basePath in dev mode
function servePublicAtBasePath(): Plugin {
  return {
    name: 'serve-public-at-basepath',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        // Rewrite requests from basePath to root for public assets
        if (req.url?.startsWith('/zkevm-benchmark-workload/marginal-gas-benchmark/')) {
          req.url = req.url.replace('/zkevm-benchmark-workload', '');
        }
        next();
      });
    },
  };
}

export default defineConfig({
  basePath: '/zkevm-benchmark-workload',
  rootDir: './docs',
  vite: {
    plugins: [servePublicAtBasePath()],
  },
  title: 'zkGas profiling',
  description: 'Comprehensive profiling framework for measuring and comparing the resources needed for proving different OPCODEs in zk environments across various gas categories.',
  sidebar: [
    {
      text: 'Getting Started',
      link: '/getting-started',
    },
    {
      text: 'Stateless Executor Guide',
      link: '/stateless-executor-guide',
    },
    {
      text: 'Download Fixtures',
      link: '/download-fixtures',
    },
    {
      text: 'Gas Categorized Fixtures',
      link: '/gas-categorized-fixtures',
    },
    {
      text: 'Gas Categorized Benchmarks',
      link: '/gas-categorized-benchmarks',
    },
    {
      text: 'Single File Benchmark',
      link: '/single-file-benchmark',
    },
    {
      text: 'Markdown Tables',
      link: '/markdown-tables',
    },
    {
      text: 'Simplified Naming',
      link: '/simplified-naming',
    },
    {
      text: 'Export Comparison CSV',
      link: '/export-comparison-csv',
    },
    {
      text: 'Compare SP1 vs RISC0',
      link: '/compare-sp1-risc0',
    },
    {
      text: 'Marginal Gas Benchmark',
      items: [
        {
          text: 'Overview',
          link: '/marginal-gas-benchmark',
        },
        {
          text: 'SP1 v5.2.3',
          link: '/marginal-gas-benchmark/sp1',
        },
        {
          text: 'RISC0 v3.0.4',
          link: '/marginal-gas-benchmark/risc0',
        },
      ],
    },
    {
      text: 'Benchmark Results',
      items: [
        {
          text: '1M, 1GPU',
          link: '/benchmark-results/gas-categorized/1m/1M-1',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/1m/sp1-1M-1',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/1m/risc0-1M-1',
            },
          ],
        },
        {
          text: '1M, 4GPU',
          link: '/benchmark-results/gas-categorized/1m/1M-4',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/1m/sp1-1M-4',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/1m/risc0-1M-4',
            },
            {
              text: 'zisk',
              link: '/benchmark-results/gas-categorized/1m/zisk-1M-4',
            },
            {
              text: 'airbender',
              link: '/benchmark-results/gas-categorized/1m/airbender-1M-4',
            },
          ],
        },
        {
          text: '0.8M, 4GPU',
          link: '/benchmark-results/gas-categorized/0.8m/0.8M-4',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/0.8m/sp1-0.8M-4',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/0.8m/risc0-0.8M-4',
            },
          ],
        },
        {
          text: '0.6M, 4GPU',
          link: '/benchmark-results/gas-categorized/0.6m/0.6M-4',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/0.6m/sp1-0.6M-4',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/0.6m/risc0-0.6M-4',
            },
          ],
        },
        {
          text: '0.4M, 4GPU',
          link: '/benchmark-results/gas-categorized/0.4m/0.4M-4',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/0.4m/sp1-0.4M-4',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/0.4m/risc0-0.4M-4',
            },
          ],
        },
        {
          text: '0.2M, 4GPU',
          link: '/benchmark-results/gas-categorized/0.2m/0.2M-4',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/0.2m/sp1-0.2M-4',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/0.2m/risc0-0.2M-4',
            },
          ],
        },
        {
          text: '0.1M, 4GPU',
          link: '/benchmark-results/gas-categorized/0.1m/0.1M-4',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/0.1m/sp1-0.1M-4',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/0.1m/risc0-0.1M-4',
            },
          ],
        },
      ],
    },
   
  ],
})
