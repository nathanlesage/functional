<template>
  <div id="wrapper" ref="wrapper">
  </div>
</template>

<script setup lang="ts">
import { watch, toRef, ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { invoke } from '@tauri-apps/api';

const props = defineProps({
  activeComponent: {
    type: String,
    required: true
  }
})

const currentPlugin = ref<PluginAPI|undefined>(undefined)

const wrapper = ref<HTMLDivElement>()

interface PluginAPI {
  dom: Element
}

watch(toRef(props, 'activeComponent'), () => {
  loadComponent().catch(err => console.error(err))
})

async function loadComponent (): Promise<void> {
  const pluginJs: string = await invoke('get_plugin_js', { pluginPath: props.activeComponent })
  import(/* @vite-ignore */ convertFileSrc(pluginJs, 'asset')).then(mod => {
    currentPlugin.value = mod.default()
    if (wrapper.value !== undefined && currentPlugin.value !== undefined) {
      wrapper.value.innerHTML = ''
      wrapper.value.appendChild(currentPlugin.value.dom)
    }
  })
  .catch(err => console.error(err))
}

// Initial component loading
loadComponent()
</script>

<style scoped>
#wrapper {
  flex-grow: 1;
  border-top: 1px solid rgb(200, 200, 200);
  border-left: 1px solid rgb(200, 200, 200);
}

@media (prefers-color-scheme: dark) {
  #wrapper {
    border-top-color: rgb(70, 70, 70);
    border-left-color: rgb(70, 70, 70);
  }
}
</style>
