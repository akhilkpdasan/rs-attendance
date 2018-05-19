<template>
  <nav>
    <div class="nav-wrapper green">
      <a href="#" class="brand-logo center">Attendance</a>
      <ul id="nav-mobile" class="right">
        <li v-if="username">
          <a @click="logout">Logout</a>
        </li>
        <li v-else >
          <router-link to="login">Login</router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script>
import api from '@/api.js'
import store from '@/store'

export default {
  name: 'Nav',
  computed: {
    username: () => {
      return store.state.username
    }
  },
  methods: {
    logout: () => {
      api.logout()
      window.location.reload(true)
    }
  }
}

</script>

<style>
</style>
