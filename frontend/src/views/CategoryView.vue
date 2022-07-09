<template>
    <div class="home">
        <Card :key="category" v-for="category in categories" :text="category" class="card" @click="chooseCategory(category)" />
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import Card from "../components/Card.vue";

@Options({
    components: {
        Card
    },
    props: {
        language: String
    }
})
export default class HomeView extends Vue {
    language!: string;

    categories: string[] = [
        "All Words",
        "All Phrases",
        "Family"
    ]

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
