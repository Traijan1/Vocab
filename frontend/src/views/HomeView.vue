<template>
    <div class="home">
        <Card :key="language" v-for="language in languages" :text="language" class="card" @click="navigateCategory(language)" />
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import Card from "../components/Card.vue";
import { useAppStore } from "@/stores/AppStore";
import { AppView } from "@/models/AppView";

@Options({
    components: {
        Card
    },
})
export default class HomeView extends Vue {
    appStore = useAppStore();

    languages: string[] = [
        "日本語",
        "German"
    ]

    created() {
        this.appStore.currentView = AppView.Vocab;
    }

    navigateCategory(language: string) {
        this.$router.push("/category/" + language);
    }
}
</script>

<style lang="scss" scoped>
    .card {
        margin-top: 20px;
    }
</style>
