<template>
    <div class="header">Foreign Language:</div>
    <input type="text" v-model="foreignLanguage" />
    
    <div class="header">Mother Tongue:</div>
    <input type="text" v-model="motherTongue" @kedown.enter="addWord" />

    <span>
        <VocabButton @click="addWord">Add</VocabButton>
    </span>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import VocabButton from '../VocabButton.vue';
import WordService from "../../services/WordService";
import { Word } from '@/models/Word';

@Options({
    components: {
        VocabButton
    }
})

export default class CreateWord extends Vue {
    foreignLanguage = "";  
    motherTongue = "";  

    async addWord() {
        if(this.foreignLanguage == "" || this.motherTongue == "")
            return;

        await WordService.postWord(new Word(this.foreignLanguage, this.motherTongue));
    
        this.motherTongue = "";
        this.foreignLanguage = "";
    }
}
</script>

<style lang="scss" scoped>
    @import "@/assets/variables.scss";

    input[type="text"] {
        @include vocab-input;
        margin-bottom: 40px;
    }

    .header {
        display: block;
        font-size: 26px;
        color: $primary-color;
        margin-bottom: 20px;
    }

    span {
        display: block;
        text-align: center;
    }

    .button { 
        margin: auto auto;
    }
</style>