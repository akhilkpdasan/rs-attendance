<template>
  <div id="app">
<!--<img src="./assets/logo.png">-->
  <Navbar :username=username />
    <router-view />
  </div>
</template>

<script>
import Navbar from '@/components/Nav.vue'

export default {
  name: 'App',
  components: {
    Navbar
  },
  data () {
    return {
      username: ''
    }
  }
}
</script>

<style>
</style>
