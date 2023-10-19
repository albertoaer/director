import { getContext, setContext, hasContext } from "svelte";
import { writable, type Readable, type Writable } from "svelte/store";
import type { FSChild, Route } from "./model/fs";

const key = 'directory-context';

export class DirectoryContext {
  private _navigate?: (route: string) => void;
  private _route$: Writable<Route> = writable();
  private _childs$: Writable<FSChild[] | undefined> = writable();

  private constructor() { }

  setNavigate(navigate: (route: string) => void) {
    this._navigate = navigate;
  }

  navigate(route: string) {
    this._navigate?.(route);
  }

  pushRoute(route: Route) {
    this._route$.set(route);
  }

  pushChilds(childs: FSChild[] | undefined) {
    this._childs$.set(childs);
  }

  get route$(): Readable<Route> {
    return this._route$;
  }

  get childs$(): Readable<FSChild[] | undefined> {
    return this._childs$;
  }

  static getOrSet(): DirectoryContext {
    if (hasContext(key)) return getContext(key) as DirectoryContext;
    return setContext(key, new DirectoryContext());
  }
}