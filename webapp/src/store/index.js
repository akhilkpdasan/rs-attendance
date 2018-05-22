import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    username: ''
  },

  mutations: {
    login (state, username) {
      state.username = username
    },
    logout (state) {
      state.username = ''
    }
  },

  actions: {
    login ({ commit }, username) {
      commit('login', username)
    },
    logout ({ commit }) {
      commit('logout')
    }
  }
})

export default store
