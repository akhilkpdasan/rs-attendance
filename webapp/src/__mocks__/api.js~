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
        error.response = new Error()
        error.response = {status: 500}
        reject(error)
      }))
    } else {
      return (new Promise((resolve, reject) => {
        let error = new Error()
        reject(error)
      }))
    }
  }
}
