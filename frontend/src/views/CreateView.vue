<template>
    <div>
        <CreateWord v-if="type == 'word'" :foreignLanguage="foreignLanguage" :motherTongue="motherTongue" />
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import CreateWord from '../components/create/CreateWord.vue';
import { useAppStore } from "@/stores/AppStore";
import { AppView } from "@/models/AppView";

@Options({
    props: {
        type: String,
        foreignLanguage: String,    
        motherTongue: String
    },
    components: {
        CreateWord
    }
})

export default class CreateView extends Vue {
    appStore = useAppStore();

    foreignLanguage!: string;    
    motherTongue!: string;
    
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
