#!/usr/bin/env python3
"""
Generate zkEVM Analytics

Simple script to generate comprehensive analytics JSON files for all zkVMs.
"""

import sys
from pathlib import Path
from zkvm_analyzer import ZKVMAnalyzer

def main():
    print("🚀 zkEVM Analytics Generator")
    print("=" * 40)
    
    # Set up paths - assumes we're in python-analytics directory
    # and zkevm-metrics is in the parent directory
    metrics_dir = Path(__file__).parent.parent / "zkevm-metrics"
    output_dir = Path(__file__).parent / "analytics_output"
    
    print(f"📂 Metrics directory: {metrics_dir}")
    print(f"📁 Output directory: {output_dir}")
    print()
    
    try:
        # Initialize analyzer
        analyzer = ZKVMAnalyzer(str(metrics_dir))
        
        # Load all data
        analyzer.load_all_data()
        
        # Generate analytics for all zkVMs
        analyzer.generate_all_analytics(str(output_dir))
        
        print()
        print("🎉 Analytics generation completed successfully!")
        print(f"📊 Check the files in: {output_dir}/")
        
        # Show what was generated
        if output_dir.exists():
            print("\n📋 Generated files:")
            for file in sorted(output_dir.glob("*.json")):
                print(f"   📄 {file.name}")
        
    except FileNotFoundError as e:
        print(f"❌ Error: {e}")
        print(f"   Make sure the zkevm-metrics directory exists at: {metrics_dir}")
        sys.exit(1)
    
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main() 