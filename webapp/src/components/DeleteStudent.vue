<template>
  <div class="container">
    <div class="row">
      <div class="col offset-s3">
        <h3>Delete Student</h3>
      </div>
    </div>
      <div class="row">
        <div class="input-field col s6 offset-s3">
          <input v-model="sid" id="sid" type="text" class="validate">
          <label for="sid">Student ID</label>
        </div>
      </div>
    <div class="row">
      <div class="col offset-s3">
        <button @click="deleteStudent" id="submit" class="btn waves-effect waves-light green offset-s3">Submit</button>
        <p class="error" v-if="error">{{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import api from '@/api.js'

export default {
  name: 'DeleteStudent',
  data () {
    return {
      sid: '',
      error: ''
    }
  },
  methods: {
    deleteStudent () {
      api.deleteStudent(this.sid)
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
