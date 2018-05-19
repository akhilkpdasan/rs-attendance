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
  routes: [
    {
      path: '/',
      name: 'StudentsList',
      component: StudentsList
    },
    {
      path: '/new',
      component: NewStudent
    },
    {
      path: '/update/:sid',
      component: UpdateStudent
    },
    {
      path: '/delete/:sid',
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
