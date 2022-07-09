import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import CategoryView from '../views/CategoryView.vue'
import TranslateView from '../views/TranslateView.vue'
import WordView from '../views/WordView.vue'

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'home',
        component: HomeView
    },
    {
        path: '/category/:language',
        name: 'category',
        component: CategoryView,
        props: true
    },
    {
        path: "/word/:language/:category",
        name: "word",
        component: WordView,
        props: true
        },
    {
        path: "/jisho",
        name: "jisho",
        component: TranslateView
    },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
