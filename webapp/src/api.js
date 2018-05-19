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
      .then(response => {
        return response.data
      })
  },

  login (username, password) {
    return config.post('/login', {'username': username, 'password': password})
      .then(response => {
        return response.data
      })
  },

  logout () {
    return config.get('/logout')
      .then(response => {
        return response.data
      })
  }
}
