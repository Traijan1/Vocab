<template>
    <div class="header">Mother Tongue:</div>
    <input type="text" v-model="mother" @kedown.enter="addWord" />

    <div class="header">Foreign Language:</div>
    <input type="text" v-model="foreign" />
    
    <div class="header">Second Reading:</div>
    <input type="text" v-model="secondReading" @kedown.enter="addWord" />

    <span>
        <VocabButton @click="addWord">Add</VocabButton>
    </span>

    <span class="error">{{ errorText }}</span>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import VocabButton from '../VocabButton.vue';
import WordService from "../../services/WordService";
import { Word } from '@/models/Word';

@Options({
    components: {
        VocabButton
    },
    props: {
        foreignLanguage: String,    
        motherTongue: String
    }
})

export default class CreateWord extends Vue {
    foreignLanguage!: string;  
    motherTongue!: string;  

    foreign = "";
    mother = "";
    secondReading: string|undefined = undefined;

    errorText = "";

    created() {
        this.foreign = this.foreignLanguage;
        this.mother = this.motherTongue;
    }

    async addWord() {
        this.errorText = "";
        
        if(this.foreign == "" || this.mother == "")
            return;

        this.errorText = await WordService.postWord(new Word(this.foreign, this.mother, this.secondReading));
    
        this.mother = "";
        this.foreign = "";
        this.secondReading = "";

        this.$router.push("/create/word");
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

    .error {
        margin-top: 20px;
        text-align: center;
        color: red;
    }
</style>