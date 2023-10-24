import { writable, type Writable } from "svelte/store";
import type { Detection } from "../model/alert";
import type { FSChild } from "../model/fs";

export class DetectionList {
  private readonly registry: Map<string, Detection> = new Map();
  private readonly childs: Writable<FSChild[]> = writable([]);

  push(detection: Detection) {
    if (this.registry.has(detection.child.path)) return;
    this.registry.set(detection.child.path, detection);
    this.childs.update(x => {
      x.push(detection.child);
      return x;
    })
  }

  clear() {
    this.registry.clear();
    this.childs.set([]);
  }

  readonly subscribe = this.childs.subscribe;
}