<script lang="ts" context="module">
  import tippy, { createSingleton, type CreateSingletonInstance, type Instance, type Props } from 'tippy.js';
  import 'tippy.js/dist/tippy.css';
  
  export interface CustomTooltipProps {
    singleton: string;
  }

  const singletons: Map<string, [Instance[], CreateSingletonInstance]> = new Map();

  export function tooltip(element: HTMLElement, options: Partial<Props & CustomTooltipProps> | undefined = undefined) {
    const instance = tippy(element, options);
    if (options?.singleton) {
      const singleton = singletons.get(options?.singleton) ?? [[], createSingleton([])];
      singleton[0].push(instance);
      singleton[1].setInstances(singleton[0]);
      singletons.set(options?.singleton, singleton);
    }
    return {
      destroy() {
        instance.destroy();
      }
    }
  }
</script>