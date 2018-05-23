import axios from 'axios'

const config = axios.create({
  baseURL: 'http://localhost:8088',
  timeout: 5000,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json'
  }
})

export default {
  getStudents () {
    return config.get('/students')
  },

  login (username, password) {
    return config.post('/login', {'username': username, 'password': password})
  },

  logout () {
    return config.get('/logout')
  },

  newStudent (sid, name, rollNo, attendance) {
    let student = {
      'id': sid,
      'name': name,
      'roll_no': parseInt(rollNo),
      'attendance': parseFloat(attendance)
    }
    return config.post('/students', student)
  },

  whoami () {
    return config.get('/whoami')
  }
}
