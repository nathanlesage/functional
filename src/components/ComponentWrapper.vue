<template>
  <div id="wrapper">
    Active component: {{  currentPlugin?.name }}<br>
    Plugin info string: {{ currentPlugin?.description ?? 'None loaded' }}<br>
  </div>
</template>

<script setup lang="ts">
import { watch, toRef, ref } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const props = defineProps({
  activeComponent: {
    type: String,
    required: true
  }
})

const currentPlugin = ref<PluginAPI|undefined>(undefined)

const dataDir = ref('none')

interface PluginAPI {
  name: string
  description: string
  author: string
}

watch(toRef(props, 'activeComponent'), () => {
  loadComponent()
})

function loadComponent (): void {
  console.log(convertFileSrc(props.activeComponent, 'asset'))
  import(/* @vite-ignore */ convertFileSrc(props.activeComponent, 'asset')).then(mod => {
    currentPlugin.value = mod.default()
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
