<template>
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
</template>

<script>
import Student from './Student.vue'
import api from '@/api.js'

export default {
  components: {
    Student
  },
  props: ['username'],
  data () {
    return {
      students: []
    }
  },
  mounted () {
    api.getStudents().then(response => {
      this.students = response.data
    })
    // console.log(getStudents())
    // TODO catch authorization error
  }
}
</script>
