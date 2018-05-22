import flushPromises from 'flush-promises'
import { mount, createLocalVue } from '@vue/test-utils'
import VueRouter from 'vue-router'
import Vuex from 'vuex'
import Nav from '@/components/Nav'
jest.mock('@/api.js')

const localVue = createLocalVue()
localVue.use(Vuex)
localVue.use(VueRouter)

describe('Nav', () => {
  let store
  let router
  let actions

  beforeEach(() => {
    router = new VueRouter()
    actions = {
      logout: jest.fn()
    }
    store = new Vuex.Store({
      state: {
        username: 'frontend'
      },
      actions
    })
  })

  test('shows logout link if user is logged in', () => {
    const wrapper = mount(Nav, {
      localVue,
      store,
      router
    })
    const link = wrapper.findAll('a').at(1)
    expect(link.text()).toBe('Logout')
  })

  test('clicking logout call logout action in store', async () => {
    const wrapper = mount(Nav, {
      localVue,
      store,
      router
    })

    wrapper.findAll('a').at(1).trigger('click')
    await flushPromises()

    expect(actions.logout).toHaveBeenCalled()
  })

  test('shows login link if user is  not logged in', () => {
    store.state.username = ''
    const wrapper = mount(Nav, {
      localVue,
      store,
      router
    })
    const links = wrapper.vm.$el.querySelectorAll('a')
    expect(links[1].href).toBe('about:blank#/login')
  })
})
