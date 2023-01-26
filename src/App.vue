<template>
  <div id="vertical-wrapper">
    <header>
      <h1>Functional</h1>
    </header>
    <div id="wrapper">
      <nav>
        <a
          v-for="c in components"
          :key="c.id"
          :class="{
            active: activeComponent === c.id
          }"
          @click="loadComponent(c.id)"
        >
          {{ c.label }}
        </a>
      </nav>

      <ComponentWrapper
        v-if="activeComponent !== undefined"
        :active-component="activeComponent"
      />
      <div v-else>
        No plugin loaded.
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import ComponentWrapper from './components/ComponentWrapper.vue'

interface PluginDescriptor {
  id: string
  label: string
}

const components = ref<PluginDescriptor[]>([])
const activeComponent = ref<string|undefined>(undefined)

invoke('list_plugins')
  .then(availablePlugins => {
    for (const plugin of availablePlugins as string[]) {
      const pathComponents = plugin.split(/[\/\\]/g)
      console.log(pathComponents)
      const basename = pathComponents[pathComponents.length - 1]
      const withoutExt = basename.substring(0, basename.lastIndexOf('.'))
      components.value.push({
        id: plugin,
        label: withoutExt
      })
    }
  })
  .catch(err => console.error(err))

function loadComponent (componentId: string): void {
  console.log('Loading component', componentId)
  activeComponent.value = componentId
}
</script>

<style scoped>
#vertical-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
}

#wrapper {
  display: flex;
  flex-direction: row;
  height: 100%;
  padding: 0;
  margin: 0;
}

header {
  font-size: 10px;
  background-color: rgb(250, 250, 250);
}

nav {
  width: 80px;
  height: 100%;
  background-color: rgb(250, 250, 250);
  overflow-y: auto;
}

nav a {
  display: block;
  height: 80px;
}

nav a:hover, nav a.active {
  background-color: rgb(220, 220, 220);
}

@media (prefers-color-scheme: dark) {
  header, nav {
    background-color: rgb(90, 90, 90);
    color: rgb(200, 200, 200);
  }

  nav a:hover, nav a.active {
    background-color: rgb(70, 70, 70);
  }
}

</style>
