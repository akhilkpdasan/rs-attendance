var getStudentsCalled = 0
export default {
  login (username, password) {
    if (username === 'correct') {
      return (new Promise(resolve => {
        resolve('Success')
      }))
    } else if (username === 'wrong') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Incorrect username or password'
        reject(error)
      }))
    } else if (username === 'internal') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Interenal server error occured'
        reject(error)
      }))
    }
  },

  register (username, password, email) {
    if (username === 'success') {
      return (new Promise(resolve => {
        resolve('Success')
      }))
    } else if (username === 'conflict') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Username already taken'
        reject(error)
      }))
    } else if (username === 'internal') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Interenal server error occured'
        reject(error)
      }))
    }
  },

  getStudents () {
    if (getStudentsCalled === 0) {
      getStudentsCalled = 1
      return (new Promise(resolve => {
        let students = [{
          id: 's35',
          name: 'akhil',
          roll_no: '35',
          attendance: '100.0'
        }]

        resolve(students)
      }))
    } else if (getStudentsCalled === 1) {
      getStudentsCalled = 2
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Please login'
        reject(error)
      }))
    } else if (getStudentsCalled === 2) {
      getStudentsCalled = 3
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Internal server error occured'
        reject(error)
      }))
    }
  },

  logout () {
    return (new Promise(resolve => {
      resolve()
    }))
  },

  newStudent (sid, name, rollNo, attendance) {
    if (name === '') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Incorrect student details'
        reject(error)
      }))
    } else if (name === 'unauthorized') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Please login'
        reject(error)
      }))
    } else if (name === 'internal') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Internal server error occured'
        reject(error)
      }))
    } else {
      return (new Promise(resolve => {
        resolve()
      }))
    }
  },

  deleteStudent (sid) {
    if (sid === '') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Incorrect student details'
        return reject(error)
      }))
    } else if (sid === 'unauthorized') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Please login'
        return reject(error)
      }))
    } else if (sid === 'internal') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Internal server error occured'
        return reject(error)
      }))
    } else {
      return (new Promise(resolve => {
        resolve()
      }))
    }
  },

  updateStudent (sid, attendance) {
    if (sid === '') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Incorrect student details'
        return reject(error)
      }))
    } else if (sid === 'unauthorized') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Please login'
        return reject(error)
      }))
    } else if (sid === 'internal') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.msg = 'Internal server error occured'
        return reject(error)
      }))
    } else {
      return (new Promise(resolve => {
        resolve()
      }))
    }
  }
}
