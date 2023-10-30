import { writable, type Writable } from "svelte/store";
import type { Detection } from "../model/alert";
import type { FSChild } from "../model/fs";

export class DetectionList {
  private readonly registry: Set<string> = new Set();
  private readonly childs: Writable<FSChild[]> = writable([]);
  private length: number = 0;

  push(...detections: Detection[]) {
    this.childs.update(x => {
      for (const detection of detections) {
        if (this.registry.has(detection.child.path)) continue;
        this.registry.add(detection.child.path);
        this.length = x.length;
        
        x.push(detection.child);
      }
      return x;
    })
  }

  clear() {
    this.length = 0;
    this.childs.update(_ => {
      this.registry.clear();

      return [];
    });
  }

  get count(): number {
    return this.length;
  }

  readonly subscribe = this.childs.subscribe;
}