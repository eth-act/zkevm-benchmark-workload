import { defineConfig } from 'vocs'

export default defineConfig({
  title: 'zkGas profiling',
  description: 'Comprehensive profiling framework for measuring and comparing the resources needed for proving different OPCODEs in zk environments across various gas categories.',
  sidebar: [
    {
      text: 'Getting Started',
      link: '/getting-started',
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
              text: 'airbender',
              link: '/benchmark-results/gas-categorized/1m/airbender-1M-4',
            },
          ],
        },
      ]
    },
   
  ],
})
