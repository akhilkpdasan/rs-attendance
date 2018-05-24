import flushPromises from 'flush-promises'
import { mount } from '@vue/test-utils'
import Register from '@/components/Register'
jest.mock('@/api.js')

describe('Register', () => {
  test('register works', async () => {
    const $router = {push: jest.fn()}
    const wrapper = mount(Register, {
      mocks: {
        $router
      }
    })

    wrapper.vm.$data.username = 'success'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect($router.push).toHaveBeenCalledWith('/login')
  })

  test('conflict error is handled', async () => {
    const wrapper = mount(Register)

    wrapper.vm.$data.username = 'conflict'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Username already taken')
  })

  test('internal server error is handled', async () => {
    const wrapper = mount(Register)

    wrapper.vm.$data.username = 'internal'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Interenal server error occured')
  })
})
