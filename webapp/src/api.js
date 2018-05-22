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
  }
}
