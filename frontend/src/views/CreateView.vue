<template>
    <div>
        <CreateWord v-if="type == 'word'" />
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import CreateWord from '../components/create/CreateWord.vue';
import { useAppStore } from "@/stores/AppStore";
import { AppView } from "@/models/AppView";

@Options({
    props: {
        type: String
    },
    components: {
        CreateWord
    }
})

export default class CreateView extends Vue {
    appStore = useAppStore();
    
    type!: string;

    created() {
        this.appStore.currentView = AppView.WordView;
    }
}
</script>

<style lang="scss" scoped>
    div {
        margin: 50px 30px;
    }
</style>
