import { mount } from '@vue/test-utils'
import StudentsList from '@/components/StudentsList'

describe('StudentsList', () => {
  test('displays students passed as props', () => {
    const wrapper = mount(StudentsList, {
      propsData: {
        students: [
          {id: 's35', name: 'akhil', roll_no: 35, attendance: 100.0}
        ]
      }
    })
    const id = wrapper.findAll('td').at(0)
    const name = wrapper.findAll('td').at(1)
    const rollNo = wrapper.findAll('td').at(2)
    const attendance = wrapper.findAll('td').at(3)
    expect(id.text()).toBe('s35')
    expect(name.text()).toBe('akhil')
    expect(rollNo.text()).toBe('35')
    expect(attendance.text()).toBe('100.0')
  })
})
