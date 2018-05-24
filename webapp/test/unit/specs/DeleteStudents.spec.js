import flushPromises from 'flush-promises'
import { mount } from '@vue/test-utils'
import DeleteStudent from '@/components/DeleteStudent'
jest.mock('@/api.js')

describe('DeleteStudent', () => {
  test('submit button calls delete with student id', async () => {
    const $router = { push: jest.fn() }
    const wrapper = mount(DeleteStudent, {
      mocks: {
        $router
      }
    })

    wrapper.vm.$data.sid = 's35'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect($router.push).toHaveBeenCalledWith('/students')
  })

  test('handles bad input error', async () => {
    const wrapper = mount(DeleteStudent)
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Incorrect student details')
  })

  test('handles unauthorized error', async () => {
    const $router = {push: jest.fn()}
    const wrapper = mount(DeleteStudent, {
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
    const wrapper = mount(DeleteStudent)

    wrapper.vm.$data.sid = 'internal'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Internal server error occured')
  })
})
