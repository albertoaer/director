import type { Unit } from "./units";

export const AlarmItems = ['folders', 'files', 'all'] as const;

export type AlarmItem = typeof AlarmItems[number];

export interface Alarm {
  name: string,
  filter: {
    minSize: number,
    sizeUnit: Unit,
    item: AlarmItem,
  }
}