<template>
    <div >
        <router-link :to="link" class="link">+</router-link>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
    props: {
        link: String
    }
})

export default class FloatingActionButton extends Vue {
  
}
</script>

<style lang="scss" scoped>
    div {
        position: absolute;
        bottom: 70px;
        right: 20px;
        text-align: center;
        font-size: 30px;
        line-height: 42.5px;
        width: 40px;
        height: 40px;
        background-color: #F9CFF2;
        border-radius: 50%;
    }

    .link {
        text-decoration: none;
    }
</style>