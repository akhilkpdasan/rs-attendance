import { mount } from '@vue/test-utils'
import StudentsList from '@/components/StudentsList'
import flushPromises from 'flush-promises'
jest.mock('@/api.js')

describe('StudentsList', () => {
  test('displays students', async () => {
    const wrapper = mount(StudentsList, {
    })

    await flushPromises()
    const id = wrapper.findAll('td').at(0)
    const name = wrapper.findAll('td').at(1)
    const rollNo = wrapper.findAll('td').at(2)
    const attendance = wrapper.findAll('td').at(3)
    expect(id.text()).toBe('s35')
    expect(name.text()).toBe('akhil')
    expect(rollNo.text()).toBe('35')
    expect(attendance.text()).toBe('100.0')
  })

  test('handles unauthorization error', async () => {
    const $router = {}
    $router.push = jest.fn()

    let wrapper = mount(StudentsList, {
      mocks: {
        $router
      }
    })
    await flushPromises()

    // expect($router.push).toHaveBeenCalledWith('login')
    const error = wrapper.find('.error')
    expect(error.text()).toBe('Please login')
  })

  test('handles internal server error', async () => {
    const wrapper = mount(StudentsList, {
    })
    await flushPromises()

    const error = wrapper.find('.error')
    expect(error.text()).toBe('Internal server error occured')
  })
})
