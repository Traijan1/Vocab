<template>
    <div class="home">
        <Card :key="category" v-for="category in categories" :text="category" class="card" @click="chooseCategory(category)" />
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
    props: {
        language: String
    }
})
export default class HomeView extends Vue {
    appStore = useAppStore();

    language!: string;

    categories: string[] = [
        "All Words",
        "All Phrases",
        "Family"
    ]

    created() {
        this.appStore.currentView = AppView.Vocab;
    }

    chooseCategory(category: string) {
        this.$router.push(`/word/${this.language}/${category}`);
    }
}
</script>

<style lang="scss" scoped>
    .card {
        margin-top: 20px;
    }
</style>
