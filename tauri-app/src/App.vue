<script>
import TopMenu from './components/TopMenu.vue'

export default {
  name: 'App',
  components: {
    TopMenu
  }
}
</script>

<template>
      <router-view />
</template>

<style>
html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

#app {
  height: 100vh;
}

.app-container {
  min-height: 100vh;
  background-color: #f5f7fa;
}

.main-content {
  padding-top: 60px;
  padding-left: 20px;
  padding-right: 20px;
}
</style>
