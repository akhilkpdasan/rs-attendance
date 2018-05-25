import Vue from 'vue'
import Router from 'vue-router'
import Login from '@/components/Login.vue'
import Register from '@/components/Register.vue'
import StudentsList from '@/components/StudentsList.vue'
import NewStudent from '@/components/NewStudent.vue'
import UpdateStudent from '@/components/UpdateStudent.vue'
import DeleteStudent from '@/components/DeleteStudent.vue'
import store from '@/store'

Vue.use(Router)

const router = new Router({
  linkActiveClass: 'active',
  routes: [
    {
      path: '/students',
      component: StudentsList,
      meta: { requiresAuth: true }
    },
    {
      path: '/new',
      component: NewStudent,
      meta: { requiresAuth: true }
    },
    {
      path: '/update',
      component: UpdateStudent,
      meta: { requiresAuth: true }
    },
    {
      path: '/delete',
      component: DeleteStudent,
      meta: { requiresAuth: true }
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

router.beforeEach((to, from, next) => {
  if (to.matched.some(record => record.meta.requiresAuth)) {
    if (store.state.username) {
      next()
    } else {
      next({path: '/login'})
    }
  } else {
    next()
  }
})

export default router
