import { writable, type Writable } from "svelte/store";
import type { Detection } from "../model/alert";
import type { FSChild } from "../model/fs";

export class DetectionList {
  private readonly registry: Map<string, Detection> = new Map();
  private readonly childs: Writable<FSChild[]> = writable([]);
  private length: number = 0;

  push(detection: Detection) {
    if (this.registry.has(detection.child.path)) return;
    this.registry.set(detection.child.path, detection);
    this.childs.update(x => {
      this.length = x.length;
      x.push(detection.child);
      return x;
    })
  }

  clear() {
    this.registry.clear();
    this.childs.set([]);
  }

  get count(): number {
    return this.length;
  }

  readonly subscribe = this.childs.subscribe;
}