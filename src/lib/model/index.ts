export interface RouteItem {
  name: string,
  path: string
}

export interface Route {
  items: RouteItem[],
  path: string,
  prefixed: boolean
}

export function routeToString(route: Route) {
  let line = route.items.map(x => x.name).join('/');
  return route.prefixed ? line : '/' + line;
}

export interface FSChild {
  name: string,
  path: string,
  size: {
    status: 'NotCalculated' | 'Calculating',
    value: undefined
  } | {
    status: 'Known' | 'Calculated',
    value: number
  },
  modified: number | null,
  created: number | null,
  type: 'file' | 'directory' | 'link' | 'other'
}

export interface FSEvent {
  entry?: {
    path: string,
    childs: FSChild[]
  }
}