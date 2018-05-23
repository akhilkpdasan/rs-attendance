import flushPromises from 'flush-promises'
import { mount } from '@vue/test-utils'
import NewStudent from '@/components/NewStudent'
jest.mock('@/api.js')

describe('NewStudent', () => {
  test('submit button calls new with student details', async () => {
    const $router = { push: jest.fn() }
    const wrapper = mount(NewStudent, {
      mocks: {
        $router
      }
    })

    wrapper.vm.$data.id = 's35'
    wrapper.vm.$data.name = 'akhil'
    wrapper.vm.$data.roll_no = '35'
    wrapper.vm.$data.attendance = '100.0'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect($router.push).toHaveBeenCalledWith('/')
  })

  test('handles bad input error', async () => {
    const wrapper = mount(NewStudent)
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('Incorrect student details')
  })

  test('handles unauthorized error', async () => {
    const $router = {push: jest.fn()}
    const wrapper = mount(NewStudent, {
      mocks: {
        $router
      }
    })

    wrapper.vm.$data.name = 'unauthorized'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect($router.push).toHaveBeenCalledWith('login')
  })

  test('handles internal error', async () => {
    const wrapper = mount(NewStudent)

    wrapper.vm.$data.name = 'internal'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('An internal server error has occured')
  })

  test('handles unknown error', async () => {
    const wrapper = mount(NewStudent)

    wrapper.vm.$data.name = 'unknown'
    wrapper.find('#submit').trigger('click')

    await flushPromises()
    expect(wrapper.find('.error').text()).toBe('An unknown error has occured')
  })
})
