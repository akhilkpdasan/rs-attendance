<template>
  <div class="container">
    <div class="row">
      <div class="col offset-s3">
        <h3>Create New Student</h3>
      </div>
    </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="sid" id="sid" type="text" class="validate">
          <label for="sid">Student ID</label>
        </div>
      </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="name" id="name" type="text" class="validate">
          <label for="name">Name</label>
        </div>
      </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="roll_no" id="roll_no" type="text" class="validate">
          <label for="roll_no">Roll No.</label>
        </div>
      </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="attendance" id="attendance" type="text" class="validate">
          <label for="attendance">Attendance</label>
        </div>
      </div>
    <div class="row">
      <div class="col offset-s3">
        <button @click="newStudent" id="submit" class="btn waves-effect waves-light green offset-s3">Submit</button>
        <p class="error" v-if="error">{{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import api from '@/api.js'

export default {
  name: 'NewStudent',
  data () {
    return {
      sid: '',
      name: '',
      roll_no: '',
      attendance: '',
      error: ''
    }
  },
  methods: {
    newStudent () {
      api.newStudent(this.sid, this.name, this.roll_no, this.attendance)
        .then(response => {
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
