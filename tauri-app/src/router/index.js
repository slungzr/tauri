import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Login',
    component: () => import('../views/Login.vue')
  },
  {
    path: '/browser',
    name: 'Browser',
    component: () => import('../views/Browser.vue')
  },
  {
    path: '/parameter-check',
    name: 'ParameterCheck',
    component: () => import('../views/1049BatchParamCheck.vue')
  },
  {
    path: '/mqtt',
    name: 'MQTT',
    component: () => import('../views/mqtt.vue')
  },
  {
    path: '/gb',
    name: 'GBParamCheck',
    component: () => import('../views/GBParamCheck.vue')
  },
  {
    path: '/gb-batch',
    name: 'GBParamBatchCheck',
    component: () => import('../views/GBParamBatchCheck.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})


// 路由守卫
router.beforeEach((to, from, next) => {
  const isAuthenticated = localStorage.getItem('redis-connected')
  if (to.name !== 'Login' && !isAuthenticated) {
    next({ name: 'Login' })
  } else {
    next()
  }
})

export default router 