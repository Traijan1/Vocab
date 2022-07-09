<template>
    <div>Word</div>

    <input type="text" />
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
})

export default class CreateWord extends Vue {
  
}
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";

    input[type="text"] {
        @include vocab-input;
    }
</style>