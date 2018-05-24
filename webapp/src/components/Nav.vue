<template>
  <nav>
    <div class="nav-wrapper green">
      <a href="#" class="brand-logo center">Attendance management</a>
      <ul id="nav-mobile" class="right">
        <template v-if="username">
          <li>
            <router-link tag="li" to="/students"><a>Students</a></router-link>
          <li>
            <router-link tag="li" to="/new"><a>New</a></router-link>
          </li>
          <li>
            <router-link tag="li" to="/update"><a>Update</a></router-link>
          </li>
          <li>
            <router-link tag="li" to="/delete"><a>Delete</a></router-link>
          </li>
          <li>
            <a @click="logout">Logout</a>
          </li>
        </template>
        <template v-else>
          <li>
            <router-link tag="li" to="/login"><a>Login</a></router-link>
          </li>
          <li>
            <router-link tag="li" to="/register"><a>Register</a></router-link>
          </li>
        </template>
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
        this.$router.push('/login')
      })
    }
  }
}

</script>

<style>
</style>
