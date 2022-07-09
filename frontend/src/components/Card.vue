<template>
    <div @click="this.$emit('click')">
        {{ text }}
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
    props: {
        text: String
    },
    emits: [
        "click"
    ]
})

export default class Card extends Vue {
    text!: string;
}
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";

    div {
        margin: auto auto;
        background-color: $secondary-color;
        color: $primary-color;
        height: 50px;
        padding: 10px 20px;
        font-size: 36px;
        line-height: 50px;
        align-content: left;
        width: 80vw;
        border-radius: 10px;
    }
</style>