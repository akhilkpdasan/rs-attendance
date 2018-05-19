<template>
  <table>
    <Student v-for="student in students" :key="student.id" :student="student"/>
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
    api.get('/students').then(res => {
      this.students = res.data
    })
  }
}
</script>

<style>
table {
  border: 1px solid;
  border-collapse: collapse;
}

td {
  border: 1px solid;
}
</style>
