<template>
  <nav>
    <div class="nav-wrapper green">
      <a href="#" class="brand-logo center">Attendance management</a>
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

export default {
  name: 'Nav',
  computed: {
    username () {
      return this.$store.state.username
    }
  },
  methods: {
    logout () {
      api.logout().then(response => {
        this.$store.dispatch('logout')
        this.$router.push('login')
      })
    }
  }
}

</script>

<style>
</style>
