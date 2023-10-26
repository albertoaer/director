import type { FSChild } from "./fs";
import type { Unit } from "./units";

export const AlertItems = ['folders', 'files', 'all'] as const;

export type AlertItem = typeof AlertItems[number];

export interface Alert {
  name: string,
  filter: {
    minSize: number,
    sizeUnit: Unit,
    item: AlertItem,
  }
}

export interface Detection {
  alert: Alert,
  child: FSChild
}