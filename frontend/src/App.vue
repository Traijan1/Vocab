<template>
    <div class="searchbar" :style="{ display: appStore.currentView == 0 ? 'block' : 'none' }">
        <input type="text" placeholder="Search.."/>
    </div>

    <nav>
        <router-link to="/" class="link" :class="{ highlight: appStore.currentView < 2 }" @click="setView(0)">Vocab</router-link>
        <router-link to="/jisho" class="link" :class="{ highlight: appStore.currentView == 2 }" @click="setView(2)">Dictionary</router-link>
    </nav>
    
    <router-view/>

    <FloatingActionButton link="/create/word" @click="setView(1)" />
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import FloatingActionButton from "./components/FloatingActionButton.vue";
import { useAppStore } from './stores/AppStore';

@Options({
    components: {
        FloatingActionButton
    }
})

export default class App extends Vue { 
    appStore = useAppStore();

    setView(view: number) {
        this.appStore.currentView = view;
    }
}
</script>


<style lang="scss">
@import "@/assets/variables.scss";

$nav-height: 30px;

#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    color: #2c3e50;
}

nav {
    display: flex;
    justify-content: space-evenly;
    position: absolute;
    bottom: 0px;
    background-color: $gray;
    width: 100vw;
    height: $nav-height;
    color: white;
    line-height: 30px;
    padding: 10px 0px;

    .link {
        color: white;
        text-decoration: none;
    }

    .highlight {
        color: $primary-color;
    }
}

.searchbar {
    margin-top: 20px;
    text-align: center;
    
    input {
        @include vocab-input;
        padding: 10px;
        width: 85vw;
    }
}

</style>
