import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    username: ''
  },
  mutations: {
    login (state, username) {
      this.state.username = username
    },
    logout (state) {
      this.state.username = ''
    }
  }
})

export default store
