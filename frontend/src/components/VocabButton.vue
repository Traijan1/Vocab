<template>
    <button @click="click()"><slot/></button>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
    emits: [
        "click"
    ]
})

export default class VocabButton extends Vue { 
    click() {
        this.$emit("click");
    }
}
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";

button {
    padding: 10px 20px;
    border-radius: 10px;
    height: 45px;
    background-color: $secondary-color;
    border: none;
    color: white;
    font-size: 20px;
}
</style>
