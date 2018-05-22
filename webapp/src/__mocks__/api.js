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
        error.response = {status: 401}
        reject(error)
      }))
    } else if (username === 'internal') {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.response = {status: 500}
        reject(error)
      }))
    } else {
      return (new Promise((resolve, reject) => {
        let error = new Error()
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

        let response = {}
        response.data = students
        response.status = 200
        resolve(response)
      }))
    } else if (getStudentsCalled === 1) {
      getStudentsCalled = 2
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.response = {status: 401}
        reject(error)
      }))
    } else if (getStudentsCalled === 2) {
      getStudentsCalled = 3
      return (new Promise((resolve, reject) => {
        let error = new Error()
        error.response = {status: 500}
        reject(error)
      }))
    } else {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        reject(error)
      }))
    }
  },

  logout () {
    return (new Promise(resolve => {
      resolve()
    }))
  }
}
