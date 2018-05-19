<template>
  <nav>
    <div class="nav-wrapper green">
      <a href="#" class="brand-logo center">Student Attendance Management System</a>
      <ul id="nav-mobile" class="right">
        <li v-if="username">
          <router-link to="logout">Logout</router-link>
        </li>
        <li v-else >
          <router-link to="login">Login</router-link>
          <router-link to="register">Register</router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script>
export default {
  name: 'Nav',
  props:
    {
      username: String
    }
}

</script>

<style>
</style>
