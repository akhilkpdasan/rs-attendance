import flushPromises from 'flush-promises'
import { mount } from '@vue/test-utils'
import UpdateStudent from '@/components/UpdateStudent'
jest.mock('@/api.js')

describe('UpdateStudent', () => {
  test('submit button calls delete with student id', async () => {
    const $router = { push: jest.fn() }
    const wrapper = mount(UpdateStudent, {
      mocks: {
        $router
      }
    })

    wrapper.vm.$data.sid = 's35'
    wrapper.vm.$data.attendance = '100.0'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect($router.push).toHaveBeenCalledWith('/students')
  })

  test('handles bad input error', async () => {
    const wrapper = mount(UpdateStudent)
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Incorrect student details')
  })

  test('handles unauthorized error', async () => {
    const $router = {push: jest.fn()}
    const wrapper = mount(UpdateStudent, {
      mocks: {
        $router
      }
    })

    wrapper.vm.$data.sid = 'unauthorized'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    // expect($router.push).toHaveBeenCalledWith('login')
    expect(wrapper.find('.error').text()).toBe('Please login')
  })

  test('handles internal error', async () => {
    const wrapper = mount(UpdateStudent)

    wrapper.vm.$data.sid = 'internal'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Internal server error occured')
  })
})
