import flushPromises from 'flush-promises'
import { mount, createLocalVue } from '@vue/test-utils'
import Vuex from 'vuex'
import Login from '@/components/Login'
jest.mock('@/api.js')

const localVue = createLocalVue()
localVue.use(Vuex)

describe('Login', () => {
  let store
  let actions

  beforeEach(() => {
    actions = {
      login: jest.fn()
    }
    store = new Vuex.Store({
      state: {},
      actions
    })
  })

  test('submit button calls login actions on store', async () => {
    const wrapper = mount(Login, {
      localVue,
      store
    })

    wrapper.vm.$data.username = 'correct'
    wrapper.find('#submit').trigger('click')

    await flushPromises()

    expect(actions.login).toHaveBeenCalled()
  })

  test('error is shown if incorrect pass or username', async () => {
    const wrapper = mount(Login, {
      localVue,
      store
    })

    wrapper.vm.$data.username = 'wrong'

    wrapper.find('#submit').trigger('click')

    await flushPromises()

    expect(actions.login.mock.calls.length).toBe(0)
    expect(wrapper.find('p').text()).toBe('Incorrect username or password')
  })

  test('IntrenalServerError is shown', async () => {
    const wrapper = mount(Login, {
      localVue,
      store
    })

    wrapper.vm.$data.username = 'internal'
    wrapper.find('#submit').trigger('click')

    await flushPromises()

    expect(actions.login.mock.calls.length).toBe(0)
    expect(wrapper.find('p').text()).toBe('Interenal server error occured')
  })
})
