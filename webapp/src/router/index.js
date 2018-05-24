import Vue from 'vue'
import Router from 'vue-router'
import Login from '@/components/Login.vue'
import Register from '@/components/Register.vue'
import StudentsList from '@/components/StudentsList.vue'
import NewStudent from '@/components/NewStudent.vue'
import UpdateStudent from '@/components/UpdateStudent.vue'
import DeleteStudent from '@/components/DeleteStudent.vue'

Vue.use(Router)

export default new Router({
  linkActiveClass: 'active',
  routes: [
    {
      path: '/students',
      component: StudentsList
    },
    {
      path: '/new',
      component: NewStudent
    },
    {
      path: '/update',
      component: UpdateStudent
    },
    {
      path: '/delete',
      component: DeleteStudent
    },
    {
      path: '/login',
      component: Login
    },
    {
      path: '/register',
      component: Register
    }
  ]
})
