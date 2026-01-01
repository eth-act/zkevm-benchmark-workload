import {
  useEffect,
  useState,
} from 'react';

interface CsvTableProps {
  src: string;
  columns?: string[]; // Optional: show only specific columns
  maxRows?: number;   // Optional: limit number of rows
}

export function CsvTable({ src, columns, maxRows }: CsvTableProps) {
  const [data, setData] = useState<string[][]>([]);
  const [headers, setHeaders] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch(src)
      .then((res) => {
        if (!res.ok) throw new Error(`Failed to load CSV: ${res.status}`);
        return res.text();
      })
      .then((text) => {
        const lines = text.trim().split('\n');
        const allHeaders = lines[0].split(',');
        const rows = lines.slice(1).map((line) => line.split(','));

        // Filter columns if specified
        if (columns && columns.length > 0) {
          const indices = columns.map((col) => allHeaders.indexOf(col)).filter((i) => i !== -1);
          setHeaders(indices.map((i) => allHeaders[i]));
          setData(rows.map((row) => indices.map((i) => row[i])));
        } else {
          setHeaders(allHeaders);
          setData(rows);
        }
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, [src, columns]);

  if (loading) return <div style={{ padding: '1rem', color: '#888' }}>Loading CSV...</div>;
  if (error) return <div style={{ padding: '1rem', color: '#f44' }}>Error: {error}</div>;

  const displayData = maxRows ? data.slice(0, maxRows) : data;

  return (
    <div style={{ overflowX: 'auto', marginBlock: '1rem' }}>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '0.875rem' }}>
        <thead>
          <tr style={{ backgroundColor: 'var(--vocs-color_background2)' }}>
            {headers.map((header, i) => (
              <th
                key={i}
                style={{
                  padding: '0.5rem 0.75rem',
                  textAlign: 'left',
                  borderBottom: '2px solid var(--vocs-color_border)',
                  whiteSpace: 'nowrap',
                }}
              >
                {header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {displayData.map((row, rowIndex) => (
            <tr
              key={rowIndex}
              style={{
                backgroundColor: rowIndex % 2 === 0 ? 'transparent' : 'var(--vocs-color_background2)',
              }}
            >
              {row.map((cell, cellIndex) => (
                <td
                  key={cellIndex}
                  style={{
                    padding: '0.5rem 0.75rem',
                    borderBottom: '1px solid var(--vocs-color_border)',
                    whiteSpace: 'nowrap',
                  }}
                >
                  {formatCell(cell)}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      {maxRows && data.length > maxRows && (
        <p style={{ marginTop: '0.5rem', color: '#888', fontSize: '0.875rem' }}>
          Showing {maxRows} of {data.length} rows.{' '}
          <a href={src} download style={{ color: 'var(--vocs-color_textAccent)' }}>
            Download full CSV
          </a>
        </p>
      )}
    </div>
  );
}

function formatCell(value: string): string {
  const num = parseFloat(value);
  if (isNaN(num)) return value;
  
  // Format large numbers
  if (Math.abs(num) >= 1e9) return (num / 1e9).toFixed(2) + 'B';
  if (Math.abs(num) >= 1e6) return (num / 1e6).toFixed(2) + 'M';
  if (Math.abs(num) >= 1e3) return (num / 1e3).toFixed(2) + 'K';
  
  // Format small decimals (like R² values)
  if (Math.abs(num) < 1 && Math.abs(num) > 0) return num.toFixed(4);
  
  // Format scientific notation
  if (value.includes('e')) return num.toExponential(2);
  
  return num.toFixed(2);
}

export default CsvTable;

