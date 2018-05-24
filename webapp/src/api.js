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
      .then(res => {
        return res.data
      })
      .catch(err => {
        let error = new Error()
        if (err.response) {
          if (err.response.status === 401) {
            error.msg = 'Please login'
          }
        } else {
          error.msg = 'Internal server error occured'
        }
        return error
      })
  },

  login (username, password) {
    return config.post('/login', {'username': username, 'password': password})
      .then(res => {
        return res
      })
      .catch(err => {
        let error = new Error()
        if (err.response) {
          if (err.response.status === 401) {
            error.msg = 'Incorrect username or password'
            return error
          }
        } else {
          error.msg = 'Interenal server error occured'
          return error
        }
      })
  },

  register (username, password, email) {
    return config.post('/register', {'username': username, 'password': password, 'email': email})
      .then(res => {
        return res
      })
      .catch(err => {
        let error = new Error()
        if (err.response) {
          if (err.response.status === 409) {
            error.msg = 'Username already taken'
            return error
          }
        } else {
          error.msg = 'Interenal server error occured'
          return error
        }
      })
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
      .then(res => {
        return res
      })
      .catch(err => {
        let error = new Error()
        if (err.response) {
          if (err.response.status === 401) {
            error.msg = 'Please login'
          } else if (err.response === 400) {
            error.msg = 'Incorrect Student details'
          }
        } else {
          error.msg = 'Interenal server error occured'
        }
        return error
      })
  },

  deleteStudent (sid) {
    return config.delete(`/students/${sid}`)
      .then(res => {
        return res
      })
      .catch(err => {
        let error = new Error()
        if (err.response) {
          if (err.response.status === 401) {
            error.msg = 'Please login'
          } else if (err.response.status === 400) {
            error.msg = 'Incorrect student details'
          }
        } else {
          error.msg = 'Internal server error occured'
        }
        return error
      })
  },

  updateStudent (sid, attendance) {
    attendance = parseFloat(attendance)
    return config.put(`/students/${sid}`, {'attendance': attendance})
      .then(res => {
        return res
      })
      .catch(err => {
        let error = new Error()
        if (err.response) {
          if (err.response.status === 401) {
            error.msg = 'Please login'
          } else if (err.response.status === 400) {
            error.msg = 'Incorrect student details'
          }
        } else {
          error.msg = 'Internal server error occured'
        }
        return error
      })
  },

  whoami () {
    return config.get('/whoami')
  }
}
