export interface Unit {
  symbol: string,
  factor: number
}

export const Units: Unit[] = [
  {
    symbol: 'B',
    factor: 0
  },
  {
    symbol: 'KB',
    factor: 3
  },
  {
    symbol: 'MB',
    factor: 6
  },
  {
    symbol: 'GB',
    factor: 9
  },
  {
    symbol: 'TB',
    factor: 12
  }
];

export function converFromBytes(value: number, factor: number) {
  return value * 1 / Math.pow(10, factor);
}

export function formatBytes(value: number, unit: Unit) {
  return converFromBytes(value, unit.factor) + unit.symbol;
}