<template>
  <div class="container">
    <div class="row">
      <div class="col offset-s3">
        <h3>Login</h3>
      </div>
    </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="username" id="username" type="text" class="validate">
          <label for="username">Username</label>
        </div>
      </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="password" id="password" type="password" class="validate">
          <label for="password">Password</label>
        </div>
      </div>
    <div class="row">
      <div class="col offset-s3">
        <button @click="login" id="submit" class="btn waves-effect waves-light green offset-s3">Submit</button>
        <p class="error" v-if="error">{{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import api from '@/api.js'

export default {
  name: 'Login',
  data () {
    return {
      username: '',
      password: '',
      error: ''
    }
  },
  methods: {
    login () {
      api.login(this.username, this.password)
        .then(response => {
          this.$store.dispatch('login', {username: this.username})
          this.$router.push('/students')
        })
        .catch(error => {
          this.error = error.msg
        })
    }
  }
}
</script>

<style>
h3 {
  color: grey;
}

.error {
  color: tomato;
}
</style>
