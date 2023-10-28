export interface Order {
  path: string,
  status: 'Unknown' | 'Running' | 'Finished'
}