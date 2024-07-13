import {createRouter, createWebHistory} from "vue-router"
import Notification from "../components/Notification.vue"
import Home from "../components/Home.vue"
import SaveDetail from "../components/SaveDetail.vue"

const router = createRouter({
    history: createWebHistory(),
    routes:[
        {
            path:'/',
            name:'Home',
            component: Home
        },
        {
            path:'/notification',
            name:'notification',
            component: Notification
        },
        {
            path:'/detail',
            name:'saveDetail',
            component: SaveDetail
        }
    ]
});

export default router