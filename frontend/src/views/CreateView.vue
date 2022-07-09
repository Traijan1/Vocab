<template>
    <div>
        <CreateWord v-if="type == 'word'" />
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import CreateWord from '../components/create/CreateWord.vue';

@Options({
    props: {
        type: String
    },
    components: {
        CreateWord
    }
})

export default class CreateView extends Vue {
    type!: string;
}
</script>

<style lang="scss" scoped>

</style>
