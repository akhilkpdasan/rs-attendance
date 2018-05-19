import { mount, createLocalVue } from '@vue/test-utils'
import VueRouter from 'vue-router'
import Nav from '@/components/Nav'

describe('Nav', () => {
  test('shows logout link if user is logged in', () => {
    const localVue = createLocalVue()
    localVue.use(VueRouter)

    const router = new VueRouter()

    const wrapper = mount(Nav, {
      localVue,
      router,
      propsData: {
        username: 'akhil'
      }
    })
    const links = wrapper.vm.$el.querySelectorAll('a')
    console.log(links)
    expect(links[1].href).toBe('about:blank#/logout')
  })

  test('shows login and register link if user is  not logged in', () => {
    const localVue = createLocalVue()
    localVue.use(VueRouter)

    const router = new VueRouter()

    const wrapper = mount(Nav, {
      localVue,
      router
    })
    const links = wrapper.vm.$el.querySelectorAll('a')
    expect(links[1].href).toBe('about:blank#/login')
    expect(links[2].href).toBe('about:blank#/register')
  })
})
