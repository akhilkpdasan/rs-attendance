<template>
  <div class="container">
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>Name</th>
          <th>Roll No</th>
          <th>Attendance</th>
        </tr>
      </thead>
      <tbody>
        <Student v-for="student in students" :key="student.id" :student="student"/>
      </tbody>
    </table>
    <p v-if="error" class="error">{{ error }}</p>
  </div>
</template>

<script>
import Student from './Student.vue'
import api from '@/api.js'

export default {
  components: {
    Student
  },
  data () {
    return {
      students: [],
      error: ''
    }
  },
  mounted () {
    api.getStudents().then(response => {
      this.students = response.data
    })
      .catch(error => {
        if (error.response) {
          if (error.response.status === 401) {
            this.$router.push('login')
          } else {
            this.error = 'Interenal server error occured'
          }
        } else {
          this.error = 'Unknown error occured'
        }
      })
  }
}
</script>

<style>
.error {
  color: tomato
}
</style>
