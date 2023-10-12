export interface RouteItem {
  name: string,
  path: string
}

export interface Route {
  items: RouteItem[],
  fullName: string,
  prefixed: boolean
}

export interface FileItem extends RouteItem {
  size?: number
}

export function routeToString(route: Route) {
  let line = route.items.map(x => x.name).join('/');
  return route.prefixed ? line : '/' + line;
}