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
          text: '1M',
          items: [
            {
              text: 'sp1',
              link: '/benchmark-results/gas-categorized/1m/1m-sp1',
            },
            {
              text: 'risc0',
              link: '/benchmark-results/gas-categorized/1m/1m-risc0',
            },
            {
              text: 'summary',
              link: '/benchmark-results/gas-categorized/1m/1m-summary',
            },
          ],
        },
      ]
    },
    {
      text: 'GPU Scaling Efficiency',
      items: [
        {
          text: '1M Risc0 GPU Scaling Efficiency',
          items: [
            {
              text: 'Proving Times',
              link: '/gpu-scaling-efficiency/1m-risc0-proving-times',
            },
            {
              text: 'Speedup & Efficiency',
              link: '/gpu-scaling-efficiency/1m-risc0-speedup-efficiency',
            },
          ],
        },
      ]
    },
  ],
})
